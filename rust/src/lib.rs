#![feature(allocator_api)]
#![feature(pointer_is_aligned)]

mod banner;
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

    println!("{:x?}", banner);
    println!("listing: {:?}", banner.content.ls("/meta").unwrap());

    Box::into_raw(Box::new_in(banner, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_banner(banner: *mut banner::Banner) {
    println!("dropping banner at {:?}", banner);
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

    let rv = {
        let mut v = file.len().to_le_bytes().to_vec_in(rshaxe::HAXE_ALLOCATOR);
        v.extend(file);
        v
    };

    unsafe {
        rshaxe::construct_array_u8(
            match rv.len().try_into() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return std::ptr::null();
                }
            },
            data,
        )
    }
}
