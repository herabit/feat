#[allow(dead_code)]
pub trait Sealed {}

macro_rules! seal {
    ($($ty:ty),* $(,)?) => {
        $(impl $crate::muck::sealed::Sealed for $ty {})*
    };
}

pub(crate) use seal;

use core::mem::MaybeUninit;
use core::num::NonZero;
use core::ptr::NonNull;

seal!(u8, u16, u32, u64, u128, usize);
seal!(
    NonZero<u8>,
    NonZero<u16>,
    NonZero<u32>,
    NonZero<u64>,
    NonZero<u128>,
    NonZero<usize>
);
seal!(i8, i16, i32, i64, i128, isize);
seal!(
    NonZero<i8>,
    NonZero<i16>,
    NonZero<i32>,
    NonZero<i64>,
    NonZero<i128>,
    NonZero<isize>
);
seal!(f32, f64);
seal!(bool, char);
seal!(core::cmp::Ordering);

impl<T: ?Sized> Sealed for *const T {}
impl<T: ?Sized> Sealed for *mut T {}
impl<T: ?Sized> Sealed for NonNull<T> {}

impl<T: ?Sized> Sealed for &mut T {}
impl<T: ?Sized> Sealed for &T {}

impl Sealed for () {}

impl<T: Sealed> Sealed for Option<T> {}

impl<T> Sealed for MaybeUninit<T> {}

impl<T: Sealed> Sealed for [T] {}
impl<T: Sealed, const N: usize> Sealed for [T; N] {}
