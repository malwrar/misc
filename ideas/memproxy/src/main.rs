/// Idea for a rust ABI over access to a remote address space. Provides:
///
/// 1.) Page table info.
/// 2.) Batched read/writes
/// 3.) Scanning capabilities (pattern, value, templated, etc)
/// 4.) Syncronization.
///
/// Access to remote address space is managed by a proxy, which is intended to
/// allow the ABI to be used to read local address space, remote address spaces
/// via debugging APIs (Read/WriteProcessMemory), exploits, rootkits, and even
/// access to files. Users can implement their own freely--so long as there's a
/// trait implementation everything else can be built on the AddressSpace
/// class. Making the Proxy and providing handles to AddressSpace is important
/// because the Proxy might have some shared state.

/// Manages direct, immediate access to an address space.
#![derive(Clone)]
struct AddressSpace {

}

type Address = usize;

struct Pointer<T> {
    address_space: AddressSpace,
    address: Address,
    cached_value: Option<T>,  // TODO: instead, this should point to cache space we have in the address_space
}

impl Pointer {
    fn to(address_space: mut AddressSpaceHandle, address: Address) {
        // TODO: create cache space
    }
}

impl<T> Deref for Pointer {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO: read from cached value
    }
}

impl<T> DerefMut for Pointer {
    type Target = T;

    fn deref_mut(&self) -> &Self::Target {
        // TODO: write to cached value
        &mut cached_value
    }
}

fn main() {
    println!("Hello, world!");

    // Open the remote address space.
    let process = Process::by_name("target.exe")?;
    let address_space = AddressSpace::open(process)?;

    // Define some targets to mess with.
    let target1: Pointer<i32> = Pointer::to(address_space, 0xdeadbeef)?;
    let target2: Pointer<i32> = Pointer::to(address_space, 0xcafebabe)?;
    let target3: Pointer<i32> = target2 + 0xff;
    let target4: Pointer<i32> = scan::pattern(address_space, "FF ? FF ?? FF")?;
    let target5: Pointer<i32> = scan::value(address_space, 1337)?;

    loop {
        // Nothing has been read yet, this will attempt one that will panic if not successful.
        let a = *target1;

        // Copy all data in the remote address space pointed to by the above pointers to our local cache
        address_space.read()?;

        // Locally modify the value pointed at by target4.
        let old_value = *target4;
        assert!(*target4 == old_value);

        *target4 = old_value + 1;
        assert!(*target4 == old_value + 1 );

        // Reread target4 specifically and demonstrate that a.) the cached value
        // was not written to remote memory and b.) the cached value updated
        // after the read.
        if target4.is_valid() {
            address_space.read_pointers(&[ target4 ])?;
        }
        // TODO: invent some method of syncronization?

        assert!(*target4 == old_value);

        // Make some changes and write them...
        let new_value = *target4 + 1;
        *target4 = new_value;

        address_space.write()?;

        // ...then read them back and confirm they were persisted.
        address_space.read()?;

        assert!(*target4 == new_value);
    }
}
