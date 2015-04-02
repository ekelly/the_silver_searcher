#[allow(unstable)]

extern crate libc;
extern crate flate;

use std::mem;
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use libc::funcs::c95::stdlib::{malloc, realloc, free};
use std::ptr::null;
use std::ffi::CString;

// need to fudge this to inflateInit2_
const ZLIB_VERSION: &'static str = "1.2.5";


const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;
const Z_NEED_DICT: c_int = 2;
const Z_ERRNO: c_int = -1;
const Z_STREAM_ERROR: c_int = -2;
const Z_DATA_ERROR: c_int = -3;
const Z_MEM_ERROR: c_int = -4;
const Z_BUF_ERROR: c_int = -5;

const Z_NO_FLUSH: c_int = 0;

const GZ_WINDOW_BITS: c_int = 47; // 15 + 32

const BAD_RESULTS: [c_int; 4] = [Z_STREAM_ERROR, Z_NEED_DICT, Z_DATA_ERROR, Z_MEM_ERROR];

#[repr(C)]
pub struct z_stream {
    pub next_in: *const c_uchar,
    pub avail_in: c_uint,
    pub total_in: c_ulong,

    pub next_out: *mut c_uchar,
    pub avail_out: c_uint,
    pub total_out: c_ulong,

    pub msg: *const c_char,
    pub state: *mut z_internal_state,

    pub zalloc: Option<z_alloc_func>,
    pub zfree: Option<z_free_func>,
    pub opaque: *mut c_void,

    pub data_type: c_int,
    pub adler: c_ulong,
    pub reserved: c_ulong,
}

pub enum z_internal_state {}

pub type z_alloc_func = extern fn(*mut c_void,
                                  size_t,
                                  size_t) -> *mut c_void;
pub type z_free_func = extern fn(*mut c_void, *mut c_void);

#[link(name = "z", kind = "dylib")]
extern {
    pub fn zlibVersion() -> *const c_char;
    pub fn inflateInit2_(stream: *mut z_stream,
                         window_bits: c_int,
                         version: *const c_char,
                         stream_size: c_int) -> c_int;
    pub fn inflate(stream: *mut z_stream, flush: c_int) -> c_int;
    pub fn inflateEnd(stream: *mut z_stream) -> c_int;
}


// creates a new, zeroed z stream for inflate
pub fn init_z_stream() -> z_stream {
    unsafe {
        mem::zeroed::<z_stream>()
    }
}

macro_rules! bail {
    () => {
        return null::<c_void>() as *mut c_void;
    }
}


pub fn decompress(buf: *const c_uchar,
                  buf_len: c_uint,
                  decompressed_len: *mut c_int)
        -> *mut c_void
{
    let my_version = CString::from_slice(ZLIB_VERSION.as_bytes()).as_ptr();
    let mut ret: c_int = Z_OK;
    let mut result: *mut c_uchar = null::<c_uchar>() as *mut c_uchar;
    let mut tmp_result: *mut c_uchar;
    let mut result_size: size_t = buf_len as size_t;
    let mut stream = init_z_stream();

    unsafe {
        // init inflate, with bits for gzip format detection
        let tmp = inflateInit2_(&mut stream,
                                GZ_WINDOW_BITS,
                                my_version,
                                mem::size_of::<z_stream>() as c_int);
        if tmp != Z_OK {
            bail!();
        }
        stream.avail_in = buf_len;
        stream.next_in = buf;

        while ret == Z_OK || stream.avail_out == 0 {
            tmp_result = result;

            // double the buffer size and realloc, since it didn't all fit last time
            result_size *= 2;
            result = realloc(result as *mut c_void,
                             result_size * mem::size_of::<c_uchar>() as size_t) as *mut c_uchar;
            if result.is_null() {
                free(tmp_result as *mut c_void);
                inflateEnd(&mut stream);
                bail!();
            }

            stream.avail_out = (result_size / 2) as c_uint;
            stream.next_out =  result.offset(stream.total_out as isize);
            ret = inflate(&mut stream, Z_NO_FLUSH);
            if BAD_RESULTS.contains(&ret) {
                free(result as *mut c_void);
                inflateEnd(&mut stream);
                bail!();
            }
        }

        *decompressed_len = stream.total_out as c_int;
        inflateEnd(&mut stream);

        if (ret == Z_STREAM_END) {
            return result as *mut c_void;
        }
        bail!();
    }
}

#[no_mangle]
pub extern "C" fn decompress_zlib_to_heap(buf: *const c_void,
                                          buf_len: c_int,
                                          decompressed_len: *mut c_int)
        -> *mut c_void
{
    println!("Address received: {:p}", decompressed_len);
    // Output: Address received: 0x7fcfe3404e30
    return decompress(buf as *const c_uchar, buf_len as c_uint, decompressed_len);
}

