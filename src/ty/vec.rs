use super::mask::*;
use crate::muck::*;
use core::hash::{Hash, Hasher};

macro_rules! define_vec {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 1>;
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 2>;
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $prim, $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 4>;
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
        );
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 8>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 16>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 32>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
        );
    };


    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 64>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
            $vis $prim, $vis $prim, $vis $prim, $vis $prim,
        );
    };
}

macro_rules! link {
    () => {
        ""
    };
    ([$ty:ty; $count:tt]) => {
        concat!("[`", stringify!([$ty; $count]), "`](array)")
    };
    ([$ty:ty]) => {
        concat!("[`", stringify!([$ty]), "`](slice)")
    };
    (&$ty:ty) => {
        concat!("[`&", stringify!($ty), "`](reference)")
    };
    (&mut $ty:ty) => {
        concat!("[`&mut ", stringify!($ty), "`](reference)")
    };
    ($ty:ty) => {
        concat!("[`", stringify!($ty), "`](type@", stringify!($ty), ")")
    };
}

macro_rules! vec_common {
    ($align:tt, $bits:tt;
        $(#[$meta:meta])*
        struct $name:ident([$elem:ident; $count:tt]) {
            $(Hash $($hash:lifetime)?)?
            $(NoUninit $($no_uninit:lifetime)?)?
            $(Pod $($pod:lifetime)?)?
            $( $(@ $($is_mask:lifetime)?)?  mask($mask_ty:ident, $bits_ty:ident))?
        }
    ) => {
        define_vec!(
            #[doc = concat!(
                "A ", stringify!($bits), "-bit vector that stores ",
                stringify!($count), " ", link!($elem), "s."
            )]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = ""]
            #[doc = "## Memory Layout"]
            #[doc = ""]
            #[doc = concat!(
                "The memory layout of a ", link!($name), " is similar to a ", link!([$elem; $count]), "."
            )]
            #[doc = ""]
            #[doc = concat!(
                "* Both store ", stringify!($count), " contiguous ", link!($elem), "s."
            )]
            #[doc = "* Both have no uninit/padding bytes."]
            #[doc = ""]
            #[doc = "They differ in terms of alignment."]
            #[doc = concat!(
                "* A ", link!([$elem; $count]), " has an alignment equal \
                 to that of ", link!($elem), ", which is platform dependent."
             )]
            #[doc = concat!(
                "* A ", link!($name), " **always** has an alignment of ",
                stringify!($align), " bytes, regardless of platform."
            )]
            #[doc = ""]
            #[doc = concat!(
                "In other words, a ", link!($name), " \
                 is a ", link!([$elem; $count]), " \
                 with stricter alignment requirements."
            )]
            #[doc = ""]
            #[doc = "## Safe Operations"]
            #[doc = ""]
            #[doc = concat!(
                "* Creating a ", link!($name), " \
                 from a ", link!([$elem; $count]), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!([$elem; $count]), " \
                 from a ", link!($name), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&[$elem; $count]), " \
                 from a ", link!(&$name), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&mut [$elem; $count]), " \
                 from a ", link!(&mut $name), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&[[$elem; $count]]), " \
                 from a ", link!(&[$name]), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&mut [[$elem; $count]]), " \
                 from a ", link!(&mut [$name]), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&[$elem]), " \
                 from a ", link!(&[$name]), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&mut [$elem]), " \
                 from a ", link!(&mut [$name]), "."
            )]
            #[doc = ""]
            #[doc = "## Unsafe Operations"]
            #[doc = ""]
            #[doc = concat!(
                "* Creating a ", link!(&$name), " \
                 from a ", link!(&[$elem; $count]), "."
            )]
            #[doc = concat!(
                "* Creating a ", link!(&mut $name), " \
                 from a ", link!(&mut [$elem; $count]), "."
            )]
            #[doc = ""]
            #[doc = "## Operation Not Covered?"]
            #[doc = ""]
            #[doc = "When in doubt, utilize tools exposed by [`muck`](mod@crate::muck) or [`bytemuck`](::bytemuck)."]
            #[derive(Debug, Clone, Copy, PartialEq, Default)]
            #[repr(C, align($align))]
            #[allow(non_camel_case_types)]
            $(#[$meta])*
            pub struct $name<$elem, $count>;
        );

        // Some of these may be overkill but, better safe than sorry.

        // Ensure that the alignment matches the expected alignment.
        const _: () = assert!(align_of::<$name>() == $align);
        // Ensure that the size of the vector is the same as it's scalar counterpart.
        const _: () = assert!(size_of::<$name>() == size_of::<[$elem; $count]>());
        // Ensure that the alignment of the vector is the same as the size of it.
        const _: () = assert!(align_of::<$name>() == size_of::<$name>());
        // Ensure that the vector is not a ZST.
        const _: () = assert!(size_of::<$name>() != 0);
        // Ensure that the element type is not a ZST.
        const _: () = assert!(size_of::<$elem>() != 0);

        impl $name {
            /// The length of this vector type when treated as an array
            /// of it's elements.
            pub const LEN: usize = $count;

            /// The amount of lanes within this vector type.
            pub const LANES: u32 = $count;

            /// The amount of bits that this vector type occupies.
            ///
            /// This is **not** the amount of bits per lane.
            pub const BITS: u32 = $bits;

            /// The amount of bits that each lane occupies.
            ///
            /// This is **not** the total amount of bits
            /// occupied by this vector type.
            pub const LANE_BITS: u32 = $name::BITS / $name::LANES;

            /// Create a vector from an array of elements.
            #[inline(always)]
            #[must_use]
            pub const fn from_array(arr: [$elem; $count]) -> $name {
                // SAFETY: See the documentation for this type.
                unsafe { core::mem::transmute(arr) }
            }

            /// Create an array of elements from a vector.
            #[inline(always)]
            #[must_use]
            pub const fn to_array(self) -> [$elem; $count] {
                // SAFETY: See the documentation for this type.
                unsafe { core::mem::transmute(self) }
            }

            /// Get a reference to an array of elements from a vector.
            #[inline(always)]
            #[must_use]
            pub const fn as_array(&self) -> &[$elem; $count] {
                // SAFETY: See the documentation for this type.
                unsafe { &*(self as *const $name as *const [$elem; $count]) }
            }

            /// Get a mutable reference to an array of elements from a vector.
            #[inline(always)]
            #[must_use]
            pub const fn as_array_mut(&mut self) -> &mut [$elem; $count] {
                // SAFETY: See the documentation for this type.
                unsafe { &mut *(self as *mut $name as *mut [$elem; $count]) }
            }

            /// Get a slice of elements from a vector.
            #[inline(always)]
            #[must_use]
            pub const fn as_slice(&self) -> &[$elem] {
                self.as_array()
            }

            /// Get a mutable slice of elements from a vector.
            #[inline(always)]
            #[must_use]
            pub const fn as_slice_mut(&mut self) -> &mut [$elem] {
                self.as_array_mut()
            }

            /// Cast a slice of vectors to a slice of arrays of elements.
            #[inline(always)]
            #[must_use]
            pub const fn as_arrays(vecs: &[$name]) -> &[[$elem; $count]] {
                // SAFETY: See the documentation for this type.
                unsafe { &*(vecs as *const [$name] as *const [[$elem; $count]]) }
            }

            /// Cast a mutable slice of vectors to a mutable slice of arrays of elements.
            #[inline(always)]
            #[must_use]
            pub const fn as_arrays_mut(vecs: &mut [$name]) -> &mut [[$elem; $count]] {
                // SAFETY: See the documentation for this type.
                unsafe { &mut *(vecs as *mut [$name] as *mut [[$elem; $count]]) }
            }

            /// Cast a slice of vectors to a slice of elements.
            #[inline(always)]
            #[must_use]
            pub const fn as_flattened(vecs: &[$name]) -> &[$elem] {
                // SAFETY: Neither the vector or element types
                //         are ZSTs, and both exist already within
                //         the address space.
                let len = unsafe { vecs.len().unchecked_mul($count) };

                // SAFETY: `[T]` is layout-identical to `[T; N]`.
                unsafe { core::slice::from_raw_parts(vecs.as_ptr().cast(), len) }
            }

            /// Cast a mutable slice of vectors to a mutable slice of elements.
            #[inline(always)]
            #[must_use]
            pub const fn as_flattened_mut(vecs: &mut [$name]) -> &mut [$elem] {
                // SAFETY: Neither the vector or element types
                //         are ZSTs, and both exist already within
                //         the address space.
                let len = unsafe { vecs.len().unchecked_mul($count) };

                // SAFETY: `[T]` is layout-identical to `[T; N]`.
                unsafe { core::slice::from_raw_parts_mut(vecs.as_mut_ptr().cast(), len) }
            }
        }

        unsafe impl Zeroable for $name {}
        $($($pod)? unsafe impl Pod for $name {})?
        $($($no_uninit)? unsafe impl NoUninit for $name {})?


        $(
            $($hash)?
            impl Hash for $name {
                #[inline]
                fn hash<H>(&self, state: &mut H)
                where
                    H: Hasher,
                {
                    self.as_array().hash(state)
                }

                #[inline]
                fn hash_slice<H>(data: &[$name], state: &mut H)
                where
                    H: Hasher,
                {
                    $name::as_flattened(data).hash(state)
                }
            }
        )?

        $(


            $($($is_mask)?
                unsafe impl CheckedBitPattern for $name {
                    type Bits = $bits_ty;

                    #[inline(always)]
                    fn is_valid_bit_pattern(bits: &$bits_ty) -> bool {
                        bits.is_mask()
                    }
                }
            )?

            const _: () = assert!($bits_ty::LANES == $name::LANES);
            const _: () = assert!($mask_ty::LANES == $name::LANES);
            const _: () = assert!(align_of::<$mask_ty>() == align_of::<$name>());

            impl $name {
                #[doc = "Returns whether this vector has a valid bit pattern for a mask vector."]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = ""]
                #[doc = concat!(
                    "A `true` return value indicates that it is safe to convert this vector \
                     into a ", link!($mask_ty), ".",
                )]
                #[inline(always)]
                #[must_use]
                pub const fn is_mask(&self) -> bool {
                    let mut elems = cast_ref::<_, $bits_ty>(self).as_slice();
                    let mut all_valid = true;

                    while let [b, rest @ ..] = elems {
                        all_valid &= matches!(*b, 0 | -1);
                        elems = rest;
                    }

                    all_valid
                }

                #[doc = concat!(
                    "Get a ", link!($mask_ty), " from `self` if it has a \
                     valid bit pattern for a ", link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                pub const fn try_to_mask(self) -> Option<$mask_ty> {
                    if self.is_mask() {
                        // SAFETY: We confirmed that this vector has a valid bit pattern
                        //         for a mask vector.
                        Some(unsafe { core::mem::transmute(self) })
                    } else {
                        None
                    }
                }

                #[doc = concat!("Get a ", link!($mask_ty), " from `self`.")]
                #[doc = ""]
                #[doc = "# Panics"]
                #[doc = ""]
                #[doc = concat!(
                    "Panics if `self` has an invalid bit pattern for a ",
                    link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const fn to_mask(self) -> $mask_ty {
                    match self.try_to_mask() {
                        Some(mask) => mask,
                        None => panic!("invalid mask vector"),
                    }
                }

                #[doc = concat!("Get a ", link!($mask_ty), " from `self`, without checks.")]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = ""]
                #[doc = concat!(
                    "The caller must ensure that `self` has a valid bit pattern for a ",
                    link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const unsafe fn to_mask_unchecked(self) -> $mask_ty {
                    match self.try_to_mask() {
                        Some(mask) => mask,
                        None if cfg!(debug_assertions) => panic!("undefined behavior: invalid mask vector"),
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }

                #[doc = concat!(
                    "Get a ", link!(&$mask_ty), " from `self` if it has a \
                     valid bit pattern for a ", link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                pub const fn try_as_mask(&self) -> Option<&$mask_ty> {
                    if self.is_mask() {
                        // SAFETY: We confirmed that this vector has a valid bit pattern
                        //         for a mask vector.
                        //
                        //         Additionally, we know that the size of the mask vector,
                        //         this vector are equal, and that the alignment of both,
                        //         are equal, and that the lanes of this vector has
                        //         the same memory layout as the mask vector's lanes.
                        Some(unsafe { &*(self as *const $name as *const $mask_ty) })
                    } else {
                        None
                    }
                }


                #[doc = concat!("Get a ", link!(&$mask_ty), " from `self`.")]
                #[doc = ""]
                #[doc = "# Panics"]
                #[doc = ""]
                #[doc = concat!(
                    "Panics if `self` has an invalid bit pattern for a ",
                    link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const fn as_mask(&self) -> &$mask_ty {
                    match self.try_as_mask() {
                        Some(mask) => mask,
                        None => panic!("invalid mask vector"),
                    }
                }


                #[doc = concat!("Get a ", link!(&$mask_ty), " from `self`, without checks.")]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = ""]
                #[doc = concat!(
                    "The caller must ensure that `self` has a valid bit pattern for a ",
                    link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const unsafe fn as_mask_unchecked(&self) -> &$mask_ty {
                    match self.try_as_mask() {
                        Some(mask) => mask,
                        None if cfg!(debug_assertions) => panic!("undefined behavior: invalid mask vector"),
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }

                #[doc = concat!(
                    "Get a ", link!(&mut $mask_ty), " from `self` if it has a \
                     valid bit pattern for a ", link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                pub const fn try_as_mask_mut(&mut self) -> Option<&mut $mask_ty> {
                    if self.is_mask() {
                        // SAFETY: We confirmed that this vector has a valid bit pattern
                        //         for a mask vector.
                        //
                        //         Additionally, we know that the size of the mask vector,
                        //         this vector are equal, and that the alignment of both,
                        //         are equal, and that the lanes of this vector has
                        //         the same memory layout as the mask vector's lanes.
                        Some(unsafe { &mut *(self as *mut $name as *mut $mask_ty) })
                    } else {
                        None
                    }
                }


                #[doc = concat!("Get a ", link!(&mut $mask_ty), " from `self`.")]
                #[doc = ""]
                #[doc = "# Panics"]
                #[doc = ""]
                #[doc = concat!(
                    "Panics if `self` has an invalid bit pattern for a ",
                    link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const fn as_mask_mut(&mut self) -> &mut $mask_ty {
                    match self.try_as_mask_mut() {
                        Some(mask) => mask,
                        None => panic!("invalid mask vector"),
                    }
                }


                #[doc = concat!("Get a ", link!(&mut $mask_ty), " from `self`, without checks.")]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = ""]
                #[doc = concat!(
                    "The caller must ensure that `self` has a valid bit pattern for a ",
                    link!($mask_ty), "."
                )]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const unsafe fn as_mask_mut_unchecked(&mut self) -> &mut $mask_ty {
                    match self.try_as_mask_mut() {
                        Some(mask) => mask,
                        None if cfg!(debug_assertions) => panic!("undefined behavior: invalid mask vector"),
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }
            }
        )?
    };
}

macro_rules! vecs {
    ($align:tt, $bits:tt;

    $(struct $mask:ident<$mask_ty:ident, $count:tt> {
        $unsigned:ident: $u_ty:ident,
        $signed:ident: $s_ty:ident
        $(, $float:ident: $f_ty:ident)?
        $(,)?
    })*
    ) => {
        $(
            vec_common!($align, $bits;
                #[derive(Eq)]
                struct $mask([$mask_ty; $count]) {
                    Hash
                    NoUninit
                    @mask($mask, $signed)
                }
            );

            vec_common!($align, $bits;
                #[derive(Eq)]
                struct $unsigned([$u_ty; $count]) {
                    Hash
                    Pod
                    mask($mask, $signed)
                }
            );


            vec_common!($align, $bits;
                #[derive(Eq)]
                struct $signed([$s_ty; $count]) {
                    Hash
                    Pod
                    mask($mask, $signed)
                }
            );

            $(vec_common!($align, $bits;
                struct $float([$f_ty; $count]) {
                    Pod
                    mask($mask, $signed)
                }
            );)?
        )*
    };
}

vecs!(8, 64;
    struct m8x8<m8, 8> {
        u8x8: u8,
        i8x8: i8,
    }

    struct m16x4<m16, 4> {
        u16x4: u16,
        i16x4: i16,
    }

    struct m32x2<m32, 2> {
        u32x2: u32,
        i32x2: i32,
        f32x2: f32,
    }
);

vecs!(16, 128;
    struct m8x16<m8, 16> {
        u8x16: u8,
        i8x16: i8,
    }

    struct m16x8<m16, 8> {
        u16x8: u16,
        i16x8: i16,
    }

    struct m32x4<m32, 4> {
        u32x4: u32,
        i32x4: i32,
        f32x4: f32,
    }

    struct m64x2<m64, 2> {
        u64x2: u64,
        i64x2: i64,
        f64x2: f64,
    }
);

vecs!(32, 256;
    struct m8x32<m8, 32> {
        u8x32: u8,
        i8x32: i8,
    }

    struct m16x16<m16, 16> {
        u16x16: u16,
        i16x16: i16,
    }

    struct m32x8<m32, 8> {
        u32x8: u32,
        i32x8: i32,
        f32x8: f32,
    }

    struct m64x4<m64, 4> {
        u64x4: u64,
        i64x4: i64,
        f64x4: f64,
    }
);

vecs!(64, 512;
    struct m8x64<m8, 64> {
        u8x64: u8,
        i8x64: i8,
    }

    struct m16x32<m16, 32> {
        u16x32: u16,
        i16x32: i16,
    }

    struct m32x16<m32, 16> {
        u32x16: u32,
        i32x16: i32,
        f32x16: f32,
    }

    struct m64x8<m64, 8> {
        u64x8: u64,
        i64x8: i64,
        f64x8: f64,
    }
);
