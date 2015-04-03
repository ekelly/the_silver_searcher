use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};


#[macro_export]
pub macro_rules! bail {
    () => {
        return null::<c_void>() as *mut c_void;
    }
}

#[macro_export]
pub macro_rules! try_bail {
    ($expr: expr) => (match $expr {
        Option::Some(v) => v,
        Option::None => { bail!() },
    })
}

#[macro_export]
pub macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        Option::Some(v) => v,
        Option::None => {
            return Option::None;
        }
    })
}
