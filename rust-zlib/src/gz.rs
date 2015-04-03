use cvec::{CVec, Buf};
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use std::ptr;

fn get_uncompressed_len(buffer: Buf) -> usize {
    unsafe {
        let byte_ptr = buffer.get_raw_pointer_to_item(buffer.len() - 4);
        ptr::read(byte_ptr as *const c_uint) as usize
    }
}



pub fn decompress(buffer: Buf) -> Option<Buf> {
    let out_len = get_uncompressed_len(buffer);
    println!("Out_len: {}", out_len);
    let out_buf = try_opt!(CVec::with_capacity(out_len));
    Some(out_buf)
}
