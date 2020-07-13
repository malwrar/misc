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
//use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::{HANDLE, PROCESS_VM_READ, PROCESS_QUERY_INFORMATION, MEMORY_BASIC_INFORMATION, MEM_FREE, MEM_COMMIT, PAGE_GUARD, MEM_PRIVATE};

mod error;
use error::{Error, Result};

/// Represents a memory address.
type Address = u64;

#[derive(Debug, Clone, Copy)]
struct Permission {
    pub read: bool,
    pub write: bool,   // NOTE: not supported on some OSes.
    pub execute: bool  // NOTE: DEP may disallow this flag.
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = if self.read { "r" } else { "-" };
        let w = if self.write { "w" } else { "-" };
        let x = if self.execute { "x" } else { "-" };

        write!(f, "{}{}{}", r, w, x)
    }
}

/// Cross-platform representation of a collection of adjacent memory pages that
/// have similar properties.
#[derive(Debug, Clone, Copy)]
struct Region {
    /// Base address of the region.
    base: Address,

    /// [Permission](struct.Protection.html)s in effect for this region.
    permissions: Permission,

    /// Indicates if this region has been allocated system resources, rather
    /// than just reserving the numeric virtual address range for future use.
    ///
    /// Refer to the following for more info:
    ///   - https://www.tenouk.com/WinVirtualAddressSpace.html#page-state
    committed: bool,

    /// Indicates if this region has a guard flag set.
    ///
    /// Refer to the following for more info:
    ///   - https://docs.microsoft.com/en-us/windows/win32/memory/memory-protection-constants
    ///   - https://j00ru.vexillium.org/2013/04/fun-facts-windows-kernel-and-guard-pages/
    guarded: bool,

    /// Indicates if this region has been freed in a way that renders the range
    /// inaccessible to users.
    ///
    /// Refer to the following for more info:
    ///   - https://docs.microsoft.com/en-us/windows/win32/memory/freeing-virtual-memory
    ///   - https://www.tenouk.com/WinVirtualAddressSpace.html#page-state
    freed: bool,

    /// Indicates if this region is shared across multiple virtual address spaces.
    shared: bool,

    /// Indicates if this region is cachable.
    cachable: bool,

    /// Size in bytes of this region.
    size: usize
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let committed = if self.committed { " committed" } else { "" };
        let guarded = if self.guarded { " guarded" } else { "" };
        let freed = if self.freed { " freed" } else { "" };
        let shared = if self.shared { " shared" } else { "" };
        let cachable = if self.cachable { " cachable" } else { "" };

        write!(f, "0x{:016x} {} {: >9}{}{}{}{}{}", self.base, self.permissions,
                ByteSize(self.size as u64).to_string(), committed, guarded,
                freed, shared, cachable)
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
    pub fn get_region(&self, address: Address) -> Option<Region> {
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
        let region = Region {
            base: meminfo.BaseAddress as Address,
            permissions: Permission {
                read: false,
                write: false,
                execute: false
            },
            committed: (meminfo.State & MEM_COMMIT) != 0,
            guarded: (meminfo.Protect & PAGE_GUARDED) != 0,
            freed: (meminfo.State & MEM_FREE) != 0,
            shared: (meminfo.Type & MEM_PRIVATE) != 0,
            cachable: false,
            size: meminfo.RegionSize
        };

        Some(region)
    }

    pub fn get_regions(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = Vec::new();

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

    //pub fn read<T>(&self, address: u64, amount: usize, out: &mut [T]) -> Result<usize, ExpError>  {
    //    /* Read the chunk */
    //    let mut amount_read = 0;
    //    let success = unsafe {
    //        ReadProcessMemory(self.handle, address as LPCVOID,
    //                out.as_mut_ptr() as LPVOID, min(out.len(), amount) as SIZE_T,
    //                &mut amount_read)
    //    };

    //    if success == FALSE {
    //        return Err(ExpError::RPMFailed);
    //    }

    //    return Ok(amount_read);
    //}
}

fn main() {
    let pid = 12944;
    
    /* Open process */
    let process = Process::open_by_pid(pid);
    let process = match process {
        Ok(obj) => obj,
        Err(error) => panic!("Failed to open pid={}: {}", pid, error),
    };
    println!("Opened handle on pid={}: {:?}.", pid, process.handle);

    /* Dump each region */
    for region in process.get_regions() {
        if !region.committed {
            println!("{} ", region);
        }
    }

    // /* Loop through each region in this process's memory. */
    // let mut region_base: usize = 0;
    // let mut regions: Vec<Region> = Vec::new();

    // let start = Instant::now();
    // loop {

    //     /* Move on to the next region */
    //     region_base += meminfo.RegionSize;
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