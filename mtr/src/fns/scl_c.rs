use anyhow::{bail, Result};
use std::{mem, ptr};

/// Header bit value `00` indicating compression to 1 byte.
pub const HDR_PCK_1: u8 = 0;
/// Header bit value `01` indicating compression to 2 bytes.
pub const HDR_PCK_2: u8 = 1;
/// Header bit value `10` indicating compression to 3 bytes.
pub const HDR_PCK_3: u8 = 2;
/// Header bit value `11` indicating no compression and 4 bytes of packing.
pub const HDR_PCK_4: u8 = 3;

/// Returns the header length in bytes.
#[inline]
pub fn hdr_len(unp_len: usize) -> usize {
    (unp_len + 3) / 4
}

/// Returns headers and the data's compressed length in bytes.
#[allow(clippy::slow_vector_initialization)]
#[inline]
pub fn hdrs_and_dat_len(decs: &[u32]) -> (Vec<u8>, usize) {
    // Compression transforms an integer to 1-byte, 2-bytes, 3-bytes, or 4-bytes.

    // Create an empty headers vector.
    let hdr_len = hdr_len(decs.len());
    let mut ret_hdrs: Vec<u8> = Vec::with_capacity(hdr_len);
    ret_hdrs.resize(hdr_len, 0);
    let mut hdrs = ret_hdrs.as_mut_slice();

    // Setup header variables.
    // `hdr_shf_len` cycles 0, 2, 4, 6, 0, 2, 4, 6 ...
    let mut hdr_shf_len: u8 = 0;

    // A minimum of 1 byte is used to compress each integer.
    // Set one byte for each integer.
    let mut dat_len: usize = decs.len();

    // Determine if any additional compressed bytes are needed.
    for val in decs {
        // After four integer compressions:
        //   * Advance to the next header byte.
        //   * Reset the header shift length.
        if hdr_shf_len == 8 {
            hdrs = &mut hdrs[1..];
            hdr_shf_len = 0;
        }

        // De-reference the uncompressed integer one time as a slight optimization.
        let val = *val;

        // Determine the header for the current integer.
        let hdr = (val > 0xFF) as u8 + (val > 0xFFFF) as u8 + (val > 0xFFFFFF) as u8;
        // println!("val:{}, hdr:{:02b}, hdr_int:{}", val, hdr, hdr);

        // Store the header.
        hdrs[0] |= hdr << hdr_shf_len;
        // Increment the shift length for two bits.
        hdr_shf_len += 2;

        // Store the header as part of compressed data length.
        dat_len += hdr as usize;
    }

    (ret_hdrs, dat_len)
}

/// Encodes a slice of u32 integers.
#[allow(clippy::slow_vector_initialization)]
pub fn enc(decs: &[u32]) -> Result<Vec<u8>> {
    // Check if there is nothing to compress.
    if decs.is_empty() {
        return Ok(vec![]);
    }

    // Create the return slice for compressed bytes.
    // Layout: | Integer Count: usize bytes | Headers: bytes | Compressed Data: bytes |
    let cnt_len = mem::size_of::<usize>();
    let hdr_len = hdr_len(decs.len());
    let cnt_hdr_len = cnt_len + hdr_len;
    let mut ret = vec![0u8; cnt_hdr_len + decs.len() * 4];
    // let mut ret: Vec<u8> = Vec::with_capacity(cnt_hdr_len);
    // ret.resize(cnt_hdr_len, 0);

    // let (mut cnts, mut hdrs) = ret.split_at_mut(cnt_len);

    // Store the number of compressed integers.
    unsafe {
        let u8_ptr = ret.as_slice().as_ptr();
        let u8_slc_ptr = ptr::slice_from_raw_parts(u8_ptr, cnt_len) as *mut u8;
        ptr::copy_nonoverlapping(decs.len().to_le_bytes().as_ptr(), u8_slc_ptr, cnt_len);
    }

    // Divide the return slice into a header slice.
    // The header slice holds information about how integers are compressed.
    let idx_fst_hdrs = cnt_len;
    let idx_lim_hdrs = idx_fst_hdrs + hdr_len;
    let hdrs = unsafe {
        let slc = &ret.as_slice()[idx_fst_hdrs..];
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
    let mut hdr_shf_len: u8 = 0;
    let mut hdr_idx: usize = 0;

    // A minimum of 1 byte is used to compress each integer.
    // Set one byte for each integer.
    let mut dat_len: usize = decs.len();

    // Determine headers for each uncompressed integer.
    // Determine the data's compressed length in bytes.
    for dec in decs.iter() {
        // De-reference the uncompressed integer one time as a slight optimization.
        let dec = *dec;

        // After four integer compressions:
        //   * Advance to the next header byte.
        //   * Reset the header shift length.
        if hdr_shf_len == 8 {
            // hdrs2 = &mut hdrs2[1..];
            hdr_shf_len = 0;
            hdr_idx += 1;
        }

        // Determine the header for the current integer.
        let hdr = (dec > 0xFF) as u8 + (dec > 0xFFFF) as u8 + (dec > 0xFFFFFF) as u8;
        // println!("dec:{dec}, hdr:{hdr:02b}, hdr_int:{hdr}, hdr_idx:{hdr_idx} hdr_shf_len:{hdr_shf_len}");

        // Store the header.
        hdrs[hdr_idx] |= hdr << hdr_shf_len;
        // Increment the shift length for two bits.
        hdr_shf_len += 2;

        // Store the header as part of compressed data length.
        dat_len += hdr as usize;
    }
    // println!("hdrs:{hdrs:?}");

    // // Expand the return vector to accomodate the compressed data.
    // println!("(ret.len:{}", ret.len());
    ret.truncate(cnt_hdr_len + dat_len);
    // println!("(ret.len:{}", ret.len());

    // Divide the return slice into a data slice.
    // The data slice holds the compressed bytes.
    // let (_, mut encs) = ret.split_at_mut(cnt_hdr_len);
    // let mut encs = unsafe {
    //     let slc = &ret.as_slice()[idx_lim_hdrs..];
    //     let u8_slc_ptr = ptr::slice_from_raw_parts(slc.as_ptr(), slc.len()) as *mut [u8];
    //     &mut *u8_slc_ptr
    // };

    // Compress data.
    hdr_shf_len = 0;
    hdr_idx = 0;
    for dec in decs.iter() {
        // De-reference the uncompressed integer one time as a slight optimization.
        let dec = *dec;

        // After four integer compressions:
        //   * Advance to the next header byte.
        //   * Reset the header shift length.
        if hdr_shf_len == 8 {
            hdr_shf_len = 0;
            hdr_idx += 1;
        }

        // Extract the header for the current integer.
        let hdr = (hdrs[hdr_idx] >> hdr_shf_len) & 0b11;
        // println!("hdrs[{}]:{:08b}", hdr_idx, hdrs[hdr_idx]);
        // println!("dec:{dec} hdr:{hdr:02b}, hdr_int:{hdr}, hdr_idx:{hdr_idx}, hdr_shf_len:{hdr_shf_len}");

        // Compress the integer.
        let pck_len = (hdr + 1) as usize;
        unsafe {
            ptr::copy_nonoverlapping(dec.to_le_bytes().as_ptr(), encs.as_mut_ptr(), pck_len);
        }

        // Advance the encoded bytes buffer.
        encs = &mut encs[pck_len..];

        // Increment the shift length for two bits.
        hdr_shf_len += 2;
    }

    Ok(ret)
}

/// Decodes a slice of u32 integers.
///
/// Uses scalar instructions.
///
/// # Arguments
///
/// * `encs` - A compressed slice of u8s.
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
    let hdr_byt_len = hdr_len(vals_len);
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
        // std::slice::from_raw_parts_mut(slc.as_mut_ptr(), slc.len())
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
        let pck_len = (((hdr >> hdr_shf_len) & 0b11) + 1) as usize;

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
    use itr::rnds_eql_byt;

    #[test]
    fn enc_dec_n() -> Result<()> {
        for len in [0, 1, 2, 3, 4, 5, 6, 7, 8, 512, 1024] {
        // for len in [7] {
            // println!("len:{}", len);
            let decs_exp: Vec<u32> = rnds_eql_byt().take(len).collect();
            let encs = enc(&decs_exp)?;
            // println!("encs:{encs:?}");
            let decs_act = dec(&encs)?;
            assert_eq!(decs_exp, decs_act);
        }

        Ok(())
    }
}
