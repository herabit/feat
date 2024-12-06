use core::{
    cmp::Ordering,
    hash::Hash,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use super::Lane;

/// Marker trait for mask types.
pub unsafe trait Mask:
    Lane<Mask = Self>
    + Not<Output = Self>
    + BitOr<Self, Output = Self>
    + BitAnd<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Eq
    + Ord
    + Hash
{
    /// All bits are set.
    const TRUE: Self;
    /// No bits are set.
    const FALSE: Self;

    /// All bits are set.
    const SET: Self = Self::TRUE;
    /// No bits are set.
    const UNSET: Self = Self::FALSE;

    /// All bits are set.
    const ALL: Self = Self::TRUE;
    /// No bits are set.
    const NONE: Self = Self::FALSE;
}

macro_rules! define_mask {
    (
        $($name:ident(
            $signed:ident,
            $unsigned:ident,
            $size:tt $(,)?
        )
    ),*
        $(,)?
    ) => {
        $(
            #[doc = concat!(
                "A ", $size, " mask type that is \
                 either all ones, or all zeros."
            )]
            ///
            /// It is essentially a [`bool`] with a special representation
            /// that makes it useful for masking things.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            #[repr($signed)]
            pub enum $name {
                /// All bits are set.
                True = -1,
                /// No bits are set.
                #[default]
                False = 0,
            }

            impl $name {
                /// How many bits are needed to store this mask.
                pub const BITS: u32 = size_of::<$name>() as u32 * 8;

                /// Create a mask from it's primitive representation.
                #[inline(always)]
                #[must_use]
                pub const fn from_repr(repr: $signed) -> Option<$name> {
                    match repr {
                        -1 => Some($name::True),
                        0 => Some($name::False),
                        _ => None,
                    }
                }

                /// Create a mask from a boolean.
                #[inline(always)]
                #[must_use]
                pub const fn from_bool(is_set: bool) -> $name {
                    match is_set {
                        true => $name::True,
                        false => $name::False,
                    }
                }

                /// Return whether this mask has all bits set.
                #[doc(alias = "to_bool")]
                #[inline(always)]
                #[must_use]
                pub const fn is_set(self) -> bool {
                    matches!(self, $name::True)
                }

                /// Return whether this mask has no bits set.
                #[inline(always)]
                #[must_use]
                pub const fn is_unset(self) -> bool {
                    !self.is_set()
                }

                /// Return this mask as a boolean.
                ///
                /// # Returns
                ///
                /// - `true`: All bits are set.
                ///
                /// - `false`: No bits are set.
                #[doc(alias = "is_set")]
                #[inline(always)]
                #[must_use]
                pub const fn to_bool(self) -> bool {
                    self.is_set()
                }

                /// Compute the bitwise and logical `NOT`/`!` of this mask.
                #[inline(always)]
                pub const fn not(self) -> $name {
                    // TODO: Determine whether doing transmutes like the other
                    //       bitwise operations is faster than this.
                    //
                    // TODO: Determine whether implementing this in terms of XOR is faster.
                    //
                    // NOTE: Both todos arise from me not being sure if this being a match
                    //       can fuck up the optimizer at all in certain scenarios.
                    //
                    //       Right now it seems to be fine.
                    match self {
                        $name::True => $name::False,
                        $name::False => $name::True,
                    }
                }

                /// Compute the bitwise and logical `AND`/`&` between two masks.
                #[inline(always)]
                pub const fn and(self, other: $name) -> $name {
                    // SAFETY: `self` and `other` always have all their bits set or none.
                    //
                    //         Therefore the result of bitwise boolean operations (NOT, AND, OR, XOR, and so on..)
                    //         always results in all or no bits being set.
                    //
                    // FIXME:  Implement this in terms of matching on the mask type
                    //         when it optimizes properly.
                    match $name::from_repr((self as $signed) & (other as $signed)) {
                        Some(mask) => mask,
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }

                /// Compute the bitwise and logical `OR`/`|` between two masks.
                #[inline(always)]
                pub const fn or(self, other: $name) -> $name {
                    // SAFETY: `self` and `other` always have all their bits set or none.
                    //
                    //         Therefore the result of bitwise boolean operations (NOT, AND, OR, XOR, and so on..)
                    //         always results in all or no bits being set.
                    //
                    // FIXME:  Implement this in terms of matching on the mask type
                    //         when it optimizes properly.

                    match $name::from_repr((self as $signed) | (other as $signed)) {
                        Some(mask) => mask,
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }

                /// Compute the bitwise and logical `XOR`/`^` between two masks.
                #[inline(always)]
                #[must_use]
                pub const fn xor(self, other: $name) -> $name {
                    // SAFETY: `self` and `other` always have all their bits set or none.
                    //
                    //         Therefore the result of bitwise boolean operations (NOT, AND, OR, XOR, and so on..)
                    //         always results in all or no bits being set.
                    //
                    // FIXME:  Implement this in terms of matching on the mask type
                    //         when it optimizes properly.
                    match $name::from_repr((self as $signed) ^ (other as $signed)) {
                        Some(mask) => mask,
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }

                /// Compute the ordering between two masks.
                #[inline(always)]
                #[must_use]
                pub const fn compare(self, rhs: $name) -> Ordering {
                    match (rhs as $signed) - (self as $signed) {
                        -1 => Ordering::Less,
                        0 => Ordering::Equal,
                        1 => Ordering::Greater,
                        // NOTE: LLVM optimizes this away fine.
                        _ => unreachable!(),
                    }
                }

                /// Get a static reference to the mask stored in `self`.
                ///
                /// # Note
                ///
                /// The returned reference is likely not at the same
                /// location as `self`.
                ///
                /// This abuses static promotion, hence why there is no mutable
                /// variant.
                #[inline(always)]
                #[must_use]
                pub const fn promote(self) -> &'static $name {
                    // match self {
                    //     $name::True => &$name::True,
                    //     $name::False => &$name::False,
                    // }

                    const LOOKUP: &'static [$name; 2] = &[$name::True, $name::False];

                    // We're getting a pointer to the `False` element.
                    //
                    // If we decrement it, we get the pointer to the `True` element.
                    //
                    // If we add zero, we get the pointer to the `False` element.
                    let ptr = unsafe { LOOKUP.as_ptr().add(1) };

                    // Rust will probably do the right thing, however
                    // just to be sure as I cannot remember the casting semantics
                    // off the top of my head, we cast to the signed integer type of the
                    // same bit size, and then to `isize`.
                    //
                    // This is so that `-1` in say a `i8` becomes `-1` in a isize,
                    // rather than zero extending.
                    let offset = (self as $signed) as isize;

                    // Calculate the pointer of the desired mask.
                    let ptr = unsafe { ptr.offset(offset) };

                    // Turn it into a reference.
                    unsafe { &*ptr }
                }
            }

            const _: () = assert!($name::True.compare($name::False).is_gt());
            const _: () = assert!($name::True.compare($name::True).is_eq());
            const _: () = assert!($name::False.compare($name::True).is_lt());
            const _: () = assert!($name::False.compare($name::False).is_eq());

            impl Ord for $name {
                #[inline(always)]
                fn cmp(&self, rhs: &$name) -> Ordering {
                    self.compare(*rhs)
                }
            }

            impl<T> PartialEq<&T> for $name
            where
                T: ?Sized,
                $name: PartialEq<T>,
            {
                #[inline(always)]
                fn eq(&self, other: &&T) -> bool {
                    self.eq(*other)
                }
            }

            impl PartialOrd for $name {
                #[inline(always)]
                fn partial_cmp(&self, rhs: &$name) -> Option<Ordering> {
                    Some(self.compare(*rhs))
                }
            }

            impl<T> PartialOrd<&T> for $name
            where
                T: ?Sized,
                $name: PartialOrd<T>,
            {
                #[inline(always)]
                fn partial_cmp(&self, rhs: &&T) -> Option<Ordering> {
                    self.partial_cmp(*rhs)
                }
            }

            impl<T> BitOr<&T> for $name
            where
                T: Copy,
                $name: BitOr<T>,
            {
                type Output = <$name as BitOr<T>>::Output;

                #[inline(always)]
                fn bitor(self, rhs: &T) -> Self::Output {
                    self.bitor(*rhs)
                }
            }


            impl<T> BitAnd<&T> for $name
            where
                T: Copy,
                $name: BitAnd<T>,
            {
                type Output = <$name as BitAnd<T>>::Output;

                #[inline(always)]
                fn bitand(self, rhs: &T) -> Self::Output {
                    self.bitand(*rhs)
                }
            }


            impl<T> BitXor<&T> for $name
            where
                T: Copy,
                $name: BitXor<T>,
            {
                type Output = <$name as BitXor<T>>::Output;

                #[inline(always)]
                fn bitxor(self, rhs: &T) -> Self::Output {
                    self.bitxor(*rhs)
                }
            }

            impl<T> BitOr<T> for &$name
            where
                $name: BitOr<T>,
            {
                type Output = <$name as BitOr<T>>::Output;

                #[inline(always)]
                fn bitor(self, rhs: T) -> Self::Output {
                    (*self).bitor(rhs)
                }
            }


            impl<T> BitAnd<T> for &$name
            where
                $name: BitAnd<T>,
            {
                type Output = <$name as BitAnd<T>>::Output;

                #[inline(always)]
                fn bitand(self, rhs: T) -> Self::Output {
                    (*self).bitand(rhs)
                }
            }


            impl<T> BitXor<T> for &$name
            where
                $name: BitXor<T>,
            {
                type Output = <$name as BitXor<T>>::Output;

                #[inline(always)]
                fn bitxor(self, rhs: T) -> Self::Output {
                    (*self).bitxor(rhs)
                }
            }


            impl<T> BitOrAssign<T> for $name
            where
                $name: BitOr<T, Output = $name>,
            {
                #[inline(always)]
                fn bitor_assign(&mut self, rhs: T) {
                    *self = self.bitor(rhs);
                }
            }


            impl<T> BitAndAssign<T> for $name
            where
                $name: BitAnd<T, Output = $name>,
            {
                #[inline(always)]
                fn bitand_assign(&mut self, rhs: T) {
                    *self = self.bitand(rhs);
                }
            }


            impl<T> BitXorAssign<T> for $name
            where
                $name: BitXor<T, Output = $name>,
            {
                #[inline(always)]
                fn bitxor_assign(&mut self, rhs: T) {
                    *self = self.bitxor(rhs);
                }
            }

            impl<T> BitOrAssign<T> for &$name
            where
                $name: BitOr<T, Output = $name>,
            {
                #[inline(always)]
                fn bitor_assign(&mut self, rhs: T) {
                    // Here we abuse static promotion.
                    *self = (**self).bitor(rhs).promote();
                }
            }

           impl<T> BitAndAssign<T> for &$name
            where
                $name: BitAnd<T, Output = $name>,
            {
                #[inline(always)]
                fn bitand_assign(&mut self, rhs: T) {
                    // Here we abuse static promotion.
                    *self = (**self).bitand(rhs).promote();
                }
            }


           impl<T> BitXorAssign<T> for &$name
            where
                $name: BitXor<T, Output = $name>,
            {
                #[inline(always)]
                fn bitxor_assign(&mut self, rhs: T) {
                    // Here we abuse static promotion.
                    *self = (**self).bitxor(rhs).promote();
                }
            }


            impl BitOr for $name {
                type Output = $name;

                #[inline(always)]
                fn bitor(self, rhs: $name) -> $name {
                    self.or(rhs)
                }
            }

            impl BitAnd for $name {
                type Output = $name;

                #[inline(always)]
                fn bitand(self, rhs: $name) -> $name {
                    self.and(rhs)
                }
            }

            impl BitXor for $name {
                type Output = $name;

                #[inline(always)]
                fn bitxor(self, rhs: $name) -> $name {
                    self.xor(rhs)
                }
            }

            impl Not for $name {
                type Output = $name;

                #[inline(always)]
                fn not(self) -> $name {
                    <$name>::not(self)
                }
            }

            unsafe impl Lane for $name {
                type Mask = $name;

                const ZERO: $name = $name::False;
            }

            unsafe impl Mask for $name {
                const TRUE: $name = $name::True;
                const FALSE: $name = $name::False;
            }
        )*
    };
}

define_mask! {
    m8(i8, u8, "8-bit"),
    m16(i16, u16, "16-bit"),
    m32(i32, u32, "32-bit"),
    m64(i64, u64, "64-bit"),
    msize(isize, usize, "pointer-sized"),
}

#[no_mangle]
fn xor(a: &mut &m8, b: m8) {
    *a ^= b;
}

#[no_mangle]
fn promote(a: msize) -> &'static msize {
    a.promote()
}

#[no_mangle]
fn promote_old(a: msize) -> &'static msize {
    match a {
        msize::True => &msize::True,
        msize::False => &msize::False,
    }
}
