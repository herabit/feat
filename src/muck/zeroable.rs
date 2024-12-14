use core::{cmp::Ordering, mem::MaybeUninit, num::NonZero, ptr::NonNull, slice};

use crate::macros::{impl_marker_arch, impl_marker_tuple};

use super::Forget;

/// Marker trait for types that can be safely initialized with zero bytes.
pub unsafe trait Zeroable: Sized + Forget {
    const ZEROED: Self = zeroed::<Self>();
}

#[inline(always)]
#[must_use]
pub const fn zeroed<T: Zeroable>() -> T {
    unsafe { core::mem::zeroed() }
}

#[inline(always)]
#[must_use]
pub const fn fill_zeros<T: Zeroable>(slice: &mut [T]) -> &mut [T] {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    unsafe { ptr.write_bytes(0, len) };

    slice
}

#[inline(always)]
#[must_use]
pub const fn init_zeros<T: Zeroable>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    let _ = fill_zeros(slice);

    // We filled the slice with zeros so we can safely assume it is initialized.
    unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
}

#[inline(always)]
#[must_use]
pub const fn write_zeroed<T: Zeroable>(target: &mut T) -> &mut T {
    &mut fill_zeros(slice::from_mut(target))[0]
}

#[inline(always)]
#[must_use]
pub const fn init_zeroed<T: Zeroable>(target: &mut MaybeUninit<T>) -> &mut T {
    &mut init_zeros(slice::from_mut(target))[0]
}

unsafe impl<T> Zeroable for *const T {}
unsafe impl<T> Zeroable for *mut T {}

unsafe impl<T> Zeroable for Option<NonNull<T>> {}
unsafe impl<T> Zeroable for Option<&T> {}
unsafe impl<T> Zeroable for Option<&mut T> {}

unsafe impl<T> Zeroable for MaybeUninit<T> {}

unsafe impl<T: Zeroable, const N: usize> Zeroable for [T; N] {}

macro_rules! zeroable {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Zeroable for $ty {}
        )*
    };
}

macro_rules! nonzero {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Zeroable for Option<NonZero<$ty>> {}
        )*
    };
}

zeroable!(u8, u16, u32, u64, u128, usize);
zeroable!(i8, i16, i32, i64, i128, isize);

nonzero!(u8, u16, u32, u64, u128, usize);
nonzero!(i8, i16, i32, i64, i128, isize);

zeroable!(f32, f64, char, bool);

zeroable!(Ordering);

impl_marker_tuple!(
    unsafe impl Zeroable
);

impl_marker_arch! {
    #[cfg(target_arch = "x86_64")]
    unsafe impl Zeroable for x86_64::{
        __m128,  __m256,  __m512,
        __m128d, __m256d, __m512d,
        __m128i, __m256i, __m512i,
    }
}

impl_marker_arch! {
    #[cfg(target_arch = "x86")]
    unsafe impl Zeroable for x86::{
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
    unsafe impl Zeroable for aarch64::{
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
