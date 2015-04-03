use cvec::{CVec, Buf};
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use std::ptr;


// every gzip file is at least 10 bytes, if not, it's invalid
const GZIP_MIN_LEN: usize = 40;

const GZIP_FILESIZE_OFFSET: usize = 4;
const GZIP_CRC_OFFSET: usize = 8;

fn get_uncompressed_len(buffer: &Buf) -> usize {
    assert!(buffer.len() > GZIP_MIN_LEN);
    unsafe {
        let byte_ptr = buffer.get_raw_pointer_to_item(buffer.len() - GZIP_FILESIZE_OFFSET);
        ptr::read(byte_ptr as *const c_uint) as usize
    }
}

fn get_crc(buffer: &Buf) -> usize {
    assert!(buffer.len() > GZIP_MIN_LEN);
    unsafe {
        let byte_ptr = buffer.get_raw_pointer_to_item(buffer.len() - GZIP_CRC_OFFSET);
        ptr::read(byte_ptr as *const c_uint) as usize
    }
}




pub fn decompress(buffer: Buf) -> Option<Buf> {
    if buffer.len() < GZIP_MIN_LEN {
        return None;
    }
    let out_len = get_uncompressed_len(&buffer);
    let crc = get_crc(&buffer);
    println!("out_len: {}", out_len);
    println!("crc: {}", crc);
    let out_buf = try_opt!(CVec::with_capacity(out_len));
    Some(out_buf)
}
