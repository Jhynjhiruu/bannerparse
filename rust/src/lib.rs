#![feature(allocator_api)]
#![feature(pointer_is_aligned)]

mod banner;
mod imd5;
mod rshaxe;

#[no_mangle]
extern "C" fn parse_banner(len: libc::size_t, data: *const u8) -> *mut banner::Banner {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let banner = match banner::Banner::parse(&mut cursor) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };

    println!("{banner:x?}");
    println!("listing: {:?}", banner.content.ls("/meta").unwrap());

    Box::into_raw(Box::new_in(banner, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_banner(banner: *mut banner::Banner) {
    println!("dropping banner at {banner:?}");
    unsafe { std::ptr::drop_in_place(banner) };
}

#[no_mangle]
extern "C" fn list_dir(
    banner: *const banner::Banner,
    len: libc::size_t,
    data: *const u8,
) -> *const u8 {
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };
    let path = match String::from_utf8(data_slice.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };
    let dir = match unsafe { &*banner }.content.ls(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };

    let rv = unsafe { rshaxe::new_array_string() };

    for file in dir {
        unsafe {
            rshaxe::array_string_push(
                rv,
                rshaxe::construct_string(
                    match file.len().try_into() {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!("{e}");
                            return std::ptr::null();
                        }
                    },
                    file.as_ptr(),
                ),
            )
        }
    }

    rv
}

#[no_mangle]
extern "C" fn get_file(
    banner: *const banner::Banner,
    len: libc::size_t,
    data: *const libc::c_uchar,
) -> *const u8 {
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };
    let path = match String::from_utf8(data_slice.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };
    let file = match unsafe { &*banner }.content.get(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };

    unsafe {
        rshaxe::construct_array_u8(
            match file.len().try_into() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return std::ptr::null();
                }
            },
            file.as_ptr(),
        )
    }
}

#[no_mangle]
extern "C" fn parse_imd5(len: libc::size_t, data: *const u8) -> *mut imd5::IMD5 {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let imd5 = match imd5::IMD5::parse(&mut cursor) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };

    println!("{imd5:x?}");

    Box::into_raw(Box::new_in(imd5, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_imd5(imd5: *mut imd5::IMD5) {
    println!("dropping imd5 at {imd5:?}");
    unsafe { std::ptr::drop_in_place(imd5) };
}
