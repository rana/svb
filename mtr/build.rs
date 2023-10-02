use proc_macro2::{Literal, TokenStream};
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

    Ok(())
}

/// Writes a token stream to a file.
pub fn write_one_fle(fle_stm: TokenStream, fle_pth: &PathBuf) -> std::io::Result<()> {
    let fle = syn::parse_file(fle_stm.to_string().as_str()).unwrap();
    let fmt = prettyplease::unparse(&fle);
    fs::write(fle_pth, fmt)
}

/// Emits a token stream for the `main` imports.
pub fn emit_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
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
    });

    stm
}

/// Emits a token stream for the main file.
pub fn emit_main_fle() -> TokenStream {
    let tok_fns = [emit_imports, emit_main_fn, emit_new_stdy];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

pub fn emit_main_fn() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
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
    });

    stm
}

pub fn emit_new_stdy() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        /// Returns a study with benchmark functions ready to be run.
        pub fn new_stdy() -> Result<Stdy<Lbl>>
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let tok_bens = [
        emit_bens_scl_enc_a,
        emit_bens_scl_enc_b,
        emit_bens_scl_enc_c,
        emit_bens_scl_enc_d,
        emit_bens_scl_enc_e,
        emit_bens_scl_enc_f,
        emit_bens_scl_enc_g,
        emit_bens_scl_dat_a,
        emit_bens_scl_dat_e,
        emit_bens_scl_dat_g,
    ];
    tok_bens
        .iter()
        .for_each(|tok_ben| stm_inr.extend(tok_ben()));

    // fn: end
    stm.extend(quote! {
        {
            let mut stdy = Stdy::new();
            #stm_inr
            Ok(stdy)
        }
    });

    stm
}

pub static RNG: Range<u32> = 4..18;

pub fn emit_bens_scl_enc_a() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_a::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, A], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_enc_b() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_b::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, B], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_enc_c() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_c::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, C], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_enc_d() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_d::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, D], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_enc_e() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_e::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, E], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_enc_f() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_f::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, F], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_enc_g() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_g::enc(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Enc, G], |x| {
            #stm_inr
        });
    });

    stm
}


pub fn emit_bens_scl_dat_a() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_a::dat_byt_len(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Dat, A], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_dat_e() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_e::dat_byt_len(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Dat, E], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_bens_scl_dat_g() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
                let vals: Vec<u32> = rnds_eql_byt().take(#lit_len).collect();
                tme.borrow_mut().start();
                let ret = scl_g::dat_byt_len(&vals);
                tme.borrow_mut().stop();
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        stdy.reg_bld(&[Scl, Dat, G], |x| {
            #stm_inr
        });
    });

    stm
}
