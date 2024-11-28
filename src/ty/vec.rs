#![allow(unused_macros)]

use super::mask::*;
use crate::muck::*;

macro_rules! assert_impl {
    ($t:ty: $($trait:path),+ $(,)?) => {
        const _: fn() = || {
            fn assert_all<T: ?Sized $(+ $trait)+>() {}

            assert_all::<$t>()
        };
    };
}

macro_rules! define_vec {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 1>;
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $lane,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 2>;
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $lane, $vis $lane,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 4>;
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
        );
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 8>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 16>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 32>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
        );
    };


    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$lane:ty, 64>;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
            $vis $lane, $vis $lane, $vis $lane, $vis $lane,
        );
    };

    ($align:tt, $bits:tt; $(#[$attr:meta])* struct $name:ident<$lane:ty, $count:tt>;) => {
        define_vec!(
            #[doc = concat!(
                "A ", stringify!($bits), "-bit vector that stores ",
                stringify!($count), " ", stringify!($lane), "s."
            )]
            #[derive(Debug, Clone, Copy, PartialEq, Default)]
            #[repr(C, align($align))]
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct $name<$lane, $count>;
        );


        const _: () = assert!(size_of::<$name>() == size_of::<[$lane; $count]>());
        const _: () = assert!(align_of::<$name>() == $align);
        const _: () = assert!(size_of::<$name>() == $align);
    };
}

macro_rules! create_vecs {
    ($(
        ($align:tt, $bits:tt) => {$(
            $count:tt => {
                $mask:ident: $m_lane:ident,
                $unsigned:ident: $u_lane:ident,
                $signed:ident: $s_lane:ident
                $(, $float:ident: $f_lane:ident)?
                $(,)?
            }
        ),* $(,)?}
    ),* $(,)?) => {
        $(
            $(
                define_vec!($align, $bits;
                    #[derive(Eq)]
                    struct $mask<$m_lane, $count>;
                );

                define_vec!($align, $bits;
                    #[derive(Eq)]
                    struct $unsigned<$u_lane, $count>;
                );

                define_vec!($align, $bits;
                    #[derive(Eq)]
                    struct $signed<$s_lane, $count>;
                );

                $(define_vec!($align, $bits;
                    struct $float<$f_lane, $count>;
                );)?
            )*
        )*

        /// Given a vector type map it to the amount of lanes it holds.
        macro_rules! lanes {
            $(
                $(
                    ($mask) => { $count };
                    ($unsigned) => { $count };
                    ($signed) => { $count };
                    $( ($float) => { $count }; )?
                )*
            )*
        }

        /// Given a vector type map it to the type of lane it holds.
        macro_rules! lane {
            $(
                $(
                    ($mask) => { $m_lane };
                    ($unsigned) => { $u_lane };
                    ($signed) => { $s_lane };
                    $( ($float) => { $f_lane }; )?
                )*
            )*
        }

        /// Given a vector type map it to the array it "wraps".
        macro_rules! array {
            $(
                $(
                    ($mask) => { [$m_lane; $count] };
                    ($unsigned) => { [$u_lane; $count] };
                    ($signed) => { [$s_lane; $count] };
                    $( ($float) => { [$f_lane; $count] }; )?
                )*
            )*
        }

        /// Given a vector type map it to a slice of its lanes.
        macro_rules! slice {
            $(
                $(
                    ($mask) => { [$m_lane] };
                    ($unsigned) => { [$u_lane] };
                    ($signed) => { [$s_lane] };
                    $( ($float) => { [$f_lane] }; )?
                )*
            )*
        }

        /// Given an array, map it to the vector type of equal length and
        /// equal lane type.
        macro_rules! vector {
            $(
                $(
                    ([$m_lane; $count]) => { $mask };
                    ([$u_lane; $count]) => { $unsigned };
                    ([$s_lane; $count]) => { $signed };
                    $( ([$f_lane; $count]) => { $float }; )?
                )*
            )*
            ($lane:ty; $length:tt) => { $crate::ty::vector!([$lane; $length]) };
        }

        /// Given a vector type, map it to the mask type of equal length and
        /// whose lane type shares a size.
        macro_rules! mask {
            $(
                $(
                    ($mask) => { $mask };
                    ($unsigned) => { $mask };
                    ($signed) => { $mask };
                    $( ($float) => { $mask }; )?
                )*
            )*
        }

        /// Given a vector type, map it to the unsigned vector type of equal length
        /// and whose lane type shares a size.
        macro_rules! unsigned {
            $(
                $(
                    ($mask) => { $unsigned };
                    ($unsigned) => { $unsigned };
                    ($signed) => { $unsigned };
                    $( ($float) => { $unsigned }; )?
                )*
            )*
        }

        /// Given a vector type, map it to the signed vector type of equal length
        /// and whose lane type shares a size.
        macro_rules! signed {
            $(
                $(
                    ($mask) => { $signed };
                    ($unsigned) => { $signed };
                    ($signed) => { $signed };
                    $( ($float) => { $signed }; )?
                )*
            )*
        }

        /// Given a vector type, map it to the float vector type of equal length
        /// and whose lane type shares a size.
        macro_rules! float {
            $(
                $(
                    $(
                        ($mask) => { $float };
                        ($unsigned) => { $float };
                        ($signed) => { $float };
                        ($float) => { $float };
                    )?
                )*
            )*
        }
    };
}

create_vecs! {
    (8, 64) => {
        8 => {
            m8x8: m8,
            u8x8: u8,
            i8x8: i8,
        },
        4 => {
            m16x4: m16,
            u16x4: u16,
            i16x4: i16,
        },
        2 => {
            m32x2: m32,
            u32x2: u32,
            i32x2: i32,
            f32x2: f32,
        },
    },
    (16, 128) => {
        16 => {
            m8x16: m8,
            u8x16: u8,
            i8x16: i8,
        },
        8 => {
            m16x8: m16,
            u16x8: u16,
            i16x8: i16,
        },
        4 => {
            m32x4: m32,
            u32x4: u32,
            i32x4: i32,
            f32x4: f32,
        },
        2 => {
            m64x2: m64,
            u64x2: u64,
            i64x2: i64,
            f64x2: f64,
        }
    },
    (32, 256) => {
        32 => {
            m8x32: m8,
            u8x32: u8,
            i8x32: i8,
        },
        16 => {
            m16x16: m16,
            u16x16: u16,
            i16x16: i16,
        },
        8 => {
            m32x8: m32,
            u32x8: u32,
            i32x8: i32,
            f32x8: f32,
        },
        4 => {
            m64x4: m64,
            u64x4: u64,
            i64x4: i64,
            f64x4: f64,
        }
    },

    (64, 512) => {
        64 => {
            m8x64: m8,
            u8x64: u8,
            i8x64: i8,
        },
        32 => {
            m16x32: m16,
            u16x32: u16,
            i16x32: i16,
        },
        16 => {
            m32x16: m32,
            u32x16: u32,
            i32x16: i32,
            f32x16: f32,
        },
        8 => {
            m64x8: m64,
            u64x8: u64,
            i64x8: i64,
            f64x8: f64,
        }
    }
}

// Implement things that all vector types have in common.

macro_rules! impl_common {
    ($($name:ident),* $(,)?) => {$(
        impl $name {
            #[inline(always)]
            #[must_use]
            pub const fn from_array(arr: array!($name)) -> $name {
                unsafe { core::mem::transmute(arr) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn to_array(self) -> array!($name) {
                unsafe { core::mem::transmute(self) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_array(&self) -> &array!($name) {
                unsafe { &*(self as *const $name as *const array!($name)) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_array_mut(&mut self) -> &mut array!($name) {
                unsafe { &mut *(self as *mut $name as *mut array!($name)) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_slice(&self) -> &slice!($name) {
                self.as_array()
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_slice_mut(&mut self) -> &mut slice!($name) {
                self.as_array_mut()
            }
        }

        // All vector types are arrays of zeroable types.
        unsafe impl Zeroable for $name {}

        assert_impl!(lane!($name): Zeroable);
    )*};
}

impl_common!(m8x8, m8x16, m8x32, m8x64);
impl_common!(u8x8, u8x16, u8x32, u8x64);
impl_common!(i8x8, i8x16, i8x32, i8x64);

impl_common!(m16x4, m16x8, m16x16, m16x32);
impl_common!(u16x4, u16x8, u16x16, u16x32);
impl_common!(i16x4, i16x8, i16x16, i16x32);

impl_common!(m32x2, m32x4, m32x8, m32x16);
impl_common!(u32x2, u32x4, u32x8, u32x16);
impl_common!(i32x2, i32x4, i32x8, i32x16);
impl_common!(f32x2, f32x4, f32x8, f32x16);

impl_common!(m64x2, m64x4, m64x8);
impl_common!(u64x2, u64x4, u64x8);
impl_common!(i64x2, i64x4, i64x8);
impl_common!(f64x2, f64x4, f64x8);

// Implement Pod for types that are Pod.

macro_rules! impl_pod {
    ($($name:ident),* $(,)?) => {
        $(
            // All types passed to this macro are pod types.
            unsafe impl Pod for $name {}
            assert_impl!(lane!($name): Pod);
        )*
    };
}

impl_pod!(u8x8, u8x16, u8x32, u8x64);
impl_pod!(i8x8, i8x16, i8x32, i8x64);

impl_pod!(u16x4, u16x8, u16x16, u16x32);
impl_pod!(i16x4, i16x8, i16x16, i16x32);

impl_pod!(u32x2, u32x4, u32x8, u32x16);
impl_pod!(i32x2, i32x4, i32x8, i32x16);
impl_pod!(f32x2, f32x4, f32x8, f32x16);

impl_pod!(u64x2, u64x4, u64x8);
impl_pod!(i64x2, i64x4, i64x8);
impl_pod!(f64x2, f64x4, f64x8);

// Implement traits for mask vectors.

macro_rules! impl_mask {
    ($($name:ident),* $(,)?) => {
        $(
            unsafe impl NoUninit for $name {}
            assert_impl!(lane!($name): NoUninit);

            unsafe impl CheckedBitPattern for $name {
                type Bits = signed!($name);

                #[inline(always)]
                #[must_use]
                fn is_valid_bit_pattern(bits: &signed!($name)) -> bool {
                    $name::validate_repr(bits)
                }
            }

            impl $name {
                #[inline(always)]
                #[must_use]
                pub const fn validate_repr(repr: &signed!($name)) -> bool {
                    // FIXME: Add better heuristics for determining a chunk size
                    //        to offer even better codegen.
                    const CHUNK_SIZE: usize = if size_of::<usize>() % size_of::<lane!($name)>() == 0 {
                        size_of::<usize>() / size_of::<lane!($name)>()
                    } else {
                        1
                    };
                    type LaneBits = <lane!($name) as CheckedBitPattern>::Bits;

                    #[inline(always)]
                    const fn validate_lane(
                        acc: bool,
                        x: &LaneBits,
                    ) -> bool {
                        acc & <lane!($name)>::try_from_repr(*x).is_some()
                    }

                    let mut chunks: &[[LaneBits; CHUNK_SIZE]] = cast_slice(core::slice::from_ref(repr));
                    let mut all_valid = true;

                    while let (true, [chunk, rest @ ..]) = (all_valid, chunks) {
                        chunks = rest;

                        let mut chunk: &[LaneBits] = chunk;
                        while let [lane, rest @ ..] = chunk {
                            chunk = rest;
                            all_valid = validate_lane(all_valid, lane);
                        }
                    }

                    all_valid
                }
            }
        )*
    };
}

impl_mask!(m8x8, m8x16, m8x32, m8x64);
impl_mask!(m16x4, m16x8, m16x16, m16x32);
impl_mask!(m32x2, m32x4, m32x8, m32x16);
impl_mask!(m64x2, m64x4, m64x8);
