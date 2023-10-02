#![allow(dead_code)]
#![allow(unused_doc_comments)]
mod fns;
mod lbl;
use anyhow::Result;
use ben::*;
use itr::*;
use crate::lbl::Lbl::{self, *};
use crate::fns::*;
use std::collections::HashMap;
/// Runs a benchmark function analysis.
///
/// cd mtr
/// clear && cargo r -q --profile release
pub fn main() -> Result<()> {
    let mut stdy = new_stdy()?;
    let itr: u16 = 64;
    let mut qry = QryBld::new();
    let scl_enc_a = qry.sel(&[Scl, Enc, A]);
    let scl_enc_b = qry.sel(&[Scl, Enc, B]);
    let scl_enc_c = qry.sel(&[Scl, Enc, C]);
    let scl_enc_d = qry.sel(&[Scl, Enc, D]);
    let scl_enc_e = qry.sel(&[Scl, Enc, E]);
    let scl_enc_f = qry.sel(&[Scl, Enc, F]);
    let scl_enc_g = qry.sel(&[Scl, Enc, G]);
    let scl_dat_a = qry.sel(&[Scl, Dat, A]);
    let scl_dat_e = qry.sel(&[Scl, Dat, E]);
    let scl_dat_g = qry.sel(&[Scl, Dat, G]);
    /// Scalar implementation: A vs B
    qry.cmp(scl_enc_a, scl_enc_b);
    /// Scalar implementation: A vs C
    qry.cmp(scl_enc_a, scl_enc_c);
    /// Scalar implementation: B vs C
    qry.cmp(scl_enc_b, scl_enc_c);
    /// Scalar implementation: A vs D
    qry.cmp(scl_enc_a, scl_enc_d);
    /// Scalar implementation: A vs E
    qry.cmp(scl_enc_a, scl_enc_e);
    /// Scalar implementation: D vs E
    qry.cmp(scl_enc_d, scl_enc_e);
    /// Scalar implementation: E vs F
    qry.cmp(scl_enc_e, scl_enc_f);
    /// Scalar implementation: A vs F
    qry.cmp(scl_enc_a, scl_enc_f);
    /// Scalar implementation: F vs G
    qry.cmp(scl_enc_f, scl_enc_g);
    /// Scalar implementation: Dat: A vs E
    qry.cmp(scl_dat_a, scl_dat_e);
    /// Scalar implementation: Dat: A vs G
    qry.cmp(scl_dat_a, scl_dat_g);
    stdy.run(qry, itr)?;
    Ok(())
}
/// Returns a study with benchmark functions ready to be run.
pub fn new_stdy() -> Result<Stdy<Lbl>> {
    let mut stdy = Stdy::new();
    stdy.reg_bld(
        &[Scl, Enc, A],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Enc, B],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_b::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Enc, C],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_c::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Enc, D],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_d::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Enc, E],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Enc, F],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_f::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Enc, G],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::enc(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Dat, A],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_a::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Dat, E],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_e::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    stdy.reg_bld(
        &[Scl, Dat, G],
        |x| {
            x.ins_prm(
                Len(16),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(64),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(128),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(256),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(512),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(1024),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(2048),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(4096),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(8192),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(16384),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(32768),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(65536),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
            x.ins_prm(
                Len(131072),
                |tme| {
                    let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                    tme.borrow_mut().start();
                    let ret = scl_g::dat_byt_len(&vals);
                    tme.borrow_mut().stop();
                    ret
                },
            );
        },
    );
    Ok(stdy)
}
