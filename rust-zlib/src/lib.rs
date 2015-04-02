#![feature(unsafe_destructor)]
#![allow(unstable)]

extern crate libc;

use std::mem;
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use libc::funcs::c95::stdlib::{malloc, realloc, free};
use std::ptr::null;
use cvec::{CVec, Buf};

mod cvec;

/////////////////////////////////////////////////////////////////////
//                    Constants & Macros                           //
/////////////////////////////////////////////////////////////////////

const MAXBITS:   usize = 15;  // maximum bits in a code
const MAXLCODES: usize = 286; // maximum number of literal/length codes
const MAXDCODES: usize = 30;  // maximum number of distance codes
const FIXLCODES: usize = 288; // number of fixed literal/length codes
// maximum number of code lengths to read
const MAXCODES:  usize = MAXLCODES + MAXDCODES;


macro_rules! bail {
    () => {
        return null::<c_void>() as *mut c_void;
    }
}

macro_rules! try_bail {
    ($expr: expr) => (match $expr {
        Option::Some(v) => v,
        Option::None => { bail!() },
    })
}

/////////////////////////////////////////////////////////////////////
//                   Decompression functions                       //
/////////////////////////////////////////////////////////////////////




/////////////////////////////////////////////////////////////////////
//                   Decompression interface                       //
/////////////////////////////////////////////////////////////////////

/// The main decompression function
/// Assumption: The Vec given to this function is a gzipped buffer
fn decompress(buffer: Buf) -> Option<Buf> {
    CVec::new()
}

#[no_mangle]
pub extern "C" fn decompress_zlib_to_heap(buf: *const c_void,
                                          buf_len: c_int,
                                          decompressed_len: *mut c_int)
        -> *mut c_void {
    let in_vec = try_bail!(unsafe { CVec::from_raw_buf(buf as *const c_uchar, buf_len as usize)});
    let out_vec = try_bail!(decompress(in_vec));
    unsafe {
        let (out_ptr, out_size) = out_vec.to_raw_buf();
        *decompressed_len = out_size as c_int;
        out_ptr as *mut c_void
    }
}

