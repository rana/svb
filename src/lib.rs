//! See Lemire C code.
//!
//!   streamvbyte.h
//!     https://github.com/lemire/streamvbyte/blob/170fef1f1401960627d8a4dff08e54116de987db/include/streamvbyte.h
//!
//!   streamvbyte_encode.c
//!     https://github.com/lemire/streamvbyte/blob/c43294a81501e0fdf14adc83818d47f7f9bc1bb6/src/streamvbyte_encode.c
//!

/// Returns the number of control bytes.
#[inline]
pub fn ctl_byt_len(in_arr_len: usize) -> usize {
    (in_arr_len + 3) / 4
}

/// Returns the number of compressed data bytes.
///
/// Goes through each integer to determine actual compression size.
#[inline]
pub fn dat_byt_len(unp: &[u32]) -> usize {
    // Compression transforms an integer to 1-byte, 2-bytes, 3-bytes, or 4-bytes.
    // Add 1 byte for each integer.
    // A minimum of 1 byte is used to compress each integer.
    let mut ret: usize = unp.len();
    for val in unp {
        let val = *val;
        // Add sizes of any additional compressed bytes.
        // Determine if minimum compression is 2-bytes, 3-bytes, or 4-bytes.
        ret += (val > 0xFF) as usize + (val > 0xFFFF) as usize + (val > 0xFFFFFF) as usize;
        // TODO: LOOP UNROLL FOR BLOCK LENGTHS?
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp() {
        println!("      0xFF:{}", 0xFF);
        println!("(1 << 8)-1:{}", (1 << 8)-1);
        println!("     0xFFFF:{}, {0:b}", 0xFFFF);
        println!("(1 << 16)-1:{}", (1 << 16)-1);
        println!("   0xFFFFFF:{}, {0:b}", 0xFFFFFF);
        println!("(1 << 24)-1:{}", (1 << 24)-1);

        println!("0x1FF:{}", 0x100);
        println!("1<<7:{}", 1 << 7);
        
        println!("(1<<8)-1:{}", (1 << 8) - 1);
    }

    #[test]
    fn dat_byt_len_n() {
        assert_eq!(dat_byt_len(&[u32::MIN]), 1);
        assert_eq!(dat_byt_len(&[1 << 7]), 1);
        assert_eq!(dat_byt_len(&[(1 << 8) - 1]), 1);
        assert_eq!(dat_byt_len(&[1 << 8]), 2);
        assert_eq!(dat_byt_len(&[1 << 15]), 2);
        assert_eq!(dat_byt_len(&[(1 << 16) - 1]), 2);
        assert_eq!(dat_byt_len(&[1 << 16]), 3);
        assert_eq!(dat_byt_len(&[1 << 23]), 3);
        assert_eq!(dat_byt_len(&[(1 << 23) - 1]), 3);
        assert_eq!(dat_byt_len(&[1 << 24]), 4);
        assert_eq!(dat_byt_len(&[1 << 31]), 4);
        assert_eq!(dat_byt_len(&[u32::MAX]), 4);
    }

    #[test]
    fn ctl_byt_len_n() {
        assert_eq!(ctl_byt_len(1), 1);
        assert_eq!(ctl_byt_len(2), 1);
        assert_eq!(ctl_byt_len(3), 1);
        assert_eq!(ctl_byt_len(4), 1);
        assert_eq!(ctl_byt_len(5), 2);
        assert_eq!(ctl_byt_len(6), 2);
        assert_eq!(ctl_byt_len(7), 2);
        assert_eq!(ctl_byt_len(8), 2);
        assert_eq!(ctl_byt_len(9), 3);
        assert_eq!(ctl_byt_len(10), 3);
        assert_eq!(ctl_byt_len(11), 3);
        assert_eq!(ctl_byt_len(12), 3);
    }
}
