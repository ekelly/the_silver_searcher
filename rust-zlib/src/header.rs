use std::fmt::Show;
use cvec::Buf;

/*
 unsigned char id[ 2 ];
 unsigned char compression_method;
 unsigned char flags;
 unsigned char mtime[ 4 ];
 unsigned char extra_flags;
 unsigned char os;
 */

#[derive(PartialEq, Show)]
pub struct GZHeader {
    compression_method: u8,
    flags: u8,
    mtime: u32,
    extra_flags: u8,
    os: u8
}

/// Return a GZIP header structure representing the information
/// contained in the beginning of the given Buf
pub fn parse_header(buffer: &Buf) -> Option<GZHeader> {
    let mut iter = buffer.iter();

    // Header fields
    let mut comp_method: u8 = 0;
    let mut flags: u8 = 0;
    let mut mtime: u32 = 0;
    let mut extra_flags: u8 = 0;
    let mut os: u8 = 0;

    // Check that the magic number is right
    if *try_opt!(iter.next()) == 0x1f && *try_opt!(iter.next()) == 0x8b {
        comp_method = *try_opt!(iter.next());
        flags = *try_opt!(iter.next());
        // We need to shift mtime because it's 4 bytes
        mtime = (*try_opt!(iter.next()) as u32) << 24;
        mtime += (*try_opt!(iter.next()) as u32) << 16;
        mtime += (*try_opt!(iter.next()) as u32) << 8;
        mtime += (*try_opt!(iter.next()) as u32);
        extra_flags = *try_opt!(iter.next());
        os = *try_opt!(iter.next());

        Some(GZHeader {
            compression_method: comp_method,
            flags: flags,
            mtime: mtime,
            extra_flags: extra_flags,
            os: os
        })
    } else {
        None
    }
}

#[cfg(test)]
mod parse_header_tests {
    use super::{parse_header, GZHeader};
    use cvec::{Buf, CVec};
    use std::mem;

    fn create_buf(raw: &[u8]) -> Buf {
        let mut buffer = CVec::with_capacity(raw.len()).unwrap();
        for &byte in raw.iter() {
            buffer.push(byte);
        }
        buffer
    }

    #[test]
    fn test_parse_header() {
        println!("parsing header");
        static HEADER_BYTES: &'static [u8] = &[
              0x1f, 0x8b, 0x08, 0x00, 0x12, 0x34, 0x56, 0x78,
              0x00, 0x07];

        let buffer = create_buf(HEADER_BYTES);
        let results = parse_header(&buffer).unwrap();
        assert_eq!(results.compression_method, 8);
        assert_eq!(results.flags, 0);
        assert_eq!(results.mtime, 305419896);
        assert_eq!(results.extra_flags, 0);
        assert_eq!(results.os, 7);
    }

    #[test]
    fn test_invalid_header() {
        static HEADER_BYTES: &'static [u8] = &[
              0x1f, 0x8c, 0x08, 0x00, 0x12, 0x34, 0x56, 0x78,
              0x00, 0x07];

        let buffer = create_buf(HEADER_BYTES);
        assert_eq!(parse_header(&buffer), None);
    }

}
