use cvec::{CVec, Buf, Iter};
use libc::c_uint;

use header;
use crc32;
use gz_reader::GzBitReader;
use huffman::inflate;

// every gzip file is at least 10 bytes, if not, it's invalid
const GZIP_MIN_LEN: usize = 40;
const GZIP_FILESIZE_OFFSET: usize = 4;
const GZIP_CRC_OFFSET: usize = 8;
const GZIP_FOOTER_LEN: usize = 8;

/// Get the length of the uncompressed file
fn get_uncompressed_len(buffer: &Buf) -> usize {
    assert!(buffer.len() > GZIP_MIN_LEN);
    buffer.get_wide::<c_uint>(buffer.len() - GZIP_FILESIZE_OFFSET).unwrap() as usize
}

/// Get the CRC of the uncompressed file
fn get_crc(buffer: &Buf) -> c_uint {
    assert!(buffer.len() > GZIP_MIN_LEN);
    buffer.get_wide::<c_uint>(buffer.len() - GZIP_CRC_OFFSET).unwrap()
}

/// Decompress the given compressed buffer
pub fn decompress_gz(buffer: Buf) -> Option<Buf> {
    if buffer.len() < GZIP_MIN_LEN {
        return None;
    }
    let out_len = get_uncompressed_len(&buffer);
    let crc = get_crc(&buffer);
    let header = try_opt!(header::parse_header(&buffer));
    let mut out_buf = try_opt!(CVec::with_capacity(out_len));
    decompress_raw(buffer.limit_iter(header.header_len, buffer.len() - GZIP_FOOTER_LEN),
                   &mut out_buf);
    println!("Output buffer length: {}", out_buf.len());
    if check_crc(&out_buf, crc) {
        Some(out_buf)
    } else {
        None
    }
}

/// Decompress the buffer into out_buf
/// Helper function for decompress
fn decompress_raw(buffer: Iter<u8>, out_buf: &mut Buf) {
    let mut gz_reader = match GzBitReader::new(buffer) {
        Some(g) => g,
        None => { return; }
    };
    match inflate(&mut gz_reader, out_buf) {
        Some(()) => {},
        None => { out_buf.clear(); }
    }
}

/// Verify that the CRC matches what we expect
fn check_crc(buffer: &Buf, crc: c_uint) -> bool {
    println!("Calculated CRC sum: {}", crc32::sum(buffer.iter()));
    println!("Actual CRC: {}", crc);
    crc32::sum(buffer.iter()) == crc
}
