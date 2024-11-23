#![cfg_attr(not(test), no_std)]

#[doc(inline)]
pub use bytemuck::{must_cast, must_cast_mut, must_cast_ref, must_cast_slice, must_cast_slice_mut};

pub mod arch;
pub mod ty;
