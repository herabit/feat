#![allow(unused)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    #[doc(inline)]
    pub use crate::core_arch::x86::*;
    #[doc(inline)]
    pub use crate::core_arch::x86_64::*;
}

#[cfg(target_arch = "x86")]
pub mod x86 {
    #[doc(inline)]
    pub use crate::core_arch::x86::*;
}
