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

fn get_uncompressed_len(buffer: &Buf) -> usize {
    assert!(buffer.len() > GZIP_MIN_LEN);
    buffer.get_wide::<c_uint>(buffer.len() - GZIP_FILESIZE_OFFSET).unwrap() as usize
}

fn get_crc(buffer: &Buf) -> c_uint {
    assert!(buffer.len() > GZIP_MIN_LEN);
    buffer.get_wide::<c_uint>(buffer.len() - GZIP_CRC_OFFSET).unwrap()
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
    let mut out_buf = try_opt!(CVec::with_capacity(out_len));
    decompress_raw(buffer.limit_iter(header.header_len, buffer.len() - GZIP_FOOTER_LEN),
                   &mut out_buf);
    try_opt!(check_crc(&out_buf, crc));
    Some(out_buf)
}

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

fn check_crc(buffer: &Buf, crc: c_uint) -> Option<()> {
    if crc32::sum(buffer.iter()) == crc {
        Some(())
    } else {
        None
    }
}
