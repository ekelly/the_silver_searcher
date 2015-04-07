#![allow(unstable)]
#![allow(dead_code)]
#![feature(unsafe_destructor)]
#![feature(box_syntax)]

extern crate libc;

use libc::{c_int, c_uchar, c_void};
use std::ptr::null;
use cvec::CVec;

#[macro_use]
mod macros;
mod cvec;
mod gz;
mod header;
mod crc32;
mod huffman;
mod gz_reader;

/////////////////////////////////////////////////////////////////////
//                    Constants & Macros                           //
/////////////////////////////////////////////////////////////////////
/*
const MAXBITS:   usize = 15;  // maximum bits in a code
const MAXLCODES: usize = 286; // maximum number of literal/length codes
const MAXDCODES: usize = 30;  // maximum number of distance codes
const FIXLCODES: usize = 288; // number of fixed literal/length codes
// maximum number of code lengths to read
const MAXCODES:  usize = MAXLCODES + MAXDCODES;
*/
/////////////////////////////////////////////////////////////////////
//                   Decompression functions                       //
/////////////////////////////////////////////////////////////////////




/////////////////////////////////////////////////////////////////////
//                   Decompression interface                       //
/////////////////////////////////////////////////////////////////////

/// The main decompression function
/// Assumption: The Vec given to this function is a gzipped buffer

#[no_mangle]
pub extern "C" fn decompress_gzip_to_heap(buf: *const c_void,
                                          buf_len: c_int,
                                          decompressed_len: *mut c_int)
        -> *mut c_void {
    let in_vec = try_bail!(unsafe { CVec::from_raw_buf(buf as *const c_uchar, buf_len as usize)});
    let out_vec = try_bail!(gz::decompress_gz(in_vec));
    unsafe {
        let (out_ptr, out_size) = out_vec.into_raw_buf();
        *decompressed_len = out_size as c_int;
        out_ptr as *mut c_void
    }
}

