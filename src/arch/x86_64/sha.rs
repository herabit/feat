use super::intrin::{
    _mm_sha1msg1_epu32, _mm_sha1msg2_epu32, _mm_sha1nexte_epu32, _mm_sha1rnds4_epu32,
    _mm_sha256msg1_epu32, _mm_sha256msg2_epu32, _mm_sha256rnds2_epu32,
};
use crate::muck::cast;
use crate::ty::u32x4;

use super::Sha;

impl Sha {
    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1msg1_epu32")]
    pub fn sha1_msg1(self, a: u32x4, b: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha1msg1_epu32(cast(a), cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1msg2_epu32")]
    pub fn sha1_msg2(self, a: u32x4, b: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha1msg2_epu32(cast(a), cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1nexte_epu32")]
    pub fn sha1_next_e(self, a: u32x4, b: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha1nexte_epu32(cast(a), cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha1rnds4_epu32")]
    pub fn sha1_rounds4<const FUNC: i32>(self, a: u32x4, b: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha1rnds4_epu32::<FUNC>(cast(a), cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha256msg1_epu32")]
    pub fn sha256_msg1(self, a: u32x4, b: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha256msg1_epu32(cast(a), cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha256msg2_epu32")]
    pub fn sha256_msg2(self, a: u32x4, b: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha256msg2_epu32(cast(a), cast(b)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_sha256rnds2_epu32")]
    pub fn sha256_rounds2(self, a: u32x4, b: u32x4, k: u32x4) -> u32x4 {
        cast(unsafe { _mm_sha256rnds2_epu32(cast(a), cast(b), cast(k)) })
    }
}
