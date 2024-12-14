#[doc(inline)]
pub use core::cmp::Ordering;

/// Trait for types that can be compared in const contexts.
pub trait Ordered: 'static + Send + Sync + Copy + Ord + sealed::Sealed {
    /// The lower ***inclusive*** bound of this type.
    const MIN: Self;
    /// The upper ***inclusive*** bound of this type.
    const MAX: Self;

    /// Compare two [`Ordered`] values.
    ///
    /// This just calls [`compare`].
    #[inline(always)]
    #[must_use]
    #[track_caller]
    fn compare(self, rhs: Self) -> Ordering {
        compare(self, rhs)
    }
}

/// Compare two [`Ordered`] values at compile time.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn compare<T: Ordered>(lhs: T, rhs: T) -> Ordering {
    const {
        let (min, max) = (T::MIN, T::MAX);

        assert!(_compare(min, max).is_le(), "MIN > MAX");
        assert!(_compare(max, min).is_ge(), "MAX < MIN");
    };

    _compare(lhs, rhs)
}

/// Determine if `lhs == rhs`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_eq<T: Ordered>(lhs: T, rhs: T) -> bool {
    compare(lhs, rhs).is_eq()
}

/// Determine if `lhs != rhs`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_ne<T: Ordered>(lhs: T, rhs: T) -> bool {
    compare(lhs, rhs).is_ne()
}

/// Determine if `lhs > rhs`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_gt<T: Ordered>(lhs: T, rhs: T) -> bool {
    compare(lhs, rhs).is_gt()
}

/// Determine if `lhs >= rhs`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_ge<T: Ordered>(lhs: T, rhs: T) -> bool {
    compare(lhs, rhs).is_ge()
}

/// Determine if `lhs < rhs`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_lt<T: Ordered>(lhs: T, rhs: T) -> bool {
    compare(lhs, rhs).is_lt()
}

/// Determine if `lhs <= rhs`.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn is_le<T: Ordered>(lhs: T, rhs: T) -> bool {
    compare(lhs, rhs).is_le()
}

#[inline(always)]
#[must_use]
#[track_caller]
const fn _compare<T: Ordered>(lhs: T, rhs: T) -> Ordering {
    use sealed::Wit;

    macro_rules! cmp {
        ($lhs:expr, $rhs:expr) => {{
            let (lhs, rhs) = ($lhs, $rhs);

            match ((lhs > rhs) as i8) - ((lhs < rhs) as i8) {
                -1 => Ordering::Less,
                0 => Ordering::Equal,
                1 => Ordering::Greater,
                _ => unreachable!(),
            }
        }};
    }

    match T::WITNESS {
        Wit::Bool(type_eq) => {
            let (lhs, rhs) = (type_eq.to_right(lhs), type_eq.to_right(rhs));

            match (lhs as i8) - (rhs as i8) {
                -1 => Ordering::Less,
                0 => Ordering::Equal,
                1 => Ordering::Greater,
                _ => unreachable!(),
            }
        }
        Wit::Char(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        // There is probably a more optimized way of doing this.
        Wit::Ordering(type_eq) => {
            cmp!(type_eq.to_right(lhs) as i8, type_eq.to_right(rhs) as i8)
        }
        Wit::U8(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::U16(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::U32(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::U64(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::U128(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::Usize(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),

        Wit::I8(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::I16(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::I32(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::I64(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::I128(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
        Wit::Isize(type_eq) => cmp!(type_eq.to_right(lhs), type_eq.to_right(rhs)),
    }
}

mod sealed {
    use crate::util::TypeEq;
    use core::cmp::Ordering;

    /// Type witness for types that can implement [`Ord`] in const.
    #[non_exhaustive]
    pub enum Wit<T> {
        Bool(TypeEq<T, bool>),
        Char(TypeEq<T, char>),
        Ordering(TypeEq<T, Ordering>),

        U8(TypeEq<T, u8>),
        U16(TypeEq<T, u16>),
        U32(TypeEq<T, u32>),
        U64(TypeEq<T, u64>),
        U128(TypeEq<T, u128>),
        Usize(TypeEq<T, usize>),

        I8(TypeEq<T, i8>),
        I16(TypeEq<T, i16>),
        I32(TypeEq<T, i32>),
        I64(TypeEq<T, i64>),
        I128(TypeEq<T, i128>),
        Isize(TypeEq<T, isize>),
    }

    impl<T> Copy for Wit<T> {}
    impl<T> Clone for Wit<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    pub trait Sealed: Sized + Copy + Send + Sync + Ord + 'static {
        /// Type witness to observe what type this is.
        const WITNESS: Wit<Self>;
    }

    macro_rules! ordered {
        ($(
            $ty:ty {
                wit: $wit:ident,
                min: $min:expr,
                max: $max:expr
                $(,)?
            }
        ),* $(,)?) => {
            $(
                impl Sealed for $ty {
                    const WITNESS: Wit<$ty> = Wit::$wit(TypeEq::new());
                }

                impl super::Ordered for $ty {
                    const MIN: $ty = $min;
                    const MAX: $ty = $max;
                }
            )*
        };
    }

    ordered! {
        bool {
            wit: Bool,
            min: false,
            max: true,
        },

        char {
            wit: Char,
            min: char::MIN,
            max: char::MAX,
        },

        Ordering {
            wit: Ordering,
            min: Ordering::Less,
            max: Ordering::Greater,
        },

        u8 {
            wit: U8,
            min: u8::MIN,
            max: u8::MAX,
        },


        u16 {
            wit: U16,
            min: u16::MIN,
            max: u16::MAX,
        },


        u32 {
            wit: U32,
            min: u32::MIN,
            max: u32::MAX,
        },


        u64 {
            wit: U64,
            min: u64::MIN,
            max: u64::MAX,
        },


        u128 {
            wit: U128,
            min: u128::MIN,
            max: u128::MAX,
        },

        usize {
            wit: Usize,
            min: usize::MIN,
            max: usize::MAX,
        },


        i8 {
            wit: I8,
            min: i8::MIN,
            max: i8::MAX,
        },


        i16 {
            wit: I16,
            min: i16::MIN,
            max: i16::MAX,
        },


        i32 {
            wit: I32,
            min: i32::MIN,
            max: i32::MAX,
        },


        i64 {
            wit: I64,
            min: i64::MIN,
            max: i64::MAX,
        },


        i128 {
            wit: I128,
            min: i128::MIN,
            max: i128::MAX,
        },

        isize {
            wit: Isize,
            min: isize::MIN,
            max: isize::MAX,
        },
    }
}
