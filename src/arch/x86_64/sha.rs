use super::intrin::*;
use crate::macros::delegate;
use crate::muck::cast;
use crate::ty::u32x4;

use super::Sha;

impl Sha {
    delegate! {
        fn sha1_msg1(a: { u32x4 }, b: { u32x4 }) -> { u32x4 } = _mm_sha1msg1_epu32;
        fn sha1_msg2(a: { u32x4 }, b: { u32x4 }) -> { u32x4 } = _mm_sha1msg2_epu32;
        fn sha1_next_e(a: { u32x4 }, b: { u32x4 }) -> { u32x4 } = _mm_sha1nexte_epu32;
        fn sha1_rounds4<const FUNC: i32>(a: { u32x4 }, b: { u32x4 }) -> { u32x4 } = _mm_sha1rnds4_epu32;

        fn sha256_msg1(a: { u32x4 }, b: { u32x4 }) -> { u32x4 } = _mm_sha256msg1_epu32;
        fn sha256_msg2(a: { u32x4 }, b: { u32x4 }) -> { u32x4 } = _mm_sha256msg2_epu32;
        fn sha256_rounds2(a: { u32x4 }, b: { u32x4 }, k: { u32x4 }) -> { u32x4 } = _mm_sha256rnds2_epu32;
    }
}
