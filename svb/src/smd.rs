//! SIMD implementation of the stream variable byte compression algorithm.

use crate::scl;
use crate::*;
use anyhow::{bail, Result};
use std::arch::x86_64::__m128i as m128;
use std::arch::x86_64::_mm_adds_epu16;
use std::arch::x86_64::_mm_loadu_si128 as load;
use std::arch::x86_64::_mm_min_epi16;
use std::arch::x86_64::_mm_min_epu8;
use std::arch::x86_64::_mm_movemask_epi8;
use std::arch::x86_64::_mm_packus_epi16;
use std::arch::x86_64::_mm_set1_epi16;
use std::arch::x86_64::_mm_set1_epi8;
use std::{mem, ptr};



/// Returns the data's compressed size in bytes.
///
/// This does not include the header bytes.
///
/// Evaluates each integer's compression size.
///
/// Uses SIMD instructions.
#[inline]
pub fn dat_byt_len(mut decs: &[u32]) -> usize {
    let mut ret: usize = 0;

    unsafe {
        // Setup bit masks
        //   x01:00000000 00000001
        // x7F00:01111111 00000000
        let mask_01: m128 = _mm_set1_epi8(0x01);
        let mask_7f00: m128 = _mm_set1_epi16(0x7F00);

        // Count with SIMD instructions for all but the last eight elements.
        // Set a limit to a modulos of 8 by removing smallest three bits with int 7.
        //  len:35 -> lim:32, len:47 -> lim:40, len:55 -> lim:48
        let lim = decs.len() & !7usize;
        let mut idx = 0usize;
        let unp_ptr = decs.as_ptr();
        while idx < lim {
            // Load two SIMD blocks of uncompressed integers.
            let lo = load(unp_ptr.add(idx) as *const m128);
            let hi = load(unp_ptr.add(idx + 4) as *const m128);

            // Calculate header bytes.
            // Process two header bytes at a time, and eight uncompressed u32 integers.
            let mut m0 = _mm_min_epu8(mask_01, lo);
            let m1 = _mm_min_epu8(mask_01, hi);
            m0 = _mm_packus_epi16(m0, m1);
            m0 = _mm_min_epi16(m0, mask_01); // convert 0x01FF to 0x0101
            m0 = _mm_adds_epu16(m0, mask_7f00); // convert: 0x0101 to 0x8001, 0xFF01 to 0xFFFF

            // Calculate two keys for a length lookup table.
            let keys = _mm_movemask_epi8(m0) as usize;

            // Accumulate the compression length for the eight u32 integers.
            ret += TBL_LENS[keys & 0xFF] as usize;
            ret += TBL_LENS[keys >> 8] as usize;

            idx += 8;
        }
    }

    // Calculate the length of the remaining integers 
    // for scalar instructions.
    // 0&7:0, 1&7:1, 6&7:6, 8&7:0
    let rem_len = decs.len() & 7;
    // Shrink the decompressed slice to the the last seven, or less integers.
    decs = &decs[decs.len() - rem_len..];
    // Measure compression length for any remaining last integers.
    ret += scl::dat_byt_len(decs);

    ret
}

/// Encode a slice of u32 integers.
///
/// Uses SIMD instructions.
pub fn enc(decs: &[u32]) -> Result<Vec<u8>> {
    todo!();
}

/// Decodes a slice of u32 integers.
///
/// Uses SIMD instructions.
pub fn dec(encs: &[u8]) -> Result<Vec<u32>> {
    todo!();
}

#[cfg(test)]
mod tst {
    use super::*;
    use anyhow::Ok;
    use itr::rnds_eql_byt;

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
        let decs: Vec<u32> = rnds_eql_byt().take(2).collect();
        assert_eq!(dat_byt_len(&decs), 3);
        let decs: Vec<u32> = rnds_eql_byt().take(7).collect();
        assert_eq!(dat_byt_len(&decs), 16);
        let decs: Vec<u32> = rnds_eql_byt().take(8).collect();
        assert_eq!(dat_byt_len(&decs), 20);
        let decs: Vec<u32> = rnds_eql_byt().take(1024).collect();
        assert_eq!(dat_byt_len(&decs), 2560);
    }

    #[test]
    fn enc_dec_n() -> Result<()> {
        for len in [0, 1, 2, 3, 4, 5, 6, 7, 8, 512, 1024] {
            let decs_exp: Vec<u32> = rnds_eql_byt().take(len).collect();
            let encs = enc(&decs_exp)?;
            let decs_act = dec(&encs)?;
            assert_eq!(decs_exp, decs_act);
        }

        Ok(())
    }
}

/// A lookup table of compression lengths.
pub const TBL_LENS: [u8; 256] = [
    4, 5, 6, 7, 5, 6, 7, 8, 6, 7, 8, 9, 7, 8, 9, 10, 5, 6, 7, 8, 6, 7, 8, 9, 7, 8, 9, 10, 8, 9, 10,
    11, 6, 7, 8, 9, 7, 8, 9, 10, 8, 9, 10, 11, 9, 10, 11, 12, 7, 8, 9, 10, 8, 9, 10, 11, 9, 10, 11,
    12, 10, 11, 12, 13, 5, 6, 7, 8, 6, 7, 8, 9, 7, 8, 9, 10, 8, 9, 10, 11, 6, 7, 8, 9, 7, 8, 9, 10,
    8, 9, 10, 11, 9, 10, 11, 12, 7, 8, 9, 10, 8, 9, 10, 11, 9, 10, 11, 12, 10, 11, 12, 13, 8, 9,
    10, 11, 9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14, 6, 7, 8, 9, 7, 8, 9, 10, 8, 9, 10, 11,
    9, 10, 11, 12, 7, 8, 9, 10, 8, 9, 10, 11, 9, 10, 11, 12, 10, 11, 12, 13, 8, 9, 10, 11, 9, 10,
    11, 12, 10, 11, 12, 13, 11, 12, 13, 14, 9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14, 12, 13,
    14, 15, 7, 8, 9, 10, 8, 9, 10, 11, 9, 10, 11, 12, 10, 11, 12, 13, 8, 9, 10, 11, 9, 10, 11, 12,
    10, 11, 12, 13, 11, 12, 13, 14, 9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14, 12, 13, 14, 15,
    10, 11, 12, 13, 11, 12, 13, 14, 12, 13, 14, 15, 13, 14, 15, 16,
];