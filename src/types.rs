mod lane;
mod mask;

#[doc(inline)]
pub use lane::*;
#[doc(inline)]
pub use mask::*;

mod sealed {
    use core::cmp::{Ordering, Reverse};
    use core::convert::Infallible;
    use core::mem::{ManuallyDrop, MaybeUninit};
    use core::num::{NonZero, Saturating, Wrapping};
    use core::ptr::NonNull;

    #[allow(dead_code)]
    pub trait Sealed {}

    macro_rules! seal {
        ($($ty:ty),* $(,)?) => {
            $(
                impl $crate::types::sealed::Sealed for $ty {}
            )*
        };
    }

    pub(super) use seal;

    seal!(bool, char, str);
    seal!(u8, u16, u32, u64, u128, usize);
    seal!(i8, i16, i32, i64, i128, isize);
    seal!(f32, f64);

    seal!(
        NonZero<u8>,
        NonZero<u16>,
        NonZero<u32>,
        NonZero<u64>,
        NonZero<u128>,
        NonZero<usize>
    );
    seal!(
        NonZero<i8>,
        NonZero<i16>,
        NonZero<i32>,
        NonZero<i64>,
        NonZero<i128>,
        NonZero<isize>
    );

    seal!(Ordering, Infallible);

    impl<T: Sealed, const N: usize> Sealed for [T; N] {}
    impl<T: Sealed> Sealed for [T] {}

    impl<T: ?Sized> Sealed for *const T {}
    impl<T: ?Sized> Sealed for *mut T {}

    impl<T: ?Sized> Sealed for &T {}
    impl<T: ?Sized> Sealed for &mut T {}

    impl<T: ?Sized> Sealed for NonNull<T> {}

    impl<T: Sealed> Sealed for Option<T> {}
    impl<T: Sealed, E: Sealed> Sealed for Result<T, E> {}

    impl<T: Sealed> Sealed for Wrapping<T> {}
    impl<T: Sealed> Sealed for Saturating<T> {}

    impl<T: Sealed> Sealed for Reverse<T> {}

    impl<T: Sealed> Sealed for MaybeUninit<T> {}
    impl<T: Sealed + ?Sized> Sealed for ManuallyDrop<T> {}
}
