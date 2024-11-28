//! Types that are generally used across platforms with intrinsics.

/// Mask types.
pub mod mask;
/// Vector types.
pub mod vec;

#[doc(inline)]
pub use mask::*;
#[doc(inline)]
pub use vec::*;
