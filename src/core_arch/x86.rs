#[cfg(target_arch = "x86_64")]
#[doc(inline)]
pub(crate) use ::core::arch::x86_64 as raw;

#[cfg(target_arch = "x86")]
#[doc(inline)]
pub(crate) use ::core::arch::x86 as raw;

mod sse;
pub use sse::*;

pub mod float;
