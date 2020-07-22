#![allow(dead_code)]

use std::fmt;
use std::cmp::min;
use time::{Instant, Duration};
use bytesize::ByteSize;

use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{LPCVOID, LPVOID, BOOL, FALSE, DWORD};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{VirtualQueryEx, ReadProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::EnumProcesses;
//use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::{HANDLE, PROCESS_VM_READ, PROCESS_QUERY_INFORMATION, MEMORY_BASIC_INFORMATION, MEM_FREE, MEM_COMMIT, PAGE_GUARD, MEM_PRIVATE};
use winapi::um::winnt;

mod error;
use error::{Error, Result};

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

impl fmt::Display for PageRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug)]
struct Process {
    id: u64,
    handle: HANDLE
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

        Ok(Process { id: process_id, handle: handle })
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
            committed: (meminfo.State & MEM_COMMIT) != 0,
            guarded: (meminfo.Protect & winnt::PAGE_GUARD) != 0,
            freed: (meminfo.State & MEM_FREE) != 0,
            shared: (meminfo.Type & MEM_PRIVATE) == 0,
            cachable: (meminfo.Protect & winnt::PAGE_NOCACHE) == 0,
            cow: (meminfo.Protect & (winnt::PAGE_EXECUTE_WRITECOPY | winnt::PAGE_WRITECOPY)) != 0,
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

    pub fn read<T>(&self, address: u64, amount: usize) -> Result<Vec<T>>  {
        /* Read the chunk */
        let mut amount_read = 0;
        let mut out: Vec<T> = Vec::with_capacity(amount);
        let success = unsafe {
            ReadProcessMemory(self.handle, address as LPCVOID,
                    out.as_mut_ptr() as LPVOID, min(out.len(), amount) as SIZE_T,
                    &mut amount_read)
        };

        if success == FALSE {
            return Err(Error::new("RPM failed."));
        }

        Ok(out)
    }
}

struct ProcDumpStats {
    process_open_time: Duration,
    region_count: usize,
    region_avg_size: usize,
    region_enum_time: Duration,
    region_dump_time: Duration,
}

impl ProcDumpStats {
    fn new() -> ProcDumpStats {
        ProcDumpStats {
            process_open_time: Duration::new(0, 0),
            region_count: 0,
            region_avg_size: 0,
            region_enum_time: Duration::new(0, 0),
            region_dump_time: Duration::new(0, 0)
        }
    }
}





// TODO: create functions to calc bytes per second, dump time, dump read call overhead










    
impl fmt::Display for ProcDumpStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes_per_microsecond = self.region_avg_size as i128 / self.region_dump_time.whole_microseconds();
        let bytes_per_second = (bytes_per_microsecond * Duration::second().whole_microseconds()) as u64;


        // TODO: is this right? low region


        write!(f, "regions={}\tavg_region_size={}\tread_speed={}/s",
                self.region_count, ByteSize(self.region_avg_size as u64),
                ByteSize(bytes_per_second))
    }
}

fn dump_proc_by_pid(pid: u64) -> Result<ProcDumpStats> {
    let mut stats = ProcDumpStats::new();

    /* Open process */
    let start = Instant::now();
    let process = Process::open_by_pid(pid)?;
    stats.process_open_time = start.elapsed();

    /* Dump each region */
    let start = Instant::now();
    let regions = process.get_regions();
    stats.region_enum_time = start.elapsed();

    let start = Instant::now();
    let mut region_size: usize = 0;
    for region in regions {
        if !region.committed { continue; }
        stats.region_count += 1;
        region_size += region.size;
        let _: Vec<u8> = process.read(region.base, region.size)?;  // dummy read to simulate work
    }
    stats.region_dump_time = start.elapsed();
    stats.region_avg_size = region_size / stats.region_count;

    Ok(stats)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut processes: [DWORD; 1024] = [0; 1024];
    let mut cbneeded: DWORD = 0;
    let _ = unsafe {
        EnumProcesses(processes.as_mut_ptr(),
                (processes.len() * std::mem::size_of::<DWORD>()) as u32, &mut cbneeded)
    };

    let process_count = cbneeded as usize / std::mem::size_of::<DWORD>();
    let dump_stats: Vec<ProcDumpStats> = Vec::new();

    for (i, pid) in processes.iter().enumerate() {
        if i == process_count { break; }  // exit early if we reach the end of the list

        let stats = match dump_proc_by_pid(*pid as u64) {
            Ok(stats) => stats,
            Err(_) => continue
        };
        dump_stats.push(stats);
    }

    let mut data_csv_file = File::open("stats.csv")
    for stats in dump_stats {
        write!(&mut data_csv_file, "{},{},{},{},{}", stats.process_open_time,
                stats.region_count, stats.region_avg_size,
                stats.region_enum_time, stats.region_dump_time);
    }

    // TODO: implement region system so we can mark e.g. processes. get rid of the get_regions functions, perhaps get_page_range only?











    // TODO: gather data across all processes, not just the first one
    // TODO: read all readable sections, getting an average bytes per second. run this a few times to get an idea if repeated reads caches better
    // TODO: log region size by average bytes per second time, see if read buffer size has an effect on speed
    // TODO: we need to determine bytes per second and read call penalty. Based on this info, we can determine if it's more efficient to read 
    //       across regions or individually
    // TODO: based on this info, build our batched read system (attempt to read multiple values in one read call, minimizing reads)














    // /* Loop through each region in this process's memory. */
    // let mut region_base: usize = 0;
    // let mut regions: Vec<PageRange> = Vec::new();

    // let start = Instant::now();
    // loop {

    //     /* Move on to the next region */
    //     region_base += meminfo.PageRangeSize;
    // }
    // println!("Discovered {} regions in {:?}", regions.len(),
    //         start.elapsed().to_std().ok().unwrap());

    // let mut total_time = Duration::seconds(0);
    // let mut total_bytes = 0;
    // for region in regions {
    //     let mut region_data: Vec<u8> = vec![0; region.size];

    //     let start = Instant::now();
    //     read(proc_handle, region.base_address, region.size, region_data.as_mut_slice());
    //     let read_time = start.elapsed();

    //     total_time += read_time;
    //     total_bytes += region.size;

    //     println!("REGION [addr={:016x}, size={:x}, read_time={:?}]",
    //             region.base_address, region.size, total_time.to_std().ok().unwrap());

    // }
    // println!("Dumped {} in {:?} (~{}/s)",
    //         ByteSize(total_bytes as u64).to_string(),
    //         total_time.to_std().unwrap(),
    //         ByteSize(((total_bytes as i128 / total_time.whole_microseconds())
    //                 * Duration::second().whole_microseconds()) as u64).to_string());
}