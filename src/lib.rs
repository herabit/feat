#![cfg_attr(not(test), no_std)]

/// Platform-specific functionality.
// pub mod arch;
/// Tools for mucking around with data.
pub mod muck;
/// SIMD types such as vectors.
pub mod ty;

/// Macros
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
