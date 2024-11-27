#![cfg_attr(not(test), no_std)]

#[doc(inline)]
pub use bytemuck::{must_cast, must_cast_mut, must_cast_ref, must_cast_slice, must_cast_slice_mut};

pub mod arch;
pub mod ty;

mod macros;

/// Essentially [`FnOnce`] that anyone can implement.
pub trait CallOnce {
    type Output;

    #[track_caller]
    fn call_once(self) -> Self::Output;
}

impl<U, T: FnOnce() -> U> CallOnce for T {
    type Output = U;

    #[inline(always)]
    fn call_once(self) -> Self::Output {
        self()
    }
}
