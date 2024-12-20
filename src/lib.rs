#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]

/// Module providing vendor-ish specific intrinsics.
mod core_arch;
mod macros;
mod util;

pub mod arch;
// pub mod cmp;
pub mod types;
