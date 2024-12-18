//! Module containing platform-nonspecific types such as vectors.
//!
//! # Vectors
//!
//! The vector types within this crate all have the following things in common:
//!
//! - For any given vector containing scalars of type `T`, and a length of `N`, its
//!   size will always be equal to that of `[T; N]`.
//!
//! - All vectors have a power-of-two length. So [`u8x1`], [`u8x64`] are defined
//!   but `u8x3` and `u8x66` will never be defined.
//!
//! - Vectors will use platform specific types as their internal representation when
//!   available. For example, [`u8x16`] uses `__m128i` on `x86` and `x86_64` targets.
//!
//!   - This means that the ABI for passing around vector types will depend on what
//!     cpu features are available. So when working with FFI, avoid passing by value.
//!
//!   - Since the internal representation of a given vector varies from platform, one
//!     must not make any assumptions about alignment. While size is guaranteed to be
//!     the same across platforms, alignment is not.
//!
//!     As such, utilize tools such as [`align_of`] to ensure
//!     that what you're doing is safe.

/// Module containing vector types.
pub mod vector;

#[doc(inline)]
#[allow(unused_imports)]
pub use vector::exports::*;
