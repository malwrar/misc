struct AddressSpaceData;

/// Platform-specific data for doing stuff w/ the address space.
pub type LinkData = u32;

pub type AddressSpace = (LinkData, AddressSpaceData);