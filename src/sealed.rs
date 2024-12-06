#![allow(dead_code)]
use crate::macros::{impl_marker_arch, seal, seal_nonzero, seal_tuple};

/// Trait for sealing types.
pub trait Sealed {}

seal!(u8, u16, u32, u64, u128, usize);
seal!(i8, i16, i32, i64, i128, isize);
seal!(f32, f64);
seal!(bool);
seal!(char);
seal!(str);

seal_nonzero!(u8, u16, u32, u64, u128, usize);
seal_nonzero!(i8, i16, i32, i64, i128, isize);

impl<T: Sealed> Sealed for [T] {}
impl<T: Sealed, const N: usize> Sealed for [T; N] {}

impl<T: Sealed> Sealed for Option<T> {}
impl<T: Sealed, E: Sealed> Sealed for Result<T, E> {}

impl<T: Sealed + ?Sized> Sealed for &T {}
impl<T: Sealed + ?Sized> Sealed for &mut T {}

impl<T: Sealed + ?Sized> Sealed for *const T {}
impl<T: Sealed + ?Sized> Sealed for *mut T {}
impl<T: Sealed + ?Sized> Sealed for core::ptr::NonNull<T> {}

impl<T: Sealed> Sealed for core::num::Wrapping<T> {}
impl<T: Sealed> Sealed for core::num::Saturating<T> {}

impl<T: Sealed> Sealed for core::cmp::Reverse<T> {}
seal!(core::cmp::Ordering);

impl<T: Sealed + ?Sized> Sealed for core::mem::ManuallyDrop<T> {}
impl<T: Sealed> Sealed for core::mem::MaybeUninit<T> {}

impl<T: Sealed + ?Sized> Sealed for core::marker::PhantomData<T> {}
seal!(core::marker::PhantomPinned);

seal!(core::convert::Infallible);

seal_tuple!();
seal_tuple!(T0);
seal_tuple!(T0, T1);
seal_tuple!(T0, T1, T2);
seal_tuple!(T0, T1, T2, T3);
seal_tuple!(T0, T1, T2, T3, T4);
seal_tuple!(T0, T1, T2, T3, T4, T5);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
seal_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);

impl_marker_arch!(
    #[cfg(target_arch = "x86_64")]
    impl Sealed for x86_64::{
        __m128,
        __m128d,
        __m128i,
        __m256,
        __m256d,
        __m256i,
        __m512,
        __m512d,
        __m512i,
    }
);

impl_marker_arch!(
    #[cfg(target_arch = "x86")]
    impl Sealed for x86::{
        __m128,
        __m128d,
        __m128i,
        __m256,
        __m256d,
        __m256i,
        __m512,
        __m512d,
        __m512i,
    }
);

use crate::simd::{m16, m32, m64, m8, msize};
seal!(m8, m16, m32, m64, msize);
