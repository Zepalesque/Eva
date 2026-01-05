
// util class for stuff idk, serialization moment
#[repr(C)]
pub struct VmSlice {
    size: usize,
    addr: *mut u8,
}