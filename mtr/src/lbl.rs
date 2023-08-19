use anyhow::{bail, Result};
use ben::{EnumStructVal, Label};
use std::fmt;

/// Benchmark labels.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Lbl {
    #[default]
    Scl,
    Enc,
    A,
    B,
    C,
    D,
    E,
    Len(u32),
}
impl fmt::Display for Lbl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Lbl::Scl => write!(f, "scl"),
            Lbl::Enc => write!(f, "enc"),
            Lbl::A => write!(f, "a"),
            Lbl::B => write!(f, "b"),
            Lbl::C => write!(f, "c"),
            Lbl::D => write!(f, "d"),
            Lbl::E => write!(f, "e"),
            Lbl::Len(x) => {
                if f.alternate() { write!(f, "len") } else { write!(f, "len({})", x) }
            }
        }
    }
}
impl EnumStructVal for Lbl {
    fn val(&self) -> Result<u32> {
        match *self {
            Lbl::Len(x) => Ok(x),
            _ => bail!("label '{}' isn't a struct enum", self),
        }
    }
}
impl Label for Lbl {}
