use std::fmt::Show;
use cvec::Buf;

/*
Flags:
bit 0   FTEXT
bit 1   FHCRC
bit 2   FEXTRA
bit 3   FNAME
bit 4   FCOMMENT
bit 5   reserved
bit 6   reserved
bit 7   reserved
*/
#[derive(PartialEq, Show)]
struct Flags {
    FTEXT: bool,
    FHCRC: bool,
    FNAME: bool,
    FEXTRA: bool,
    FCOMMENT: bool,
}

impl Flags {
    fn new(flags: u8) -> Flags {
        Flags {
            FTEXT: flags & 1 != 0,
            FHCRC: flags & 2 != 0,
            FNAME: flags & 4 != 0,
            FEXTRA: flags & 8 != 0,
            FCOMMENT: flags & 16 != 0,
        }
    }
}

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
    flags: Flags,
    mtime: u32,
    extra_flags: u8,
    os: u8,
    extra: Option<Vec<u8>>,
    fname: Option<String>,
    comment: Option<String>,
    crc: Option<u16>
}

/// Return a GZIP header structure representing the information
/// contained in the beginning of the given Buf
pub fn parse_header(buffer: &Buf) -> Option<GZHeader> {
    let mut iter = buffer.iter();

    // Header fields
    let mut comp_method: u8 = 0;
    let mut flags: Flags;
    let mut mtime: u32 = 0;
    let mut extra_flags: u8 = 0;
    let mut os: u8 = 0;

    // Check that the magic number is right
    if *try_opt!(iter.next()) == 0x1f && *try_opt!(iter.next()) == 0x8b {
        comp_method = *try_opt!(iter.next());
        flags = Flags::new(*try_opt!(iter.next()));
        // We need to shift mtime because it's 4 bytes
        mtime = (*try_opt!(iter.next()) as u32) << 24;
        mtime += (*try_opt!(iter.next()) as u32) << 16;
        mtime += (*try_opt!(iter.next()) as u32) << 8;
        mtime += (*try_opt!(iter.next()) as u32);
        extra_flags = *try_opt!(iter.next());
        os = *try_opt!(iter.next());

        // Optional stuff
        let extra = if_opt!(flags.FEXTRA, {
            let mut len: u16 = (*try_opt!(iter.next()) as u16) << 8;
            len += (*try_opt!(iter.next()) as u16);
            let mut data = Vec::with_capacity(len as usize);
            for i in 0..(len as usize) {
                let byte: u8 = *try_opt!(iter.next());
                data.push(byte);
            }
            data
        });
        let name = match if_opt!(flags.FNAME, {
            let mut name_bytes = Vec::with_capacity(512);
            while let Some(&byte) = iter.next() {
                name_bytes.push(byte);
                if byte == 0x00 {
                    break
                }
            }
            match String::from_utf8(name_bytes) {
                Ok(result) => Some(result),
                Err(..) => None
            }
        }) {
            Some(n) => n,
            None => None
        };

        Some(GZHeader {
            compression_method: comp_method,
            flags: flags,
            mtime: mtime,
            extra_flags: extra_flags,
            os: os,
            extra: extra,
            fname: name,
            comment: None,
            crc: None
        })
    } else {
        None
    }
}

#[cfg(test)]
mod parse_header_tests {
    use super::{parse_header, GZHeader, Flags};
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
    fn test_basic_header() {
        static HEADER_BYTES: &'static [u8] = &[
              0x1f, 0x8b, 0x08, 0x00, 0x12, 0x34, 0x56, 0x78,
              0x00, 0x07];

        let buffer = create_buf(HEADER_BYTES);
        let results = parse_header(&buffer).unwrap();
        assert_eq!(results.compression_method, 8);
        assert_eq!(results.flags, Flags {
            FTEXT: false, FHCRC: false, FNAME: false,
            FEXTRA: false, FCOMMENT: false
        });
        assert_eq!(results.mtime, 305419896);
        assert_eq!(results.extra_flags, 0);
        assert_eq!(results.os, 7);
    }


    #[test]
    fn test_complex_header() {
        static HEADER_BYTES: &'static [u8] = &[
              0x1f, 0x8b, 0x08, 0x08, 0x12, 0x34, 0x56, 0x78,
              0x00, 0x07, 0x00, 0x04, 0x12, 0x34, 0x56, 0x78];

        let buffer = create_buf(HEADER_BYTES);
        let results = parse_header(&buffer).unwrap();
        assert_eq!(results.compression_method, 8);
        assert_eq!(results.flags, Flags {
            FTEXT: false, FHCRC: false, FNAME: false,
            FEXTRA: true, FCOMMENT: false
        });
        assert_eq!(results.mtime, 305419896);
        assert_eq!(results.extra_flags, 0);
        assert_eq!(results.os, 7);
        assert_eq!(results.extra, Some(vec![0x12, 0x34, 0x56, 0x78]));
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
