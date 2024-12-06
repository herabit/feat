#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]

pub mod arch;
pub mod simd;

mod macros;
mod util;

pub(crate) mod sealed;
