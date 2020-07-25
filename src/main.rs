#![allow(dead_code)]

use std::result::Result as StdResult;

use bytesize::ByteSize;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{LPCVOID, LPVOID, BOOL, FALSE, DWORD};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{VirtualQueryEx, ReadProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::EnumProcesses;
use winapi::um::winnt::{HANDLE, PROCESS_VM_READ, PROCESS_QUERY_INFORMATION, MEMORY_BASIC_INFORMATION};
use winapi::um::winnt;

mod error;
use error::{Error, Result};

mod stats;
use stats::{IoStats};

/// Represents a memory address.
type Address = u64;

/// Cross-platform representation of a collection of adjacent memory pages that
/// have similar properties.
///
/// Some good resources explaining permissions are as follows:
///   - windows
///     - 
#[derive(Debug, Clone, Copy)]
struct PageRange {
    /// Base address of the region.
    base: Address,

    // TODO: fill out docs
    pub read: bool,
    pub write: bool,    // NOTE: not supported on some OSes.
    pub execute: bool,  // NOTE: DEP may disallow this flag.

    /// Indicates if this region has been allocated system resources, rather
    /// than just reserving the numeric virtual address range for future use.
    ///
    /// Refer to the following for more info:
    ///   - https://www.tenouk.com/WinVirtualAddressSpace.html#page-state
    ///   - https://docs.microsoft.com/en-us/windows/win32/memory/memory-protection-constants
    ///   - https://j00ru.vexillium.org/2013/04/fun-facts-windows-kernel-and-guard-pages/
    ///   - https://docs.microsoft.com/en-us/windows/win32/memory/freeing-virtual-memory
    ///   - https://www.tenouk.com/WinVirtualAddressSpace.html#page-state
    committed: bool,

    /// Indicates if this region has a guard flag set.
    guarded: bool,

    /// Indicates if this region has been freed in a way that renders the range
    /// inaccessible to users.
    freed: bool,

    /// Indicates if this region is shared across multiple virtual address spaces.
    shared: bool,

    /// Indicates if this region is cachable.
    cachable: bool,

    /// Indicates if this region is copy-on-write.
    cow: bool,

    /// Size in bytes of this region.
    size: usize
}

impl std::fmt::Display for PageRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let committed = if self.committed { "" } else { "not-committed" };
        let guarded = if self.guarded { " guarded" } else { "" };
        let freed = if self.freed { " freed" } else { "" };
        let shared = if self.shared { " shared" } else { "" };
        let cachable = if self.cachable { "" } else { "not-cachable" };
        let cow = if self.cow { " copy-on-write" } else { "" };
        let r = if self.read { "r" } else { "-" };
        let w = if self.write { "w" } else if self.cow { "c" } else { "-" };
        let x = if self.execute { "x" } else { "-" };

        write!(f, "0x{:016x} {}{}{} {: >9}{}{}{}{}{}{}", self.base, r, w, x,
                ByteSize(self.size as u64).to_string_as(true), committed,
                guarded, freed, shared, cachable, cow)
    }
}

// TODO: split IO operations from Process into an AddressSpace struct that we populate from a process

#[derive(Debug)]
struct Process {
    id: u64,
    handle: HANDLE,
    read_stats: IoStats
}

impl Drop for Process {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                CloseHandle(self.handle);
            };
        }
    }
}

impl Process {
    pub fn open_by_pid(process_id: u64) -> Result<Process> {
        let handle: HANDLE = unsafe {
            OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION,
                    false as BOOL, process_id as DWORD)
        };

        if handle.is_null() {
            return Err(Error::new("Failed to open handle"));
        }

        Ok(Process {
            id: process_id,
            handle: handle,
            read_stats: IoStats::new()
        })
    }

    /// Get the region that the provided address is located in.
    pub fn get_region(&self, address: Address) -> Option<PageRange> {
        /* Get info on this region, if there is one. */
        let mut meminfo: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        let ret: SIZE_T = unsafe {
            VirtualQueryEx(self.handle, address as LPCVOID, &mut meminfo,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>() as SIZE_T)
        };

        if ret == 0 {
            return None;
        }

        /* Construct region. */
        let mut region = PageRange {
            base: meminfo.BaseAddress as Address,
            read: false,  // These two will be set later
            write: false,
            execute: (meminfo.Protect & 0xf0) != 0,
            committed: (meminfo.State & winnt::MEM_COMMIT) != 0,
            guarded: (meminfo.Protect & winnt::PAGE_GUARD) != 0,
            freed: (meminfo.State & winnt::MEM_FREE) != 0,
            shared: (meminfo.Type & winnt::MEM_PRIVATE) == 0,
            cachable: (meminfo.Protect & winnt::PAGE_NOCACHE) == 0,
            cow: (meminfo.Protect
                    & (winnt::PAGE_EXECUTE_WRITECOPY
                        | winnt::PAGE_WRITECOPY)) != 0,
            size: meminfo.RegionSize
        };

        // Write is a superset of read, so we can save lines by checking it
        // first. We also add in a bonus check to try and detect weird
        // permissions. Note that rwx permissions are stored in the lowest
        // byte, which is why we mask it off.
        region.write = match meminfo.Protect & 0xff {
            winnt::PAGE_EXECUTE =>           false,
            winnt::PAGE_EXECUTE_READ =>      false,
            winnt::PAGE_EXECUTE_READWRITE => true,
            winnt::PAGE_EXECUTE_WRITECOPY => true,
            winnt::PAGE_NOACCESS =>  false,
            winnt::PAGE_READONLY =>  false,
            winnt::PAGE_READWRITE => true,
            winnt::PAGE_WRITECOPY => true,
            0x00 =>  false,  // Appears to be set when page isn't valid.
            _ => panic!("Encountered unknown page permissions: {}",
                    meminfo.Protect & 0xff)
        };

        // Having write permissions implies read permissions, so we only need
        // to check read permissions here.
        region.read = match meminfo.Protect & 0xff {
            winnt::PAGE_EXECUTE_READ => true,
            winnt::PAGE_READONLY =>     true,
            _ => false
        } || region.write;

        Some(region)
    }

    pub fn get_regions(&self) -> Vec<PageRange> {
        let mut regions: Vec<PageRange> = Vec::new();

        /* Enumerate all regions. */
        let mut base: Address = 0;
        loop {
            let region = self.get_region(base);
            match region {
                Some(r) => r,
                None => break,
            };
            let region = region.unwrap();

            regions.push(region);

            base += region.size as Address;
        }

        regions
    }

    pub fn read<T>(&mut self, address: u64, amount: usize) -> Result<Vec<T>>  {
        /* Read the chunk */
        let mut amount_read: SIZE_T = 0;
        let mut out: Vec<T> = Vec::with_capacity(amount);

        let mut stats = self.read_stats.begin(amount);

        let success = unsafe {
            ReadProcessMemory(self.handle, address as LPCVOID,
                    out.as_mut_ptr() as LPVOID, out.capacity() as SIZE_T,
                    &mut amount_read)
        };

        // TODO: reduce vector size based on amount actually read?

        if success == FALSE {
            return Err(Error::new("RPM failed."));
        }

        self.read_stats.end(&mut stats);

        Ok(out)
    }
}

/// Dump a process
fn dump_proc_by_pid(pid: u64) -> Result<IoStats> {
    let mut process = Process::open_by_pid(pid)?;







    // TODO: locate adjacent regions and read across them, get_read_regions()? 
    // TODO: graph region read speed







    for region in process.get_regions() {
        if !region.committed { continue; }
        let _: Vec<u8> = match process.read(region.base, region.size) {
            Ok(data) => data,
            Err(_) => continue
        };
    }

    Ok(process.read_stats.clone())
}

fn main() -> StdResult<(), Box<dyn std::error::Error>> {
    /* Get a snapshot of all valid process IDs */
    let mut processes: [DWORD; 1024] = [0; 1024];
    let mut cbneeded: DWORD = 0;
    let _ = unsafe {
        EnumProcesses(processes.as_mut_ptr(),
                (processes.len() * std::mem::size_of::<DWORD>()) as u32, &mut cbneeded)
    };

    /* Dump each process, recording read statistics on each dump. */
    let mut dump_stats: Vec<IoStats> = Vec::new();

    let process_count = cbneeded as usize / std::mem::size_of::<DWORD>();
    for (i, pid) in processes.iter().enumerate() {
        if i == process_count { break; }  // exit early if we reach the end of the list

        let read_stats = match dump_proc_by_pid(*pid as u64) {
            Ok(read_stats) => read_stats,
            Err(_) => continue
        };

        println!("{}/{} - {}", i, process_count, read_stats);

        dump_stats.push(read_stats.clone());
    }

    let aggregate_read_stats = dump_stats.iter().fold(IoStats::new(),
            | agg, cur | agg + cur.clone());  // NOTE: clone here is probably a slow idea
    
    println!("total_reads={}, avg_read_speed={}", aggregate_read_stats.io_metrics.len(), aggregate_read_stats);

    //for metric in aggregate_read_stats.io_metrics {
    //    println!("{}", metric);
    //}

    Ok(())
}






    // TODO: we need the ability to tag keys (e.g. module="foo_exe") to sections of memory, preferably with a notion of heirarchy (e.g. "foo_exe.text").
    //       object graph can tap into this?