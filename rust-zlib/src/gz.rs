use cvec::{CVec, Buf};
use libc::{c_int, c_uint, c_ulong, c_char, c_uchar, c_void, size_t};
use std::ptr;

use header;

// every gzip file is at least 10 bytes, if not, it's invalid
const GZIP_MIN_LEN: usize = 40;

const GZIP_FILESIZE_OFFSET: usize = 4;
const GZIP_CRC_OFFSET: usize = 8;

const GZIP_FOOTER_LEN: usize = 8;

fn get_uncompressed_len(buffer: &Buf) -> usize {
    assert!(buffer.len() > GZIP_MIN_LEN);
    buffer.get_wide::<c_uint>(buffer.len() - GZIP_FILESIZE_OFFSET).unwrap() as usize
}

fn get_crc(buffer: &Buf) -> usize {
    assert!(buffer.len() > GZIP_MIN_LEN);
    buffer.get_wide::<c_uint>(buffer.len() - GZIP_CRC_OFFSET).unwrap() as usize
}




pub fn decompress_gz(buffer: Buf) -> Option<Buf> {
    if buffer.len() < GZIP_MIN_LEN {
        return None;
    }
    let out_len = get_uncompressed_len(&buffer);
    let crc = get_crc(&buffer);
    let header = try_opt!(header::parse_header(&buffer));
    println!("in_len: {}", buffer.len());
    println!("out_len: {}", out_len);
    println!("crc: {}", crc);
    println!("header len: {}", header.header_len);
    println!("header: {:?}", header);
    let out_buf = try_opt!(CVec::with_capacity(out_len));
    Some(out_buf)
}
