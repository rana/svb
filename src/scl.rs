//! Scalar implementation of the stream variable byte compression algorithm.
//!
//! See Lemire C code.
//!
//!   streamvbyte.h
//!     https://github.com/lemire/streamvbyte/blob/170fef1f1401960627d8a4dff08e54116de987db/include/streamvbyte.h
//!
//!   streamvbyte_encode.c
//!     https://github.com/lemire/streamvbyte/blob/c43294a81501e0fdf14adc83818d47f7f9bc1bb6/src/streamvbyte_encode.c
//!

use crate::*;
use anyhow::{bail, Result};
use std::{mem, ptr};

/// Returns the control header's size in bytes.
///
/// Uses scalar instructions.
#[inline]
pub fn hdr_byt_len(unp_len: usize) -> usize {
    (unp_len + 3) / 4
}

/// Returns the data's compressed size in bytes.
///
/// Evaluates each integer's compression size.
///
/// Uses scalar instructions.
#[inline]
pub fn dat_byt_len(unp: &[u32]) -> usize {
    // Compression transforms an integer to 1-byte, 2-bytes, 3-bytes, or 4-bytes.

    // A minimum of 1 byte is used to compress each integer.
    // Add 1 byte for each integer.
    let mut ret: usize = unp.len();
    for val in unp {
        let val = *val;
        // Add sizes for any additional compressed bytes.
        // Determine if minimum compression is 2-bytes, 3-bytes, or 4-bytes.
        ret += (val > 0xFF) as usize + (val > 0xFFFF) as usize + (val > 0xFFFFFF) as usize;
    }
    ret
}

/// Encode a slice of u32 integers.
///
/// Uses scalar instructions.
pub fn enc(vals: &[u32]) -> Result<Vec<u8>> {
    // Check if there is nothing to compress.
    if vals.is_empty() {
        return Ok(vec![]);
    }

    // Create the return slice for compressed bytes.
    // Layout: | Integer Count: usize bytes | Headers: bytes | Compressed Data: bytes |
    let cnt_len = mem::size_of::<usize>();
    let hdr_byt_len = hdr_byt_len(vals.len());
    let ret = vec![0u8; cnt_len + hdr_byt_len + dat_byt_len(vals)];

    // Store the number of compressed integers.
    unsafe {
        let u8_ptr = ret.as_slice().as_ptr();
        let u8_slc_ptr = ptr::slice_from_raw_parts(u8_ptr, cnt_len) as *mut u8;
        ptr::copy_nonoverlapping(vals.len().to_le_bytes().as_ptr(), u8_slc_ptr, cnt_len);
    }

    // Divide the return slice into a header slice.
    // The header slice holds information about how integers are compressed.
    let idx_fst_hdrs = cnt_len;
    let idx_lim_hdrs = idx_fst_hdrs + hdr_byt_len;
    let mut hdrs = unsafe {
        let slc = &ret.as_slice()[idx_fst_hdrs..idx_lim_hdrs];
        let u8_slc_ptr = ptr::slice_from_raw_parts(slc.as_ptr(), slc.len()) as *mut [u8];
        &mut *u8_slc_ptr
    };

    // Divide the return slice into a data slice.
    // The data slice holds the compressed bytes.
    let mut encs = unsafe {
        let slc = &ret.as_slice()[idx_lim_hdrs..];
        let u8_slc_ptr = ptr::slice_from_raw_parts(slc.as_ptr(), slc.len()) as *mut [u8];
        &mut *u8_slc_ptr
    };

    // Setup header variables used for compression.
    // `hdr_shf_len` cycles 0, 2, 4, 6, 0, 2, 4, 6 ...
    let mut hdr: u8 = 0;
    let mut hdr_shf_len: u8 = 0;

    // Iterate through each uncompressed integer.
    for val in vals.iter() {
        // De-reference the uncompressed integer one time as a slight optimization.
        let val = *val;

        // After four integer compressions,
        // Write the header.
        // Reset the header and shift length.
        if hdr_shf_len == 8 {
            hdrs[0] = hdr;
            hdrs = &mut hdrs[1..];
            hdr = 0;
            hdr_shf_len = 0;
        }

        // Determine the number of compressed bytes.
        let hdr_pck: u8 = if val < (1 << 8) {
            HDR_PCK_1
        } else if val < (1 << 16) {
            HDR_PCK_2
        } else if val < (1 << 24) {
            HDR_PCK_3
        } else {
            HDR_PCK_4
        };

        // Compress the integer.
        let pck_len = (hdr_pck + 1) as usize;
        unsafe {
            ptr::copy_nonoverlapping(val.to_le_bytes().as_ptr(), encs.as_mut_ptr(), pck_len);
        }

        encs = &mut encs[pck_len..];

        // Store the header pack size.
        // The header pack size indicates how much compression occurred.
        hdr |= hdr_pck << hdr_shf_len;
        hdr_shf_len += 2;
    }

    // Write the last header.
    hdrs[0] = hdr;

    Ok(ret)
}

/// Decodes a slice of u32 integers.
///
/// Uses scalar instructions.
pub fn dec(encs: &[u8]) -> Result<Vec<u32>> {
    // Check if there is nothing to decompress.
    if encs.is_empty() {
        return Ok(vec![]);
    }

    // Layout: | Integer Count: usize bytes | Headers: bytes | Compressed Data: bytes |

    // Load the number of compressed integers.
    let cnt_len = mem::size_of::<usize>();
    if encs.len() < cnt_len {
        bail!(
            "missing count: too few bytes for integer count (expect:{}, actual:{})",
            cnt_len,
            encs.len(),
        );
    }
    let (int_bytes, _) = encs.split_at(cnt_len);
    let vals_len = usize::from_le_bytes(int_bytes.try_into().unwrap());

    // Divide the input slice into a header slice.
    // The header slice holds information about how integers are compressed.
    let hdr_byt_len = hdr_byt_len(vals_len);
    let avl_len = encs.len() - cnt_len;
    if avl_len < hdr_byt_len {
        bail!(
            "missing headers: too few bytes for header (expect:{}, actual:{})",
            hdr_byt_len,
            avl_len,
        );
    }
    let idx_fst_hdrs = cnt_len;
    let idx_lim_hdrs = idx_fst_hdrs + hdr_byt_len;
    let mut hdrs = unsafe {
        let slc = &encs[idx_fst_hdrs..idx_lim_hdrs];
        let u8_slc_ptr = ptr::slice_from_raw_parts(slc.as_ptr(), slc.len()) as *mut [u8];
        &mut *u8_slc_ptr
    };

    // Divide the input slice into a data slice.
    // The data slice holds the compressed bytes.
    let mut encs = unsafe {
        let slc = &encs[idx_lim_hdrs..];
        let u8_slc_ptr = ptr::slice_from_raw_parts(slc.as_ptr(), slc.len()) as *mut [u8];
        &mut *u8_slc_ptr
    };

    // Create the return slice for decompressed integers.
    let vals = vec![0u32; vals_len];
    let mut decs = unsafe {
        let u8_ptr = vals.as_slice().as_ptr() as *const u8;
        let u8_slc_ptr =
            ptr::slice_from_raw_parts(u8_ptr, mem::size_of::<u32>() * vals.len()) as *mut [u8];
        &mut *u8_slc_ptr
    };

    // Setup variables.
    // `hdr_shf_len` cycles 0, 2, 4, 6, 0, 2, 4, 6 ...
    let mut hdr: u8 = hdrs[0];
    let mut hdr_shf_len: u8 = 0;

    // Iterate through decoded bytes.
    while !decs.is_empty() {
        // After four integer decompressions,
        // Get the next header and reset the shift length.
        if hdr_shf_len == 8 {
            hdrs = &mut hdrs[1..];
            hdr = hdrs[0];
            hdr_shf_len = 0;
        }

        // Extract the integer pack length from the header.
        // The compressed length is `code + 1`.
        let pck_len = (((hdr >> hdr_shf_len) & 0x3) + 1) as usize;

        // Decompress the integer.
        unsafe {
            ptr::copy_nonoverlapping(encs.as_ptr(), decs.as_mut_ptr(), pck_len);
        }
        encs = &mut encs[pck_len..];

        // Increment the header shift length.
        hdr_shf_len += 2;

        // Advance through decoded bytes 4 at a time.
        decs = &mut decs[4..];
    }

    Ok(vals)
}

#[cfg(test)]
mod tst {
    use super::*;
    use anyhow::Ok;
    use rand::{seq::SliceRandom, thread_rng, Rng};

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

    #[test]
    fn enc_dec_n() -> Result<()> {
        for len in [0, 1, 2, 3, 4, 5, 6, 7, 8, 512, 1024] {
            let decs_exp = rnd_vals(len);
            let encs = enc(&decs_exp)?;
            let decs_act = dec(&encs)?;
            assert_eq!(decs_exp, decs_act);
        }

        Ok(())
    }

    /// Returns random values.
    ///
    /// Generates equal quantities of numbers compressible
    /// to 1-byte, 2-bytes, 3-bytes, and 4-bytes.
    fn rnd_vals(len: usize) -> Vec<u32> {
        let mut rng = thread_rng();
        let mut ret: Vec<u32> = Vec::with_capacity(len);
        let mut idx: usize = 0;
        while idx < len {
            ret.push(rng.gen_range(0..=0xFF));
            ret.push(rng.gen_range(0..=0xFFFF));
            ret.push(rng.gen_range(0..=0xFFFFFF));
            ret.push(rng.gen_range(0..=0xFFFFFFFF));
            idx += 4;
        }
        ret.shuffle(&mut rng);
        ret
    }
}
