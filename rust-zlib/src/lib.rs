#![feature(unsafe_destructor)]
#![allow(unstable)]

extern crate libc;

use std::mem;
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use libc::funcs::c95::stdlib::{malloc, realloc, free};
use std::ptr::null;
use cvec::{CVec, Buf};
use header::GZHeader;

#[macro_use]
mod macros;
mod cvec;
mod gz;
mod header;

/////////////////////////////////////////////////////////////////////
//                    Constants & Macros                           //
/////////////////////////////////////////////////////////////////////

const MAXBITS:   usize = 15;  // maximum bits in a code
const MAXLCODES: usize = 286; // maximum number of literal/length codes
const MAXDCODES: usize = 30;  // maximum number of distance codes
const FIXLCODES: usize = 288; // number of fixed literal/length codes
// maximum number of code lengths to read
const MAXCODES:  usize = MAXLCODES + MAXDCODES;

/////////////////////////////////////////////////////////////////////
//                   Decompression functions                       //
/////////////////////////////////////////////////////////////////////




/////////////////////////////////////////////////////////////////////
//                   Decompression interface                       //
/////////////////////////////////////////////////////////////////////

/// The main decompression function
/// Assumption: The Vec given to this function is a gzipped buffer

#[no_mangle]
pub extern "C" fn decompress_zlib_to_heap(buf: *const c_void,
                                          buf_len: c_int,
                                          decompressed_len: *mut c_int)
        -> *mut c_void {
    let in_vec = try_bail!(unsafe { CVec::from_raw_buf(buf as *const c_uchar, buf_len as usize)});
    println!("{:?}", in_vec);
    let out_vec = try_bail!(gz::decompress_gz(in_vec));
    unsafe {
        let (out_ptr, out_size) = out_vec.into_raw_buf();
        *decompressed_len = out_size as c_int;
        out_ptr as *mut c_void
    }
}

