#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]

/// -Module providing vendor-ish specific intrinsics.
mod core_arch;
mod macros;
mod util;

pub mod arch;
pub mod types;

pub trait Func {
    type Output;

    #[track_caller]
    fn call(self) -> Self::Output;
}

impl<R, F: FnOnce() -> R> Func for F {
    type Output = R;

    #[inline(always)]
    fn call(self) -> Self::Output {
        self()
    }
}
