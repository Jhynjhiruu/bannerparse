extern "C" {
    fn alloc(size: libc::c_uint, align: libc::size_t) -> *mut u8;
    pub fn construct_array_u8(size: libc::c_uint, data: *const u8) -> *mut u8;
    pub fn new_array_string() -> *mut u8;
    pub fn array_string_push(arr: *mut u8, str: *const u8);
    pub fn construct_string(size: libc::c_uint, data: *const u8) -> *mut u8;
}

pub const HAXE_ALLOCATOR: HXAlloc = HXAlloc::new();

pub struct HXAlloc {}

impl HXAlloc {
    const fn new() -> Self {
        Self {}
    }
}

unsafe impl std::alloc::Allocator for HXAlloc {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let ptr = unsafe {
            alloc(
                match layout.size().try_into() {
                    Ok(s) => s,
                    Err(_) => Err(std::alloc::AllocError)?,
                },
                layout.align(),
            )
        };

        if ptr.is_null() || !ptr.is_aligned_to(layout.align()) {
            println!(
                "ptr = {:?}, ptr align = {}",
                ptr,
                ptr.is_aligned_to(layout.align())
            );
            return Err(std::alloc::AllocError);
        }

        let slc = unsafe { std::slice::from_raw_parts_mut(ptr, layout.size()) };
        std::ptr::NonNull::new(slc).ok_or(std::alloc::AllocError)
    }

    unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {}
}
