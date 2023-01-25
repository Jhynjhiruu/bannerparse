#![feature(allocator_api)]
#![feature(pointer_is_aligned)]

mod banner;
mod rshaxe;

#[no_mangle]
extern "C" fn parse_banner(len: libc::size_t, data: *const u8) -> *const banner::Banner {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let banner = match banner::Banner::parse(&mut cursor) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };

    println!("{:x?}", banner);
    println!("{:?}", banner.content.build_tree());

    Box::into_raw(Box::new_in(banner, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_banner(banner: *mut banner::Banner) {
    println!("dropping banner at {:?}", banner);
    unsafe { std::ptr::drop_in_place(banner) };
}
