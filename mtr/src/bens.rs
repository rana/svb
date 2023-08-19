use anyhow::Result;
use ben::*;
use itr::*;
use crate::lbl::Lbl;
use crate::fns::scl_a;
use crate::fns::scl_b;
use crate::fns::scl_c;
use crate::fns::scl_d;
use crate::fns::scl_e;
/// Run analysis on benchmark functions.
pub fn run_mtr_qrys() -> Result<()> {
    let set = new_mtr_set()?;
    let itr: u32 = 64;
    set.qry(Qry {
        frm: vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::E]],
        grp: Some(
            vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::E]],
        ),
        srt: Some(Lbl::Len(0)),
        sta: Some(Sta::Mdn),
        trn: Some(Lbl::Len(0)),
        cmp: true,
        itr,
    })?;
    Ok(())
}
/// Returns a populated set of benchmark functions.
pub fn new_mtr_set() -> Result<Set<Lbl>> {
    let ret = Set::new();
    {
        let sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::A]);
        sec.ins_prm(
            &[Lbl::Len(16)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(64)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(128)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(256)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(512)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(1024)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(2048)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(4096)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(8192)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(16384)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32768)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(65536)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(131072)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
    }
    {
        let sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::B]);
        sec.ins_prm(
            &[Lbl::Len(16)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(64)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(128)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(256)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(512)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(1024)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(2048)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(4096)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(8192)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(16384)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32768)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(65536)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(131072)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
    }
    {
        let sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::C]);
        sec.ins_prm(
            &[Lbl::Len(16)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(64)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(128)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(256)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(512)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(1024)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(2048)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(4096)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(8192)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(16384)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32768)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(65536)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(131072)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
    }
    {
        let sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::D]);
        sec.ins_prm(
            &[Lbl::Len(16)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(64)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(128)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(256)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(512)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(1024)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(2048)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(4096)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(8192)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(16384)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32768)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(65536)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(131072)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
    }
    {
        let sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::E]);
        sec.ins_prm(
            &[Lbl::Len(16)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(64)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(64).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(128)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(128).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(256)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(256).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(512)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(512).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(1024)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(1024).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(2048)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(2048).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(4096)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(4096).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(8192)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(8192).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(16384)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(16384).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(32768)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(32768).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(65536)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(65536).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
        sec.ins_prm(
            &[Lbl::Len(131072)],
            |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(131072).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            },
        )?;
    }
    Ok(ret)
}
