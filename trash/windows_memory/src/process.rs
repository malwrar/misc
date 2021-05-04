#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod platform;
#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

// TODO: implement process as a wrapper for each major OS that implements AddressSpace traits
// TODO: after this, implement local memory AddressSpace reader, use it to write AddressSpace tests
// TODO: after that, implement objects that are linked to an address space (and thus can refresh themselves) and contain a value
// TODO: after that, implement value scanning for objects over an addressspace, range, etc
// TODO: after that, implement the graph system. around now is when we should revisit our design doc and refine this thing in the final library