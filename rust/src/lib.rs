#![feature(allocator_api)]
#![feature(pointer_is_aligned)]

mod banner;
mod imd5;
mod rshaxe;
mod tpl;
mod u8;

macro_rules! unwrap_null {
    ($s: expr) => {
        match $s {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                return std::ptr::null_mut();
            }
        }
    };
}

#[no_mangle]
extern "C" fn parse_banner(len: libc::size_t, data: *const u8) -> *mut banner::Banner {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let banner = unwrap_null!(banner::Banner::parse(&mut cursor));

    Box::into_raw(Box::new_in(banner, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_banner(banner: *mut banner::Banner) {
    unsafe { std::ptr::drop_in_place(banner) };
}

#[no_mangle]
extern "C" fn get_banner(banner: *mut banner::Banner) -> *const u8 {
    let file = &unsafe { &*banner }.get_data();

    unsafe { rshaxe::construct_array_u8(unwrap_null!(file.len().try_into()), file.as_ptr()) }
}

#[no_mangle]
extern "C" fn get_titles(banner: *mut banner::Banner) -> *const u8 {
    let rv = unsafe { rshaxe::new_array_string() };
    for lang in unsafe { &*banner }.get_names() {
        // uncomment this to escape (some) Unicode characters in titles, e.g.
        // ゼルダの伝説　ﾄﾜｲﾗｲﾄﾌﾟﾘﾝｾｽ -> ゼルダの伝説\u{3000}ﾄﾜｲﾗｲﾄﾌﾟﾘﾝｾｽ
        // let lang = lang.escape_debug().to_string();
        unsafe {
            rshaxe::array_string_push(
                rv,
                rshaxe::construct_string(unwrap_null!(lang.len().try_into()), lang.as_ptr()),
            )
        }
    }
    rv
}

#[no_mangle]
extern "C" fn parse_u8(len: libc::size_t, data: *const u8) -> *mut u8::U8Archive {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let arc = unwrap_null!(u8::U8Archive::parse(&mut cursor));

    Box::into_raw(Box::new_in(arc, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_u8(arc: *mut u8::U8Archive) {
    unsafe { std::ptr::drop_in_place(arc) };
}

#[no_mangle]
extern "C" fn list_dir(arc: *const u8::U8Archive, len: libc::size_t, data: *const u8) -> *const u8 {
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };
    let path = unwrap_null!(String::from_utf8(data_slice.to_vec()));
    let dir = unwrap_null!(unsafe { &*arc }.ls(path));

    let rv = unsafe { rshaxe::new_array_string() };

    for file in dir {
        unsafe {
            rshaxe::array_string_push(
                rv,
                rshaxe::construct_string(unwrap_null!(file.len().try_into()), file.as_ptr()),
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
    let path = unwrap_null!(String::from_utf8(data_slice.to_vec()));
    let file = unwrap_null!(unsafe { &*arc }.get(path));

    unsafe { rshaxe::construct_array_u8(unwrap_null!(file.len().try_into()), file.as_ptr()) }
}

#[no_mangle]
extern "C" fn parse_imd5(len: libc::size_t, data: *const u8) -> *mut imd5::IMD5 {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let imd5 = unwrap_null!(imd5::IMD5::parse(&mut cursor));

    Box::into_raw(Box::new_in(imd5, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_imd5(imd5: *mut imd5::IMD5) {
    unsafe { std::ptr::drop_in_place(imd5) };
}

#[no_mangle]
extern "C" fn get_imd5(imd5: *mut imd5::IMD5) -> *const u8 {
    let file = unsafe { &*imd5 }.get_data();

    unsafe { rshaxe::construct_array_u8(unwrap_null!(file.len().try_into()), file.as_ptr()) }
}

#[no_mangle]
extern "C" fn decompress_lz77(len: libc::size_t, data: *const u8) -> *const u8 {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let dec_data = unwrap_null!(unwrap_null!(ninty77::LZ77::parse(data)).decompress());

    unsafe {
        rshaxe::construct_array_u8(unwrap_null!(dec_data.len().try_into()), dec_data.as_ptr())
    }
}

#[no_mangle]
extern "C" fn parse_tpl(len: libc::size_t, data: *const u8) -> *mut tpl::Tpl {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = std::io::Cursor::new(data);

    let tpl = unwrap_null!(tpl::Tpl::parse(&mut cursor));

    Box::into_raw(Box::new_in(tpl, rshaxe::HAXE_ALLOCATOR))
}

#[no_mangle]
extern "C" fn drop_tpl(tpl: *mut tpl::Tpl) {
    unsafe { std::ptr::drop_in_place(tpl) };
}

#[no_mangle]
extern "C" fn get_tpl_num_imgs(tpl: *mut tpl::Tpl) -> u32 {
    unsafe { &*tpl }.get_num_imgs()
}

#[no_mangle]
extern "C" fn get_tpl_size(tpl: *mut tpl::Tpl, idx: u32) -> u32 {
    let (width, height) = unsafe { &*tpl }
        .get_image_dims(idx as usize)
        .unwrap_or((0, 0));
    ((width as u32) << 0x10) | (height as u32)
}
