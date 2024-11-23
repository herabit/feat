use crate::ty::vec::*;
// use intrin::{__m128, __m128d, __m128i, __m256, __m256d, __m256i, __m512, __m512d, __m512i};

use bytemuck::TransparentWrapper;

impl_intrin!(
    pub struct m512(__m512) {
        from_m512,
        from_m512_ref,
        from_m512_mut,
        to_m512,
        as_m512,
        as_m512_mut,
    }: f32x16
);

impl_intrin!(
    pub struct m512d(__m512d) {
        from_m512d,
        from_m512d_ref,
        from_m512d_mut,
        to_m512d,
        as_m512d,
        as_m512d_mut,
    }: f64x8
);

impl_intrin!(
    pub struct m512i(__m512i) {
        from_m512i,
        from_m512i_ref,
        from_m512i_mut,
        to_m512i,
        as_m512i,
        as_m512i_mut,
    }:
        u8x64, i8x64,
        u16x32, i16x32,
        u32x16, i32x16,
        u64x8, i64x8,
);

impl_intrin!(
    pub struct m256(__m256) {
        from_m256,
        from_m256_ref,
        from_m256_mut,
        to_m256,
        as_m256,
        as_m256_mut,
    }: f32x8
);

impl_intrin!(
    pub struct m256d(__m256d) {
        from_m256d,
        from_m256d_ref,
        from_m256d_mut,
        to_m256d,
        as_m256d,
        as_m256d_mut,
    }: f64x4
);

impl_intrin!(
    pub struct m256i(__m256i) {
        from_m256i,
        from_m256i_ref,
        from_m256i_mut,
        to_m256i,
        as_m256i,
        as_m256i_mut,
    }:
        u8x32, i8x32,
        u16x16, i16x16,
        u32x8, i32x8,
        u64x4, i64x4,
);

impl_intrin!(
    pub struct m128(__m128) {
        from_m128,
        from_m128_ref,
        from_m128_mut,
        to_m128,
        as_m128,
        as_m128_mut,
    }: f32x4
);

impl_intrin!(
    pub struct m128d(__m128d) {
        from_m128d,
        from_m128d_ref,
        from_m128d_mut,
        to_m128d,
        as_m128d,
        as_m128d_mut,
    }: f64x2
);

impl_intrin!(
    pub struct m128i(__m128i) {
        from_m128i,
        from_m128i_ref,
        from_m128i_mut,
        to_m128i,
        as_m128i,
        as_m128i_mut,
    }:
        u8x16, i8x16,
        u16x8, i16x8,
        u32x4, i32x4,
        u64x2, i64x2,
);

macro_rules! impl_intrin {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($inner:ident)
        {
            $from:ident,
            $from_ref:ident,
            $from_mut:ident,
            $to:ident,
            $as_ref:ident,
            $as_mut:ident $(,)?
        }: $($vec:ty),* $(,)?
    ) => {
        $(#[$attr])*
        #[doc = concat!(
            "See [`", stringify!(intrin::$inner), "`] for details."
        )]
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[allow(non_camel_case_types)]
        $vis struct $name($vis intrin::$inner);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<intrin::$inner> for $name {
            #[inline]
            fn from(x: intrin::$inner) -> $name {
                TransparentWrapper::wrap(x)
            }
        }

        impl From<$name> for intrin::$inner {
            #[inline]
            fn from(x: $name) -> intrin::$inner {
                TransparentWrapper::peel(x)
            }
        }

        impl AsRef<intrin::$inner> for $name {
            #[inline]
            fn as_ref(&self) -> &intrin::$inner {
                TransparentWrapper::peel_ref(self)
            }
        }

        impl AsRef<$name> for intrin::$inner {
            #[inline]
            fn as_ref(&self) -> &$name {
                TransparentWrapper::wrap_ref(self)
            }
        }

        impl AsMut<intrin::$inner> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut intrin::$inner {
                TransparentWrapper::peel_mut(self)
            }
        }

        impl AsMut<$name> for intrin::$inner {
            #[inline]
            fn as_mut(&mut self) -> &mut $name {
                TransparentWrapper::wrap_mut(self)
            }
        }

        impl core::borrow::Borrow<intrin::$inner> for $name {
            #[inline]
            fn borrow(&self) -> &intrin::$inner {
                self.as_ref()
            }
        }


        impl core::borrow::Borrow<$name> for intrin::$inner {
            #[inline]
            fn borrow(&self) -> &$name {
                self.as_ref()
            }
        }

        impl core::borrow::BorrowMut<intrin::$inner> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut intrin::$inner {
                self.as_mut()
            }
        }


        impl core::borrow::BorrowMut<$name> for intrin::$inner {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $name {
                self.as_mut()
            }
        }


        // SAFETY: All of the inner types in bytemuck implement
        //         these traits, however one, the 512-bit types,
        //         are not available without a feature that requires nightly.
        unsafe impl bytemuck::Zeroable for $name {}
        unsafe impl bytemuck::Pod for $name {}
        unsafe impl bytemuck::TransparentWrapper<intrin::$inner> for $name {}

        $(

            const _: () = {
                assert!(size_of::<$vec>() == size_of::<$name>(), "unexpected size");
                assert!(align_of::<$vec>() == align_of::<$name>(), "unexpected alignment");
            };

            impl $vec {
                #[inline(always)]
                #[must_use]
                pub const fn $from(x: $name) -> $vec {
                    bytemuck::must_cast(x)
                }

                #[inline(always)]
                #[must_use]
                pub const fn $from_ref(x: &$name) -> &$vec {
                    bytemuck::must_cast_ref(x)
                }

                #[inline(always)]
                #[must_use]
                pub fn $from_mut(x: &mut $name) -> &mut $name {
                    bytemuck::must_cast_mut(x)
                }

                #[inline(always)]
                #[must_use]
                pub const fn $to(self) -> $name {
                    bytemuck::must_cast(self)
                }

                #[inline(always)]
                #[must_use]
                pub const fn $as_ref(&self) -> &$name {
                    bytemuck::must_cast_ref(self)
                }

                #[inline(always)]
                #[must_use]
                pub fn $as_mut(&mut self) -> &mut $name {
                    bytemuck::must_cast_mut(self)
                }
            }
        )*
    };
}

pub(self) use impl_intrin;

pub use core::arch::x86_64 as intrin;

mod aes;
pub use aes::*;
mod sha;
pub use sha::*;
