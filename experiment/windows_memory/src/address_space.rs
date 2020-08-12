
pub trait AddressSpacePageRange {
}

pub trait AddressSpaceRegions {
    get_all_regions(); // Enum all
    get_regions(); // Lookup by key,value tuples, addresses, etc, get list of candidates
}

impl AddressSpaceRegions for AddressSpace {
}

pub trait AddressSpaceLink {
    fn connect();
    fn connected() -> bool;
}

pub trait AddressSpace<T> {
    /// Reads value from location pointed to by address.
    fn read(&self) -> Result<T>;

    /// Writes value into location pointed to by address.
    fn write(&self, value: &T) -> Result<T>;

    /// Determines destination address of a pointer located at address.
    fn deref(&self, address: Address) -> Result<Address>;

    /// Fetches information about the contiguous range of pages that the
    /// provided address is located in.
    fn get_page_ranges(&self, address: Address) -> Result<Option<PageRange>>;

    /// Enumerates all contiguous ranges of pages in the address space.
    fn get_all_page_ranges(&self) -> Result<Vec<PageRange>>;
}

// TODO: create region system. Regions are a range of memory that is associated
// with a key/value pair, and can either be static (will never move), cachable
// (rarely move but when they do can be detected), and dynamic (liable to
// change frequently in an undetectable way) and require updates on fetch.
//
// AddressSpace parsing implementations (such as one that reads usermode
// processes from another usermode process) can define special keys like
// "module" or platform-specific keys like "heap_space" to be added by default
// when we open an address space so the user doesn't need to explicitly call
// system dependent functions to determine such info.
//
// Perhaps we can make AddressSpaceSysInfoGatherer into a trait too, so that we
// can implement it with usermode debug functions, a stealthy kernel driver, a
// hypervisor, and more. This would allow us to capture system-specific region
// searches into a generic set of traits that could be implemented in multiple
// ways depending on the user's requirements. If I'm debugging a normal
// process, I might choose to use the stock usermode implementation. If I'm
// debugging a process that tries to keep debuggers out, I might use a stealthy
// info. If I'm running from the kernel or another system, I'll need to
// implement the region searches far differently.
//
// This system can then be extended to work for objects. We'll probably want to
// locate interesting objects immediately too like the PEB, threads, and
// module PE file info. From Objects and ObjectRefs we can form a graph, and
// if we implement fault detection and the "static", "cachable", and "dynamic"
// system for classifying permanence we can create a robust graph interface
// over address space metadata and contents. We could even cache their values
// as well as their locations, to make reads easier.
//
// The caching layer would use stats gathered by read/write ops + batch
// operation data to try and intelligently store values locally if they're
// fetched more than once.
//
// Perhaps in the future we can add a plugin system to do disassembly on the
// code sections of processes, both statically for files on disk (simulate
// loading and read that) or dynamically (operate on snapshots of process as it
// executes live). Combined with powerful address space info that we can point
// metadata at, it aught to add much more intelligence into reversing.