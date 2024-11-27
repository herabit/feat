use super::mask::*;
use crate::muck::*;

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

macro_rules! vec_common {
    ($align:tt, $bits:tt; $(#[$meta:meta])* struct $name:ident([$elem:ty; $count:tt]);) => {
        define_vec!(
            #[derive(Debug, Clone, Copy, PartialEq, Default, Zeroable)]
            #[repr(C, align($align))]
            #[allow(non_camel_case_types)]
            $(#[$meta])*
            pub struct $name<$elem, $count>;
        );

        const _: () = assert!(size_of::<$name>() == size_of::<[$elem; $count]>());
        const _: () = assert!(align_of::<$name>() == size_of::<$name>());

        impl $name {
            #[inline(always)]
            #[must_use]
            pub const fn from_array(arr: [$elem; $count]) -> $name {
                unsafe { core::mem::transmute(arr) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn to_array(self) -> [$elem; $count] {
                unsafe { core::mem::transmute(self) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_array(&self) -> &[$elem; $count] {
                unsafe { &*(self as *const $name as *const [$elem; $count]) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_array_mut(&mut self) -> &mut [$elem; $count] {
                unsafe { &mut *(self as *mut $name as *mut [$elem; $count]) }
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_slice(&self) -> &[$elem] {
                self.as_array()
            }

            #[inline(always)]
            #[must_use]
            pub const fn as_slice_mut(&mut self) -> &mut [$elem] {
                self.as_array_mut()
            }
        }
    };
}

macro_rules! vecs {
    ($align:tt, $bits:tt;

    $(struct $mask:ident<$mask_ty:ty, $count:tt> {
        $unsigned:ident: $u_ty:ty,
        $signed:ident: $s_ty:ty
        $(, $float:ident: $f_ty:ty)?
        $(,)?
    })*
    ) => {
        $(
            vec_common!($align, $bits;
                #[derive(Eq, CheckedBitPattern, Hash)]
                struct $mask([$mask_ty; $count]);
            );

            vec_common!($align, $bits;
                #[derive(Eq, Pod, Hash)]
                struct $unsigned([$u_ty; $count]);
            );


            vec_common!($align, $bits;
                #[derive(Eq, Pod, Hash)]
                struct $signed([$s_ty; $count]);
            );

            $(vec_common!($align, $bits;
                #[derive(Pod)]
                struct $float([$f_ty; $count]);
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
