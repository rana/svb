use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

/// Runs the build script.
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    write_all_files("./src/")
}

/// Writes all files to a directory.
pub fn write_all_files(dir: &str) -> std::io::Result<()> {
    let pth = Path::new(dir);
    fs::create_dir_all(pth)?;

    write_one_fle(emit_main_fle(), &pth.join("main.rs"))?;
    write_one_fle(emit_bens_fle(), &pth.join("bens.rs"))?;

    Ok(())
}

/// Writes a token stream to a file.
pub fn write_one_fle(fle_stm: TokenStream, fle_pth: &PathBuf) -> std::io::Result<()> {
    let fle = syn::parse_file(fle_stm.to_string().as_str()).unwrap();
    let fmt = prettyplease::unparse(&fle);
    fs::write(fle_pth, fmt)
}

/// Emits a token stream for the main file.
pub fn emit_main_fle() -> TokenStream {
    let tok_fns = [emit_main_imports, emit_main_fn];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for the `main` imports.
pub fn emit_main_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(dead_code)]
        mod bens;
        mod fns;
        mod lbl;
        use anyhow::Result;
        use bens::*;
    });

    stm
}

/// Emits a token stream for the `main` function.
pub fn emit_main_fn() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {

        pub fn main() -> Result<()> {
            run_mtr_qrys()?;
            Ok(())
        }

    });

    stm
}

pub fn emit_bens_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        use anyhow::Result;
        use ben::*;
        use itr::*;
        use crate::lbl::Lbl;
        use crate::fns::scl_a;
        use crate::fns::scl_b;
        use crate::fns::scl_c;
        use crate::fns::scl_d;
    });

    stm
}

pub fn emit_bens_fle() -> TokenStream {
    let tok_fns = [
        emit_bens_imports,
        emit_bens_run_mtr_qrys,
        emit_bens_new_mtr_set,
    ];
    let ret = tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    });

    ret
}

pub fn emit_bens_run_mtr_qrys() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        /// Run analysis on benchmark functions.
        pub fn run_mtr_qrys() -> Result<()> {
            let set = new_mtr_set()?;
            let itr: u32 = 64;

            // // Scalar implementation: A vs B
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::B]],
            //     grp: Some(vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::B]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Scalar implementation: A vs C
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::C]],
            //     grp: Some(vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::C]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Scalar implementation: B vs C
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Scl, Lbl::Enc, Lbl::B], vec![Lbl::Scl, Lbl::Enc, Lbl::C]],
            //     grp: Some(vec![vec![Lbl::Scl, Lbl::Enc, Lbl::B], vec![Lbl::Scl, Lbl::Enc, Lbl::C]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // Scalar implementation: A vs D
            set.qry(Qry{
                frm: vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::D]],
                grp: Some(vec![vec![Lbl::Scl, Lbl::Enc, Lbl::A], vec![Lbl::Scl, Lbl::Enc, Lbl::D]]),
                srt: Some(Lbl::Len(0)),
                sta: Some(Sta::Mdn),
                trn: Some(Lbl::Len(0)),
                cmp: true,
                itr,
            })?;

            Ok(())
        }
    });

    stm
}

pub fn emit_bens_new_mtr_set() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        /// Returns a populated set of benchmark functions.
        pub fn new_mtr_set() -> Result<Set<Lbl>>
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let tok_bens = [
        emit_bens_scl_a_enc,
        emit_bens_scl_b_enc,
        emit_bens_scl_c_enc,
        emit_bens_scl_d_enc,
    ];
    tok_bens
        .iter()
        .for_each(|tok_ben| stm_inr.extend(tok_ben()));

    // fn: end
    stm.extend(quote! {
        {
            let ret = Set::new();
            #stm_inr
            Ok(ret)
        }
    });

    stm
}

pub static RNG: Range<u32> = 4..18;

pub fn emit_bens_scl_a_enc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::A]);
    });
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

pub fn emit_bens_scl_b_enc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::B]);
    });
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

pub fn emit_bens_scl_c_enc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::C]);
    });
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}


pub fn emit_bens_scl_d_enc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Scl, Lbl::Enc, Lbl::D]);
    });
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}
