use std::cmp::min;
use time::{Instant, Duration};
use bytesize::ByteSize;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{LPCVOID, LPVOID, BOOL, FALSE, DWORD};
use winapi::um::memoryapi::{VirtualQueryEx, ReadProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
//use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::{HANDLE, PROCESS_VM_READ, PROCESS_QUERY_INFORMATION, MEMORY_BASIC_INFORMATION, MEM_FREE, MEM_RESERVE};

struct Region {
    base_address: u64,
    size: usize
}

#[derive(Debug)]
pub enum ExpError {
    RPMFailed,
}

//struct ReadStats {
//    //region: *Region,
//    chunk_size: usize,
//    time_taken: usize
//}

/// Wrapper over method to read slices of memory.
fn read<T>(proc_handle: HANDLE, address: u64, amount: usize, out: &mut [T]) -> Result<usize, ExpError>  {
    /* Read the chunk */
    let mut amount_read = 0;
    let success = unsafe {
        ReadProcessMemory(proc_handle, address as LPCVOID,
                out.as_mut_ptr() as LPVOID, min(out.len(), amount) as SIZE_T,
                &mut amount_read)
    };

    if success == FALSE {
        return Err(ExpError::RPMFailed);
    }

    return Ok(amount_read);
}

fn main() {
    let pid = 12944;
    
    let proc_handle: HANDLE = unsafe {
        OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION,
                false as BOOL, pid as DWORD)
    };
    if proc_handle.is_null() {
        println!("Failed to open handle on pid={}", pid);
        return;
    }
    println!("Opened handle on pid={}: {:?}", pid, proc_handle);


    /* Loop through each region in this process's memory. */
    let mut region_base: usize = 0;
    let mut regions: Vec<Region> = Vec::new();

    let start = Instant::now();
    loop {
        /* Get info on this region, if there is one. */
        let mut meminfo: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        let ret: SIZE_T = unsafe {
            VirtualQueryEx(proc_handle, region_base as LPCVOID, &mut meminfo,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>() as SIZE_T)
        };
        if ret == 0 { break; }

        /* Only record regions that we can interact with. */
        if meminfo.Type != 0 && meminfo.State != MEM_FREE && meminfo.State != MEM_RESERVE {
            regions.push(Region {
                base_address: region_base as u64,
                size: meminfo.RegionSize
            });
        }

        /* Move on to the next region */
        region_base += meminfo.RegionSize;
    }
    println!("Discovered {} regions in {:?}", regions.len(),
            start.elapsed().to_std().ok().unwrap());

    let mut total_time = Duration::seconds(0);
    let mut total_bytes = 0;
    for region in regions {
        let mut region_data: Vec<u8> = vec![0; region.size];

        let start = Instant::now();
        read(proc_handle, region.base_address, region.size, region_data.as_mut_slice());
        let read_time = start.elapsed();

        total_time += read_time;
        total_bytes += region.size;

        println!("REGION [addr={:016x}, size={:x}, read_time={:?}]",
                region.base_address, region.size, total_time.to_std().ok().unwrap());

    }
    println!("Dumped {} in {:?} (~{}/s)",
            ByteSize(total_bytes as u64).to_string(),
            total_time.to_std().unwrap(),
            ByteSize(((total_bytes as i128 / total_time.whole_microseconds())
                    * Duration::second().whole_microseconds()) as u64).to_string());
}