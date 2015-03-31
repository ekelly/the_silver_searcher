#![feature(lang_items)]
#[allow(unstable)]

extern crate libc;

use libc::{c_void, size_t, c_int, c_char};

const TINFL_FLAG_PARSE_ZLIB_HEADER: c_int = 0x1; // parse zlib header and adler32 checksum

#[link(name = "miniz", kind = "static")]
#[allow(dead_code)]
extern {
    /// Raw miniz compression function.
    fn tdefl_compress_mem_to_heap(psrc_buf: *const c_void,
                                  src_buf_len: size_t,
                                  pout_len: *mut size_t,
                                  flags: c_int)
                                  -> *mut c_void;

    /// Raw miniz decompression function.
    fn tinfl_decompress_mem_to_heap(psrc_buf: *const c_void,
                                    src_buf_len: size_t,
                                    pout_len: *mut size_t,
                                    flags: c_int)
                                    -> *mut c_void;
}



#[no_mangle]
pub extern "C" fn decompress_zlib_to_heap(buf: *const c_void,
                                          buf_len: c_int,
                                          _: *const c_char,
                                          new_buf_len: *mut c_int)
        -> *mut c_void {
    print!("hello rust");
    let ptr;
    let mut output_len: size_t = 0;
    let input_len: size_t = buf_len as size_t;
    unsafe {
        ptr = tinfl_decompress_mem_to_heap(buf,
                                           input_len,
                                           &mut output_len,
                                           TINFL_FLAG_PARSE_ZLIB_HEADER);
        *new_buf_len = output_len as c_int;
    }
    return ptr;
}



#[test]
fn it_works() {
}
