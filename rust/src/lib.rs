#[no_mangle]
extern "C" fn parse_banner(len: libc::size_t, data: *const u8) -> *const banner::Banner {}
