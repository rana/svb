#![allow(dead_code)]
mod bens;
mod fns;
mod lbl;
use anyhow::Result;
use bens::*;
pub fn main() -> Result<()> {
    run_mtr_qrys()?;
    Ok(())
}
