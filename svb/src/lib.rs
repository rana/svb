//! Stream variable byte compression
//!
//!
//!
//!
//! See Lemire C code.
//!
//!   streamvbyte.h
//!     https://github.com/lemire/streamvbyte/blob/170fef1f1401960627d8a4dff08e54116de987db/include/streamvbyte.h
//!
//!   streamvbyte_encode.c
//!     https://github.com/lemire/streamvbyte/blob/c43294a81501e0fdf14adc83818d47f7f9bc1bb6/src/streamvbyte_encode.c
//!
pub mod scl;
pub mod smd;

/// Header bit value `00` indicating compression to 1 byte.
pub const HDR_PCK_1: u8 = 0;
/// Header bit value `01` indicating compression to 2 bytes.
pub const HDR_PCK_2: u8 = 1;
/// Header bit value `10` indicating compression to 3 bytes.
pub const HDR_PCK_3: u8 = 2;
/// Header bit value `11` indicating no compression and 4 bytes of packing.
pub const HDR_PCK_4: u8 = 3;

/// Returns the control header's size in bytes.
#[inline]
pub fn hdr_byt_len(unp_len: usize) -> usize {
    (unp_len + 3) / 4
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    fn hdr_byt_len_n() {
        assert_eq!(hdr_byt_len(1), 1);
        assert_eq!(hdr_byt_len(2), 1);
        assert_eq!(hdr_byt_len(3), 1);
        assert_eq!(hdr_byt_len(4), 1);
        assert_eq!(hdr_byt_len(5), 2);
        assert_eq!(hdr_byt_len(6), 2);
        assert_eq!(hdr_byt_len(7), 2);
        assert_eq!(hdr_byt_len(8), 2);
        assert_eq!(hdr_byt_len(9), 3);
        assert_eq!(hdr_byt_len(10), 3);
        assert_eq!(hdr_byt_len(11), 3);
        assert_eq!(hdr_byt_len(12), 3);
    }
}
