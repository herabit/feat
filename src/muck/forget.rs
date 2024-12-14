use core::{
    cmp::{Ordering, Reverse},
    convert::Infallible,
    marker::{PhantomData, PhantomPinned},
    mem::{ManuallyDrop, MaybeUninit},
    num::{NonZero, Saturating, Wrapping},
    ptr::NonNull,
};

/// Marker trait for types that do not need to be dropped.
///
/// A type implementing this indicates that it can be forgotten without
/// any side effects.
///
/// This trait is safe because Rust does not guarantee that destructors will run.
pub trait Forget {
    /// This ***should*** be false, but may not be.
    ///
    /// This is because it relies upon [`core::mem::needs_drop`],
    /// which may return `true` for types that don't actually
    /// need to be dropped.
    const NEEDS_DROP: bool = core::mem::needs_drop::<Self>();
}

/// Forget a type that is forgettable.
#[inline(always)]
#[must_use]
pub const fn forget<T: Forget>(value: T) {
    core::mem::forget(value)
}

impl<T: Forget> Forget for [T] {}
impl<T: Forget, const N: usize> Forget for [T; N] {}
impl<T: ?Sized> Forget for *const T {}
impl<T: ?Sized> Forget for *mut T {}
impl<T: ?Sized> Forget for NonNull<T> {}

impl<T: ?Sized> Forget for &T {}
impl<T: ?Sized> Forget for &mut T {}

impl<T: Forget> Forget for Option<T> {}
impl<T: Forget, E: Forget> Forget for Result<T, E> {}

impl<T: Forget + ?Sized> Forget for PhantomData<T> {}

impl<T: Forget> Forget for Reverse<T> {}
impl<T: Forget> Forget for Saturating<T> {}
impl<T: Forget> Forget for Wrapping<T> {}

impl<T: Forget + ?Sized> Forget for ManuallyDrop<T> {}

// For the sake of simplicity we assume a `MaybeUninit` is uninitialized,
// as `MaybeUninit` has no destructor.
impl<T> Forget for MaybeUninit<T> {}

macro_rules! forget {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Forget for $ty {}
        )*
    };
}

forget!(u8, u16, u32, u64, u128, usize);
forget!(i8, i16, i32, i64, i128, isize);
forget!(
    NonZero<u8>,
    NonZero<u16>,
    NonZero<u32>,
    NonZero<u64>,
    NonZero<u128>,
    NonZero<usize>
);
forget!(
    NonZero<i8>,
    NonZero<i16>,
    NonZero<i32>,
    NonZero<i64>,
    NonZero<i128>,
    NonZero<isize>
);
forget!(f32, f64);
forget!(char, bool, str);
forget!(Ordering, Infallible, PhantomPinned);

use crate::macros::{impl_marker_arch, impl_marker_tuple};

impl_marker_tuple!(impl Forget);

impl_marker_arch! {
    #[cfg(target_arch = "x86_64")]
    impl Forget for x86_64::{
        __m128,  __m256,  __m512,
        __m128d, __m256d, __m512d,
        __m128i, __m256i, __m512i,

        CpuidResult,
    }
}

impl_marker_arch! {
    #[cfg(target_arch = "x86")]
    impl Forget for x86::{
        __m128,  __m256,  __m512,
        __m128d, __m256d, __m512d,
        __m128i, __m256i, __m512i,

        CpuidResult,
    }
}

impl_marker_arch! {
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
    ))]
    impl Forget for aarch64::{
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
