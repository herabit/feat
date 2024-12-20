//! Module for SIMD Vector types.

/// Module for [`prim@f32`] vectors.
pub mod f32;
/// Module for [`prim@f64`] vectors.
pub mod f64;

/// Module for [`prim@i16`] vectors.
pub mod i16;
/// Module for [`prim@i32`] vectors.
pub mod i32;
/// Module for [`prim@i64`] vectors.
pub mod i64;
// Module for [`prim@i8`] vectors.
pub mod i8;

/// Module for [`prim@u16`] vectors.
pub mod u16;
/// Module for [`prim@u32`] vectors.
pub mod u32;
/// Module for [`prim@u64`] vectors.
pub mod u64;
// Module for [`prim@u8`] vectors.
pub mod u8;

pub(crate) mod exports {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::f32::*;
    #[doc(inline)]
    pub use super::f64::*;

    #[doc(inline)]
    pub use super::i16::*;
    #[doc(inline)]
    pub use super::i32::*;
    #[doc(inline)]
    pub use super::i64::*;
    #[doc(inline)]
    pub use super::i8::*;

    #[doc(inline)]
    pub use super::u16::*;
    #[doc(inline)]
    pub use super::u32::*;
    #[doc(inline)]
    pub use super::u64::*;
    #[doc(inline)]
    pub use super::u8::*;
}

#[doc(inline)]
pub use exports::*;
