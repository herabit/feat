use super::{sealed, transmute, Forget};
use crate::cmp::*;

use core::num::NonZero;

/// Marker trait for types that are represented in memory as some integer in some contigous range of values.
///
/// # Safety
///
/// Implementors must:
///
/// - Have a nonzero size.
///
/// - Be inhabited.
///
/// - Have the same size as `Self::Int`.
///
/// - Have the same alignment as `Self::Int`.
///
/// - Have no gaps between `Self::MIN..=Self::MAX`.
///
/// - Ensure that `Self::MIN <= Self::MAX`.
///
/// - All values in the range `Self::MIN..=Self::MAX` have
///   exactly one interpretation as a `Self`.
///
/// - All values of `Self` can be represented as a `Self::Int`.
pub unsafe trait Contiguous: sealed::Sealed + Copy + Send + Sync + 'static + Forget {
    /// This ***must*** be some integer primitive, it is considered
    /// undefined behavior for this to be otherwise.
    type Int: Contiguous<Int = Self::Int> + Ordered;

    /// The lower ***inclusive*** bound of this type.
    const MIN: Self::Int;
    /// The upper ***inclusive*** bound of this type.
    const MAX: Self::Int;

    #[doc(hidden)]
    const SANITY_CHECK: () = {
        assert!(
            size_of::<Self>() == size_of::<Self::Int>(),
            "size of type doesn't match integer"
        );
        assert!(
            align_of::<Self>() == align_of::<Self::Int>(),
            "alignment of type doesn't match integer"
        );

        assert!(compare(Self::MIN, Self::MAX).is_le(), "MIN > MAX");
        assert!(compare(Self::MAX, Self::MIN).is_ge(), "MAX < MIN");
    };

    /// Convert an integer to a `T` if it lies within the acceptable range
    /// of values for `T`.
    ///
    /// This just calls [`from_int`].
    #[inline(always)]
    #[must_use]
    #[track_caller]
    fn from_int(int: Self::Int) -> Option<Self> {
        from_int(int)
    }

    /// Convert a `T` into its underlying integer type.
    ///
    /// This just calls [`to_int`]
    #[inline(always)]
    #[must_use]
    #[track_caller]
    fn to_int(self) -> Self::Int {
        to_int(self)
    }
}

/// Returns whether a given integer has a valid bit pattern for a `T`
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_valid<T: Contiguous>(int: T::Int) -> bool {
    const { T::SANITY_CHECK };

    // (T::MIN <= int) && (T::MAX >= int)
    is_le(T::MIN, int) & is_ge(T::MAX, int)
}

/// Convert an integer to a `T` if it lies within the acceptable range
/// of values for `T`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn from_int<T: Contiguous>(int: T::Int) -> Option<T> {
    const { T::SANITY_CHECK };

    if is_valid::<T>(int) {
        // SAFETY: We know that the bit pattern is a valid `T`.
        Some(unsafe { transmute(int) })
    } else {
        None
    }
}

/// Convert a reference to an integer to a `&T` if it lies within the
/// acceptable range of values for `T`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn from_ref<T: Contiguous>(int: &T::Int) -> Option<&T> {
    const { T::SANITY_CHECK };

    if is_valid::<T>(*int) {
        // SAFETY: We know that the bit pattern is a valid `T`.
        Some(unsafe { &*(int as *const T::Int as *const T) })
    } else {
        None
    }
}

/// Convert a mutable reference to an integer to a `&mut T` if it lies within the
/// acceptable range of values for `T`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn from_mut<T: Contiguous>(int: &mut T::Int) -> Option<&mut T> {
    const { T::SANITY_CHECK };

    if is_valid::<T>(*int) {
        // SAFETY: We know that the bit pattern is a valid `T`.
        //
        //        Additionally we know that all values for `T`
        //        are valid for `T::Int`.
        Some(unsafe { &mut *(int as *mut T::Int as *mut T) })
    } else {
        None
    }
}

/// Convert a `T` into its underlying integer type.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn to_int<T: Contiguous>(value: T) -> T::Int {
    const { T::SANITY_CHECK };

    // SAFETY: We know that the bit pattern is valid for `T::Int`.
    unsafe { transmute(value) }
}

/// Convert a reference to `T` to a reference to its underlying integer type.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn as_int<T: Contiguous>(value: &T) -> &T::Int {
    const { T::SANITY_CHECK };

    // SAFETY: We know that the bit pattern is valid for `T::Int`.
    unsafe { &*(value as *const T as *const T::Int) }
}

/// Convert a mutable reference to a `T` to a mutable reference to its underlying integer type.
///
/// # Safety
///
/// The caller must ensure that the underlying integer is a valid `T` before the borrow ends.
///
/// Failing to do so is undefined behavior.
#[inline(always)]
#[must_use]
#[track_caller]
pub const unsafe fn as_int_mut<T: Contiguous>(value: &mut T) -> &mut T::Int {
    const { T::SANITY_CHECK };

    // SAFETY: The caller ensures that this is safe.
    unsafe { &mut *(value as *mut T as *mut T::Int) }
}

macro_rules! contiguous {
    (@int $($ty:ty),*) => {
        $(
            unsafe impl Contiguous for $ty {
                type Int = $ty;

                const MIN: $ty = <$ty>::MIN;
                const MAX: $ty = <$ty>::MAX;
            }

            const _: () = <$ty as Contiguous>::SANITY_CHECK;
        )*
    };

    (@uint $($ty:ty),*) => {
        contiguous!(@int $($ty),*);

        $(
            unsafe impl Contiguous for NonZero<$ty> {
                type Int = $ty;

                const MIN: $ty = NonZero::<$ty>::MIN.get();
                const MAX: $ty = NonZero::<$ty>::MAX.get();
            }

            const _: () = <NonZero<$ty> as Contiguous>::SANITY_CHECK;
        )*
    };
}

contiguous!(@uint u8, u16, u32, u64, u128, usize);
contiguous!(@int i8, i16, i32, i64, i128, isize);

unsafe impl Contiguous for bool {
    type Int = u8;

    const MIN: u8 = false as u8;
    const MAX: u8 = true as u8;
}

unsafe impl Contiguous for Ordering {
    type Int = i8;

    const MIN: i8 = Ordering::Less as i8;
    const MAX: i8 = Ordering::Greater as i8;
}

const _: () = <bool as Contiguous>::SANITY_CHECK;
const _: () = <Ordering as Contiguous>::SANITY_CHECK;
