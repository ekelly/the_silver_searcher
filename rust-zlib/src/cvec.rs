extern crate libc;
extern crate core;

use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use libc::funcs::c95::stdlib::{malloc, realloc, free};
use std::vec::Vec;
use std::mem;
use self::core::raw::Slice as RawSlice;
use self::core::num::Int;
use std::ptr;
use std::fmt;

const DEFAULT_CVEC_CAPACITY: usize = 8;

pub type Buf = CVec<u8>;

macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        Option::Some(v) => v,
        Option::None => {
            return Option::None;
        }
    })
}

// this is an analogue of a Vec, but uses C-style allocation and reallocation
// so we can safely construct it from a C pointer, or return it as a C pointer.
// This cannot safely be used on zero-sized types, and will panic if you try.

pub struct CVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    mutable: bool,
}

#[feature(int_uint)]
impl<T> CVec<T> {
    fn check_type_size() {
        if mem::size_of::<T>() == 0 {
            panic!("tried to use a CVec with a zero-size type");
        }
    }

    pub fn new() -> Option<CVec<T>> {
        CVec::<T>::with_capacity(DEFAULT_CVEC_CAPACITY)
    }

    // Constructs a new CVec with given capacity
    // returns None if the allocation fails
    pub fn with_capacity(capacity: usize) -> Option<CVec<T>> {
        let capacity = if capacity > 0 { capacity } else { DEFAULT_CVEC_CAPACITY } ;
        CVec::<T>::check_type_size();
        let size = capacity.checked_mul(mem::size_of::<T>() as usize);
        if size.is_none() {
            return None;
        }
        let ptr = unsafe { malloc(size.unwrap() as size_t) } as *mut T;
        if ptr.is_null() {
            None
        } else {
            Some(CVec {
                ptr: ptr,
                len: 0,
                cap: capacity,
                mutable: true
            })
        }
    }

    // Constructs a new CVec around a given buffer in memory, without copying
    // If the input pointer is null or buf_size is 0, then None is returned
    // The returned CVec CANNOT be modified!
    pub unsafe fn from_raw_buf(ptr: *const T, buf_size: usize) -> Option<CVec<T>> {
        if ptr.is_null() || buf_size == 0 {
            None
        } else {
            Some(CVec {
                ptr: ptr as *mut T,
                len: buf_size,
                cap: buf_size,
                mutable: false
            })
        }
    }

    // Converts this CVec to a raw pointer. The CVec cannot be used after this
    // is called. The raw pointer must be freed by the caller.

    pub fn to_raw_buf(self) -> (*mut T, usize) {
        let ret = (self.ptr, self.len);
        unsafe { mem::forget(self); }
        ret
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // doubles our capacity!
    // returns None if the allocation failed
    // CVec cannot be used after allocation failure
    pub fn double_capacity(&mut self) -> Option<()> {
        assert!(self.mutable);
        let old_size = self.cap * mem::size_of::<T>();
        let size = old_size * 2;
        if old_size > size {
            unsafe { mem::drop(self); }
            return None;
        }
        unsafe {
            let new_ptr = realloc(self.ptr as *mut c_void, size as size_t);
            if new_ptr.is_null() {
                mem::drop(self);
                return None;
            }
            self.ptr = new_ptr as *mut T;
        }
        self.cap = self.cap * 2;
        Some(())
    }

    // returns None if we had to reallocate and it failed
    // CVec cannot be used after allocation failure
    pub fn push(&mut self, value: T) -> Option<()> {
        assert!(self.mutable);
        if self.len == self.cap {
            try_opt!(self.double_capacity());
        }
        assert!(self.cap > self.len);
        unsafe {
            let end = self.ptr.offset(self.len as isize);
            ptr::write(&mut *end, value);
            self.len += 1;
        }
        Some(())
    }

    pub fn pop(&mut self) -> Option<T> {
        assert!(self.mutable);
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.as_slice().get_unchecked(self.len())))
            }
        }
    }

}

#[unsafe_destructor]
impl<T> Drop for CVec<T> {
    fn drop(&mut self) {
        if self.mutable {
            unsafe { free(self.ptr as *mut c_void); }
        }
    }
}

impl<T> AsSlice<T> for CVec<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        unsafe {
            mem::transmute(RawSlice {
                data: self.ptr as *const T,
                len: self.len
            })
        }
    }
}

impl<T: fmt::Show> fmt::Show for CVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Show::fmt(self.as_slice(), f)
    }
}
