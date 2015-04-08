use cvec::{CVec, Iter};

#[derive(Show)]
pub struct GzBitReader<'a> {
    iter: Iter<'a, u8>,
    buf: u8,
    mask: u8
}

/// Read the GZIP data bit by bit
impl<'a> GzBitReader<'a> {
    pub fn new(mut iter: Iter<'a, u8>) -> Option<GzBitReader<'a>> {
        let starting_buf = try_opt!(iter.next());
        Some(GzBitReader {
            iter: iter,
            buf: *starting_buf,
            mask: 0x01
        })
    }

    #[inline]
    pub fn next_bit(&mut self) -> Option<u32> {
        if self.mask == 0 {
            self.buf = *try_opt!(self.iter.next());
            self.mask = 0x01;
        }
        let bit = if (self.buf & self.mask) > 0 { 1 } else { 0 };
        self.mask <<= 1;
        Some(bit)
    }

    /// reads bits in little-endian form, interprets them in big-endian
    /// as per gzip spec
    pub fn read_bits(&mut self, count: u32) -> Option<u32> {
        let mut bit: u32;
        let mut value: u32 = 0;
        for i in (0 .. count) {
            bit = try_opt!(self.next_bit());
            value |= (bit << i);
        }
        Some(value)
    }
}
