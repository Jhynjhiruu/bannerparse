#![feature(allocator_api)]
#![feature(pointer_is_aligned)]

mod banner;
mod imd5;
mod rshaxe;
mod u8;

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

    Box::into_raw(Box::new_in(banner, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_banner(banner: *mut banner::Banner) {
    println!("dropping banner at {banner:?}");
    unsafe { std::ptr::drop_in_place(banner) };
}

#[no_mangle]
extern "C" fn get_banner(banner: *mut banner::Banner) -> *const u8 {
    let file = &unsafe { &*banner }.data;

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
extern "C" fn get_titles(banner: *mut banner::Banner) -> *const u8 {
    let rv = unsafe { rshaxe::new_array_string() };
    for lang in &unsafe { &*banner }.header.names {
        // uncomment this to escape (some) Unicode characters in titles, e.g.
        // ゼルダの伝説　ﾄﾜｲﾗｲﾄﾌﾟﾘﾝｾｽ -> ゼルダの伝説\u{3000}ﾄﾜｲﾗｲﾄﾌﾟﾘﾝｾｽ
        // let lang = lang.escape_debug().to_string();
        unsafe {
            rshaxe::array_string_push(
                rv,
                rshaxe::construct_string(
                    match lang.len().try_into() {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!("{e}");
                            return std::ptr::null();
                        }
                    },
                    lang.as_ptr(),
                ),
            )
        }
    }
    rv
}

#[no_mangle]
extern "C" fn parse_u8(len: libc::size_t, data: *const u8) -> *mut u8::U8Archive {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let arc = match u8::U8Archive::parse(&mut cursor) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };

    println!("listing: {:?}", arc.ls("").unwrap());

    Box::into_raw(Box::new_in(arc, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_u8(arc: *mut u8::U8Archive) {
    println!("dropping arc at {arc:?}");
    unsafe { std::ptr::drop_in_place(arc) };
}

#[no_mangle]
extern "C" fn list_dir(arc: *const u8::U8Archive, len: libc::size_t, data: *const u8) -> *const u8 {
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };
    let path = match String::from_utf8(data_slice.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };
    let dir = match unsafe { &*arc }.ls(path) {
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
    arc: *const u8::U8Archive,
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
    let file = match unsafe { &*arc }.get(path) {
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

#[no_mangle]
extern "C" fn get_imd5(imd5: *mut imd5::IMD5) -> *const u8 {
    let file = &unsafe { &*imd5 }.data;

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
extern "C" fn decompress_lz77(len: libc::size_t, data: *const u8) -> *const u8 {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let dec_data = match match ninty77::LZ77::parse(data) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    }
    .decompress()
    {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null();
        }
    };

    unsafe {
        rshaxe::construct_array_u8(
            match dec_data.len().try_into() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return std::ptr::null();
                }
            },
            dec_data.as_ptr(),
        )
    }
}
