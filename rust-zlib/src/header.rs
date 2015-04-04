use std::fmt::Show;
use cvec;
use cvec::{Iter, Buf, CVec};

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

/// GZHeader consists of the following fields.
/// Optional fields are, naturally, Options in the GZHeader.
/// Whether or not they exist depends on whether it's associated
/// flag bit is set.
#[derive(PartialEq, Show)]
pub struct GZHeader {
    compression_method: u8,
    flags: Flags,
    mtime: u32,
    extra_flags: u8,
    os: u8,
    extra: Option<(String, Vec<u8>)>,
    fname: Option<String>,
    comment: Option<String>,
    crc: Option<u16>
}

/// Return a GZIP header structure representing the information
/// contained in the beginning of the given Buf
pub fn parse_header(buffer: &cvec::Buf) -> Option<GZHeader> {
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
        // We don't know how to decompress anything other than 8
        if (comp_method != 8) { return None; }
        flags = Flags::new(*try_opt!(iter.next()));
        // We need to shift mtime because it's 4 bytes
        mtime = (*try_opt!(iter.next()) as u32) << 24;
        mtime += (*try_opt!(iter.next()) as u32) << 16;
        mtime += (*try_opt!(iter.next()) as u32) << 8;
        mtime += *try_opt!(iter.next()) as u32;
        extra_flags = *try_opt!(iter.next());
        os = *try_opt!(iter.next());

        // Optional stuff
        let extra = get_extra(&flags, &mut iter);
        let name = get_string(flags.FNAME, &mut iter);
        let comment = get_string(flags.FCOMMENT, &mut iter);
        let crc = get_crc(&flags, &mut iter);

        Some(GZHeader {
            compression_method: comp_method,
            flags: flags,
            mtime: mtime,
            extra_flags: extra_flags,
            os: os,
            extra: extra,
            fname: name,
            comment: comment,
            crc: crc
        })
    } else {
        None
    }
}

fn get_extra(flags: &Flags, iter: &mut cvec::Iter<u8>) -> Option<(String, Vec<u8>)> {
    if_opt!(flags.FEXTRA, {
        let mut id_bytes = Vec::with_capacity(2);
        id_bytes.push(*try_opt!(iter.next()));
        id_bytes.push(*try_opt!(iter.next()));
        let id = match String::from_utf8(id_bytes) {
            Ok(string) => string,
            Err(..) => return None
        };
        let mut len: u16 = (*try_opt!(iter.next()) as u16) << 8;
        len += (*try_opt!(iter.next()) as u16);
        let mut data = Vec::with_capacity(len as usize);
        for i in 0..(len as usize) {
            let byte: u8 = *try_opt!(iter.next());
            data.push(byte);
        }
        (id, data)
    })
}

fn get_string(flag: bool, iter: &mut cvec::Iter<u8>) -> Option<String> {
    match if_opt!(flag, {
        let mut str_bytes = Vec::with_capacity(512);
        while let Some(&byte) = iter.next() {
            if byte == 0x00 {
                break
            }
            str_bytes.push(byte);
        }
        match String::from_utf8(str_bytes) {
            Ok(result) => Some(result),
            Err(..) => None
        }
    }) {
        Some(n) => n,
        None => None
    }
}

fn get_crc(flags: &Flags, iter: &mut cvec::Iter<u8>) -> Option<u16> {
    if_opt!(flags.FHCRC, {
        let mut crc: u16 = (*try_opt!(iter.next()) as u16) << 8;
        crc += *try_opt!(iter.next()) as u16;
        crc
    })
}

#[cfg(test)]
mod parse_header_tests {
    use super::{parse_header, GZHeader, Flags};
    use std::mem;
    use cvec;

    fn create_buf(raw: &[u8]) -> cvec::Buf {
        let mut buffer = cvec::CVec::with_capacity(raw.len()).unwrap();
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
            // magic header
            0x1f, 0x8b,
            // compression method
            0x08,
            // Flags
            0x1f,
            // time
            0x12, 0x34, 0x56, 0x78,
            // extra flags
            0x00,
            // OS
            0x07,
            // extra id + length + extra
            0x41, 0x70, 0x00, 0x04, 0x12, 0x34, 0x56, 0x78,
            // name
            0x41, 0x42, 0x43, 0x44, 0x45, 0x00,
            // comment
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00,
            // CRC
            0x00, 0x01];

        let buffer = create_buf(HEADER_BYTES);
        let results = parse_header(&buffer).unwrap();
        assert_eq!(results.compression_method, 8);
        assert_eq!(results.flags, Flags {
            FTEXT: true, FHCRC: true, FNAME: true,
            FEXTRA: true, FCOMMENT: true
        });
        assert_eq!(results.mtime, 305419896);
        assert_eq!(results.extra_flags, 0);
        assert_eq!(results.os, 7);
        assert_eq!(results.extra, Some(("Ap".to_string(), vec![0x12, 0x34, 0x56, 0x78])));
        assert_eq!(results.fname, Some("ABCDE".to_string()));
        assert_eq!(results.comment, Some("AAAAAA".to_string()));
        assert_eq!(results.crc, Some(1));
    }

    #[test]
    fn test_partial_header() {
        static HEADER_BYTES: &'static [u8] = &[
            // magic header
            0x1f, 0x8b,
            // compression method
            0x08,
            // Flags
            0x17,
            // time
            0x12, 0x34, 0x56, 0x78,
            // extra flags
            0x00,
            // OS
            0x07,
            // name
            0x41, 0x42, 0x43, 0x44, 0x45, 0x00,
            // comment
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00,
            // CRC
            0x00, 0x01];

        let buffer = create_buf(HEADER_BYTES);
        let results = parse_header(&buffer).unwrap();
        assert_eq!(results.compression_method, 8);
        assert_eq!(results.flags, Flags {
            FTEXT: true, FHCRC: true, FNAME: true,
            FEXTRA: false, FCOMMENT: true
        });
        assert_eq!(results.mtime, 305419896);
        assert_eq!(results.extra_flags, 0);
        assert_eq!(results.os, 7);
        assert_eq!(results.extra, None);
        assert_eq!(results.fname, Some("ABCDE".to_string()));
        assert_eq!(results.comment, Some("AAAAAA".to_string()));
        assert_eq!(results.crc, Some(1));
    }

    #[test]
    fn test_invalid_header() {
        // Magic bytes are wrong
        let HEADER_BYTES: &[u8] = &[
              0x1f, 0x8c, 0x08, 0x00, 0x12, 0x34, 0x56, 0x78,
              0x00, 0x07];
        let buffer = create_buf(HEADER_BYTES);
        assert_eq!(parse_header(&buffer), None);
        // Wrong compression type
        let HEADER_BYTES: &[u8] = &[
              0x1f, 0x8b, 0x07, 0x00, 0x12, 0x34, 0x56, 0x78,
              0x00, 0x07];
        let buffer = create_buf(HEADER_BYTES);
        assert_eq!(parse_header(&buffer), None);
    }

}
