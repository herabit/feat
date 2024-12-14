#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]

pub(crate) mod macros;
pub(crate) mod util;

/// Tools for comparing values.
pub mod cmp;
/// Types for mucking around with bytes.
pub mod muck;
/// Platform independent SIMD types.
pub mod types;
