//! SIMD implementation of the stream variable byte compression algorithm.

use crate::*;
use anyhow::{bail, Result};
use std::{mem, ptr};

/// Returns the data's compressed size in bytes.
///
/// This does not include the header bytes.
/// 
/// Evaluates each integer's compression size.
///
/// Uses SIMD instructions.
#[inline]
pub fn dat_byt_len(unp: &[u32]) -> usize {
    todo!();
}

/// Encode a slice of u32 integers.
///
/// Uses SIMD instructions.
pub fn enc(vals: &[u32]) -> Result<Vec<u8>> {
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
