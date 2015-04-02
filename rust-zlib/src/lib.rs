#[allow(unstable)]

extern crate libc;

use std::mem;
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use libc::funcs::c95::stdlib::{malloc, realloc, free};
use std::ptr::null;

/////////////////////////////////////////////////////////////////////
//                    Constants & Macros                           //
/////////////////////////////////////////////////////////////////////

const MAXBITS:   usize = 15;  // maximum bits in a code
const MAXLCODES: usize = 286; // maximum number of literal/length codes
const MAXDCODES: usize = 30;  // maximum number of distance codes
const FIXLCODES: usize = 288; // number of fixed literal/length codes
// maximum number of code lengths to read
const MAXCODES:  usize = MAXLCODES + MAXDCODES;

type Buf = Vec<u8>;

macro_rules! bail {
    () => {
        return null::<c_void>() as *mut c_void;
    }
}

/////////////////////////////////////////////////////////////////////
//                   Decompression functions                       //
/////////////////////////////////////////////////////////////////////




/////////////////////////////////////////////////////////////////////
//                   Decompression interface                       //
/////////////////////////////////////////////////////////////////////

/// The main decompression function
/// Assumption: The Vec given to this function is a gzipped buffer
fn decompress(buffer: Vec<u8>) -> Vec<u8> {
    buffer
}

#[no_mangle]
pub extern "C" fn decompress_zlib_to_heap(buf: *const c_void,
                                          buf_len: c_int,
                                          decompressed_len: *mut c_int)
        -> *mut c_void {
    return decompress(buf as *const c_uchar, buf_len as c_uint, decompressed_len);
}

