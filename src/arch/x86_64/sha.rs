use bytemuck::must_cast;

use super::intrin::{
    _mm_sha1msg1_epu32, _mm_sha1msg2_epu32, _mm_sha1nexte_epu32, _mm_sha1rnds4_epu32,
    _mm_sha256msg1_epu32, _mm_sha256msg2_epu32, _mm_sha256rnds2_epu32,
};
use crate::ty::u32x4;

/// Type level proof of the existence of SHA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Sha(());

impl Sha {
    #[inline(always)]
    #[must_use]
    pub const unsafe fn new_unchecked() -> Sha {
        Sha(())
    }

    #[inline]
    #[target_feature(enable = "sha")]
    pub unsafe fn execute<T, F: FnOnce() -> T>(f: F) -> T {
        f()
    }

    #[inline(always)]
    pub fn run<T, F: FnOnce() -> T>(self, f: F) -> T {
        // SAFETY: We know the SHA features exist.
        unsafe { Sha::execute(f) }
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1msg1_epu32")]
    pub fn sha1_msg1(self, a: u32x4, b: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha1msg1_epu32(must_cast(a), must_cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1msg2_epu32")]
    pub fn sha1_msg2(self, a: u32x4, b: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha1msg2_epu32(must_cast(a), must_cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1nexte_epu32")]
    pub fn sha1_next_e(self, a: u32x4, b: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha1nexte_epu32(must_cast(a), must_cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1rnds4_epu32")]
    pub fn sha1_rounds4<const FUNC: i32>(self, a: u32x4, b: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha1rnds4_epu32::<FUNC>(must_cast(a), must_cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha256msg1_epu32")]
    pub fn sha256_msg1(self, a: u32x4, b: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha256msg1_epu32(must_cast(a), must_cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha256msg2_epu32")]
    pub fn sha256_msg2(self, a: u32x4, b: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha256msg2_epu32(must_cast(a), must_cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha256rnds2_epu32")]
    pub fn sha256_rounds2(self, a: u32x4, b: u32x4, k: u32x4) -> u32x4 {
        must_cast(unsafe { _mm_sha256rnds2_epu32(must_cast(a), must_cast(b), must_cast(k)) })
    }
}
