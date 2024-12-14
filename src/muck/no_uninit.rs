use core::ptr::NonNull;

use super::Forget;
use crate::macros::impl_marker_arch;
// use core::{cmp::Ordering, mem::MaybeUninit, num::NonZero, ptr::NonNull, slice};

/// Trait for types that contain no uninitialized or padding bytes.
///
/// This means that it is always safe to get the bytes of this type.
///
/// It does not however mean that it is valid for any bit representation.
pub unsafe trait NoUninit: Forget + Sized {}

unsafe impl<T> NoUninit for *const T {}
unsafe impl<T> NoUninit for *mut T {}
unsafe impl<T> NoUninit for NonNull<T> {}

unsafe impl<T> NoUninit for &T {}
unsafe impl<T> NoUninit for &mut T {}

unsafe impl<T> NoUninit for Option<NonNull<T>> {}
unsafe impl<T> NoUninit for Option<&T> {}
unsafe impl<T> NoUninit for Option<&mut T> {}

impl_marker_arch! {
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninit for x86_64::{
        __m128,  __m256,  __m512,
        __m128d, __m256d, __m512d,
        __m128i, __m256i, __m512i,
    }
}

impl_marker_arch! {
    #[cfg(target_arch = "x86")]
    unsafe impl NoUninit for x86::{
        __m128,  __m256,  __m512,
        __m128d, __m256d, __m512d,
        __m128i, __m256i, __m512i,
    }
}

impl_marker_arch! {
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
    ))]
    unsafe impl NoUninit for aarch64::{
        float32x2_t, float32x2x2_t, float32x2x3_t, float32x2x4_t,
        float32x4_t, float32x4x2_t, float32x4x3_t, float32x4x4_t,

        float64x1_t, float64x1x2_t, float64x1x3_t, float64x1x4_t,
        float64x2_t, float64x2x2_t, float64x2x3_t, float64x2x4_t,

        int8x8_t,    int8x8x2_t,    int8x8x3_t,    int8x8x4_t,
        int8x16_t,   int8x16x2_t,   int8x16x3_t,   int8x16x4_t,

        int16x4_t,   int16x4x2_t,   int16x4x3_t,   int16x4x4_t,
        int16x8_t,   int16x8x2_t,   int16x8x3_t,   int16x8x4_t,

        int32x2_t,   int32x2x2_t,   int32x2x3_t,   int32x2x4_t,
        int32x4_t,   int32x4x2_t,   int32x4x3_t,   int32x4x4_t,

        int64x1_t,   int64x1x2_t,   int64x1x3_t,   int64x1x4_t,
        int64x2_t,   int64x2x2_t,   int64x2x3_t,   int64x2x4_t,

        poly8x8_t,   poly8x8x2_t,   poly8x8x3_t,   poly8x8x4_t,
        poly8x16_t,  poly8x16x2_t,  poly8x16x3_t,  poly8x16x4_t,

        poly16x4_t,  poly16x4x2_t,  poly16x4x3_t,  poly16x4x4_t,
        poly16x8_t,  poly16x8x2_t,  poly16x8x3_t,  poly16x8x4_t,

        poly64x1_t,  poly64x1x2_t,  poly64x1x3_t,  poly64x1x4_t,
        poly64x2_t,  poly64x2x2_t,  poly64x2x3_t,  poly64x2x4_t,

        uint8x8_t,   uint8x8x2_t,   uint8x8x3_t,   uint8x8x4_t,
        uint8x16_t,  uint8x16x2_t,  uint8x16x3_t,  uint8x16x4_t,

        uint16x4_t,  uint16x4x2_t,  uint16x4x3_t,  uint16x4x4_t,
        uint16x8_t,  uint16x8x2_t,  uint16x8x3_t,  uint16x8x4_t,

        uint32x2_t,  uint32x2x2_t,  uint32x2x3_t,  uint32x2x4_t,
        uint32x4_t,  uint32x4x2_t,  uint32x4x3_t,  uint32x4x4_t,

        uint64x1_t,  uint64x1x2_t,  uint64x1x3_t,  uint64x1x4_t,
        uint64x2_t,  uint64x2x2_t,  uint64x2x3_t,  uint64x2x4_t,
    }
}
