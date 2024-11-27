use super::mask::*;
use crate::muck;

impl_vecs!(8, 64;
    pub struct u8x8<u8, 8> Eq Hash;
    pub struct i8x8<i8, 8> Eq Hash;

    pub struct u16x4<u16, 4> Eq Hash;
    pub struct i16x4<i16, 4> Eq Hash;

    pub struct u32x2<u32, 2> Eq Hash;
    pub struct i32x2<i32, 2> Eq Hash;
    pub struct f32x2<f32, 2>;

    // pub struct u64x1<u64, 1> Eq Hash;
    // pub struct i64x1<i64, 1> Eq Hash;
    // pub struct f64x1<f64, 1>;
);

mask_vecs!(8, 64;
    pub struct m8x8<m8, 8> {
        u8x8: {
            try_from_u8x8,
            from_u8x8,
            from_u8x8_unchecked,
            to_u8x8,
            as_u8x8,
        },
        i8x8: {
            try_from_i8x8,
            from_i8x8,
            from_i8x8_unchecked,
            to_i8x8,
            as_i8x8,
        },
    }
    pub struct m16x4<m16, 4> {
        u16x4: {
            try_from_u16x4,
            from_u16x4,
            from_u16x4_unchecked,
            to_u16x4,
            as_u16x4,
        },
        i16x4: {
            try_from_i16x4,
            from_i16x4,
            from_i16x4_unchecked,
            to_i16x4,
            as_i16x4,
        },
    }
    pub struct m32x2<m32, 2> {
        u32x2: {
            try_from_u32x2,
            from_u32x2,
            from_u32x2_unchecked,
            to_u32x2,
            as_u32x2,
        },
        i32x2: {
            try_from_i32x2,
            from_i32x2,
            from_i32x2_unchecked,
            to_i32x2,
            as_i32x2,
        },
        f32x2: {
            try_from_f32x2,
            from_f32x2,
            from_f32x2_unchecked,
            to_f32x2,
            as_f32x2,
        },
    }
);

impl_vecs!(16, 128;
    pub struct u8x16<u8, 16> Eq Hash;
    pub struct i8x16<i8, 16> Eq Hash;

    pub struct u16x8<u16, 8> Eq Hash;
    pub struct i16x8<i16, 8> Eq Hash;

    pub struct u32x4<u32, 4> Eq Hash;
    pub struct i32x4<i32, 4> Eq Hash;
    pub struct f32x4<f32, 4>;

    pub struct u64x2<u64, 2> Eq Hash;
    pub struct i64x2<i64, 2> Eq Hash;
    pub struct f64x2<f64, 2>;
);

mask_vecs!(16, 128;
    pub struct m8x16<m8, 16> {
        u8x16: {
            try_from_u8x16,
            from_u8x16,
            from_u8x16_unchecked,
            to_u8x16,
            as_u8x16,
        },
        i8x16: {
            try_from_i8x16,
            from_i8x16,
            from_i8x16_unchecked,
            to_i8x16,
            as_i8x16,
        },
    }
    pub struct m16x8<m16, 8> {
        u16x8: {
            try_from_u16x8,
            from_u16x8,
            from_u16x8_unchecked,
            to_u16x8,
            as_u16x8,
        },
        i16x8: {
            try_from_i16x8,
            from_i16x8,
            from_i16x8_unchecked,
            to_i16x8,
            as_i16x8,
        },
    }
    pub struct m32x4<m32, 4> {
        u32x4: {
            try_from_u32x4,
            from_u32x4,
            from_u32x4_unchecked,
            to_u32x4,
            as_u32x4,
        },
        i32x4: {
            try_from_i32x4,
            from_i32x4,
            from_i32x4_unchecked,
            to_i32x4,
            as_i32x4,
        },
        f32x4: {
            try_from_f32x4,
            from_f32x4,
            from_f32x4_unchecked,
            to_f32x4,
            as_f32x4,
        },
    }
    pub struct m64x2<m64, 2> {
        u64x2: {
            try_from_u64x2,
            from_u64x2,
            from_u64x2_unchecked,
            to_u64x2,
            as_u64x2,
        },
        i64x2: {
            try_from_i64x2,
            from_i64x2,
            from_i64x2_unchecked,
            to_i64x2,
            as_i64x2,
        },
        f64x2: {
            try_from_f64x2,
            from_f64x2,
            from_f64x2_unchecked,
            to_f64x2,
            as_f64x2,
        },
    }
);

impl_vecs!(32, 256;
    pub struct u8x32<u8, 32> Eq Hash;
    pub struct i8x32<i8, 32> Eq Hash;

    pub struct u16x16<u16, 16> Eq Hash;
    pub struct i16x16<i16, 16> Eq Hash;

    pub struct u32x8<u32, 8> Eq Hash;
    pub struct i32x8<i32, 8> Eq Hash;
    pub struct f32x8<f32, 8>;

    pub struct u64x4<u64, 4> Eq Hash;
    pub struct i64x4<i64, 4> Eq Hash;
    pub struct f64x4<f64, 4>;
);

mask_vecs!(32, 256;
    pub struct m8x32<m8, 32> {
        u8x32: {
            try_from_u8x32,
            from_u8x32,
            from_u8x32_unchecked,
            to_u8x32,
            as_u8x32,
        },
        i8x32: {
            try_from_i8x32,
            from_i8x32,
            from_i8x32_unchecked,
            to_i8x32,
            as_i8x32,
        },
    }
    pub struct m16x16<m16, 16> {
        u16x16: {
            try_from_u16x16,
            from_u16x16,
            from_u16x16_unchecked,
            to_u16x16,
            as_u16x16,
        },
        i16x16: {
            try_from_i16x16,
            from_i16x16,
            from_i16x16_unchecked,
            to_i16x16,
            as_i16x16,
        },
    }
    pub struct m32x8<m32, 8> {
        u32x8: {
            try_from_u32x8,
            from_u32x8,
            from_u32x8_unchecked,
            to_u32x8,
            as_u32x8,
        },
        i32x8: {
            try_from_i32x8,
            from_i32x8,
            from_i32x8_unchecked,
            to_i32x8,
            as_i32x8,
        },
        f32x8: {
            try_from_f32x8,
            from_f32x8,
            from_f32x8_unchecked,
            to_f32x8,
            as_f32x8,
        },
    }
    pub struct m64x4<m64, 4> {
        u64x4: {
            try_from_u64x4,
            from_u64x4,
            from_u64x4_unchecked,
            to_u64x4,
            as_u64x4,
        },
        i64x4: {
            try_from_i64x4,
            from_i64x4,
            from_i64x4_unchecked,
            to_i64x4,
            as_i64x4,
        },
        f64x4: {
            try_from_f64x4,
            from_f64x4,
            from_f64x4_unchecked,
            to_f64x4,
            as_f64x4,
        },
    }
);

impl_vecs!(64, 512;
    pub struct u8x64<u8, 64> Eq Hash;
    pub struct i8x64<i8, 64> Eq Hash;

    pub struct u16x32<u16, 32> Eq Hash;
    pub struct i16x32<i16, 32> Eq Hash;

    pub struct u32x16<u32, 16> Eq Hash;
    pub struct i32x16<i32, 16> Eq Hash;
    pub struct f32x16<f32, 16>;

    pub struct u64x8<u64, 8> Eq Hash;
    pub struct i64x8<i64, 8> Eq Hash;
    pub struct f64x8<f64, 8>;
);

#[no_mangle]
pub fn valid_mask(b: f32x4) -> bool {
    b.is_valid_mask()
}

mask_vecs!(64, 512;
    pub struct m8x64<m8, 64> {
        u8x64: {
            try_from_u8x64,
            from_u8x64,
            from_u8x64_unchecked,
            to_u8x64,
            as_u8x64,
        },
        i8x64: {
            try_from_i8x64,
            from_i8x64,
            from_i8x64_unchecked,
            to_i8x64,
            as_i8x64,
        },
    }
    pub struct m16x32<m16, 32> {
        u16x32: {
            try_from_u16x32,
            from_u16x32,
            from_u16x32_unchecked,
            to_u16x32,
            as_u16x32,
        },
        i16x32: {
            try_from_i16x32,
            from_i16x32,
            from_i16x32_unchecked,
            to_i16x32,
            as_i16x32,
        },
    }
    pub struct m32x16<m32, 16> {
        u32x16: {
            try_from_u32x16,
            from_u32x16,
            from_u32x16_unchecked,
            to_u32x16,
            as_u32x16,
        },
        i32x16: {
            try_from_i32x16,
            from_i32x16,
            from_i32x16_unchecked,
            to_i32x16,
            as_i32x16,
        },
        f32x16: {
            try_from_f32x16,
            from_f32x16,
            from_f32x16_unchecked,
            to_f32x16,
            as_f32x16,
        },
    }
    pub struct m64x8<m64, 8> {
        u64x8: {
            try_from_u64x8,
            from_u64x8,
            from_u64x8_unchecked,
            to_u64x8,
            as_u64x8,
        },
        i64x8: {
            try_from_i64x8,
            from_i64x8,
            from_i64x8_unchecked,
            to_i64x8,
            as_i64x8,
        },
        f64x8: {
            try_from_f64x8,
            from_f64x8,
            from_f64x8_unchecked,
            to_f64x8,
            as_f64x8,
        },
    }
);

macro_rules! mask_vecs {
    ($align:literal, $bits:literal;
    $(
        $(#[$meta:meta])* $vis:vis struct $name:ident<$mask:ident, $count:tt> {
            $(
                $vec_name:ident: {
                    $try_from_vec:ident,
                    $from_vec:ident,
                    $from_vec_unchecked:ident,
                    $to_vec:ident,
                    $as_vec:ident
                    $(,)?
                }
            ),*

            $(,)?
        }
    )*
    ) => {
        $(
            self::impl_vec!(
                #[allow(non_camel_case_types)]
                #[repr(C, align($align))]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ::bytemuck::Zeroable, ::bytemuck::NoUninit, ::bytemuck::CheckedBitPattern)]
                $(#[$meta])*
                #[doc = concat!(
                    "A ", stringify!($bits), "-bit vector type storing ",
                    stringify!($count), " [`", stringify!($mask), "`]s."
                )]
                #[doc = ""]
                #[doc = concat!(
                    "It is aligned to ", stringify!($align), "-byte boundaries, regardless if",
                    " the host system actually supports SIMD with this type."
                )]
                $vis struct $name<$mask, $count>
            );


            const _: ()  = {
                assert!(size_of::<$name>() == $align);
                assert!(align_of::<$name>() == $align);
            };

            impl $name {
                /// Create a new vector from an array of elements.
                #[inline(always)]
                #[must_use]
                pub const fn from_array(arr: [$mask; $count]) -> $name {
                    // SAFETY: The layout of a vector of masks is the same as an overaligned array.
                    unsafe { core::mem::transmute(arr) }
                }

                /// Return an array containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn to_array(self) -> [$mask; $count] {
                    // SAFETY: The layout of a vector of masks is the same as an overaligned array.
                    unsafe { core::mem::transmute(self) }
                }

                /// Return an array reference containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_array(&self) -> &[$mask; $count] {
                    // SAFETY: The layout of a vector of masks is the same as an overaligned array.
                    unsafe { &*(self as *const $name as *const [$mask; $count]) }
                }

                /// Return a mutable array reference containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_array_mut(&mut self) -> &mut [$mask; $count] {
                    // SAFETY: The layout of a vector of maskes is the same as an overaligned array.
                    unsafe { &mut *(self as *mut $name as *mut [$mask; $count]) }
                }

                /// Return a slice containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_slice(&self) -> &[$mask] {
                    self.as_array()
                }

                /// Return a mutable slice containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_slice_mut(&mut self) -> &mut [$mask] {
                    self.as_array_mut()
                }
            }

            impl AsRef<[$mask; $count]> for $name {
                #[inline(always)]
                fn as_ref(&self) -> &[$mask; $count] {
                    self.as_array()
                }
            }

            impl AsMut<[$mask; $count]> for $name {
                #[inline(always)]
                fn as_mut(&mut self) -> &mut [$mask; $count] {
                    self.as_array_mut()
                }
            }

            impl core::borrow::Borrow<[$mask; $count]> for $name {
                #[inline(always)]
                fn borrow(&self) -> &[$mask; $count] {
                    self.as_array()
                }
            }

            impl core::borrow::BorrowMut<[$mask; $count]> for $name {
                #[inline(always)]
                fn borrow_mut(&mut self) -> &mut [$mask; $count] {
                    self.as_array_mut()
                }
            }

            impl AsRef<[$mask]> for $name {
                #[inline(always)]
                fn as_ref(&self) -> &[$mask] {
                    self.as_slice()
                }
            }

            impl AsMut<[$mask]> for $name {
                #[inline(always)]
                fn as_mut(&mut self) -> &mut [$mask] {
                    self.as_slice_mut()
                }
            }

            impl core::borrow::Borrow<[$mask]> for $name {
                #[inline(always)]
                fn borrow(&self) -> &[$mask] {
                    self.as_slice()
                }
            }

            impl core::borrow::BorrowMut<[$mask]> for $name {
                #[inline(always)]
                fn borrow_mut(&mut self) -> &mut [$mask] {
                    self.as_slice_mut()
                }
            }

            impl From<[$mask; $count]> for $name {
                #[inline(always)]
                fn from(arr: [$mask; $count]) -> $name {
                    $name::from_array(arr)
                }
            }

            impl core::ops::Index<usize> for $name {
                type Output = $mask;

                #[inline(always)]
                fn index(&self, index: usize) -> &$mask {
                    &self.as_array()[index]
                }
            }

            impl core::ops::IndexMut<usize> for $name {
                #[inline(always)]
                fn index_mut(&mut self, index: usize) -> &mut $mask {
                    &mut self.as_array_mut()[index]
                }
            }

            impl From<$name> for [$mask; $count] {
                #[inline(always)]
                fn from(vec: $name) -> [$mask; $count] {
                    vec.to_array()
                }
            }


            impl core::hash::Hash for $name {
                #[inline]
                fn hash<H>(&self, state: &mut H)
                where
                    H: core::hash::Hasher,
                {
                    self.as_array().hash(state)
                }

                #[inline]
                fn hash_slice<H>(data: &[$name], state: &mut H)
                where
                    H: core::hash::Hasher,
                    Self: Sized,
                {
                    // SAFETY: `data` is a slice of [`Self`], and `Self` is just an overaligned,
                    //          array of masks with zero padding. So, it is always valid to get a slice
                    //          of masks from a slice of `Self`s.

                    // The length of the slice is determined by the amount of vectors multiplied
                    // by how many lanes exist in each vector.
                    let len = data.len() * (size_of::<$name>() / size_of::<$mask>());
                    let slice: &[$mask] = unsafe { core::slice::from_raw_parts(data.as_ptr() as *const $mask, len) };

                    slice.hash(state)
                }
            }
        )*
    };
}

pub(self) use mask_vecs;

macro_rules! impl_vecs {
    ($align:literal, $bits:literal;
        $( $(#[$meta:meta])* $vis:vis struct $name:ident<$prim:ty, $count:tt> $(Eq $($eq:lifetime)?)? $(Hash $($hash:lifetime)?)? ;)*
    ) => {
        $(
            self::impl_vec!(
                #[allow(non_camel_case_types)]
                #[repr(C, align($align))]
                #[derive(Debug, Clone, Copy, PartialEq, Default, ::bytemuck::Zeroable, ::bytemuck::Pod)]
                $(#[$meta])*
                #[doc = concat!(
                    "A ", stringify!($bits), "-bit vector type storing ",
                    stringify!($count), " [`", stringify!($prim), "`]s."
                )]
                #[doc = ""]
                #[doc = concat!(
                    "It is aligned to ", stringify!($align), "-byte boundaries, regardless if",
                    " the host system actually supports SIMD with this type."
                )]
                $vis struct $name<$prim, $count>
            );

            const _: () = {
                assert!(size_of::<$name>() == $align);
                assert!(align_of::<$name>() == $align);
            };

            impl $name {
                /// Create a new vector from an array of elements.
                #[inline(always)]
                #[must_use]
                pub const fn from_array(arr: [$prim; $count]) -> $name {
                    muck::cast(arr)
                }

                /// Return an array containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn to_array(self) -> [$prim; $count] {
                    muck::cast(self)
                }

                /// Return an array reference containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_array(&self) -> &[$prim; $count] {
                    muck::cast_ref(self)
                }

                /// Return a mutable array reference containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_array_mut(&mut self) -> &mut [$prim; $count] {
                    muck::cast_mut(self)
                }

                /// Return a slice containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_slice(&self) -> &[$prim] {
                    self.as_array()
                }

                /// Return a mutable slice containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_slice_mut(&mut self) -> &mut [$prim] {
                    self.as_array_mut()
                }

                /// Return whether all the elements within this vector
                /// are valid masks.
                #[inline(always)]
                #[must_use]
                pub const fn is_valid_mask(&self) -> bool {
                    let mut elements = self.as_slice();
                    let mut is_valid = true;

                    while let [e, rest @ ..] = elements {
                        is_valid &= crate::muck::identify_mask(*e).is_some();
                        elements = rest;
                    }

                    is_valid
                }
            }

            impl AsRef<[$prim; $count]> for $name {
                #[inline(always)]
                fn as_ref(&self) -> &[$prim; $count] {
                    self.as_array()
                }
            }

            impl AsMut<[$prim; $count]> for $name {
                #[inline(always)]
                fn as_mut(&mut self) -> &mut [$prim; $count] {
                    self.as_array_mut()
                }
            }

            impl core::borrow::Borrow<[$prim; $count]> for $name {
                #[inline(always)]
                fn borrow(&self) -> &[$prim; $count] {
                    self.as_array()
                }
            }

            impl core::borrow::BorrowMut<[$prim; $count]> for $name {
                #[inline(always)]
                fn borrow_mut(&mut self) -> &mut [$prim; $count] {
                    self.as_array_mut()
                }
            }

            impl AsRef<[$prim]> for $name {
                #[inline(always)]
                fn as_ref(&self) -> &[$prim] {
                    self.as_slice()
                }
            }

            impl AsMut<[$prim]> for $name {
                #[inline(always)]
                fn as_mut(&mut self) -> &mut [$prim] {
                    self.as_slice_mut()
                }
            }

            impl core::borrow::Borrow<[$prim]> for $name {
                #[inline(always)]
                fn borrow(&self) -> &[$prim] {
                    self.as_slice()
                }
            }

            impl core::borrow::BorrowMut<[$prim]> for $name {
                #[inline(always)]
                fn borrow_mut(&mut self) -> &mut [$prim] {
                    self.as_slice_mut()
                }
            }

            impl From<[$prim; $count]> for $name {
                #[inline(always)]
                fn from(arr: [$prim; $count]) -> $name {
                    $name::from_array(arr)
                }
            }

            impl core::ops::Index<usize> for $name {
                type Output = $prim;

                #[inline(always)]
                fn index(&self, index: usize) -> &$prim {
                    &self.as_array()[index]
                }
            }

            impl core::ops::IndexMut<usize> for $name {
                #[inline(always)]
                fn index_mut(&mut self, index: usize) -> &mut $prim {
                    &mut self.as_array_mut()[index]
                }
            }

            impl From<$name> for [$prim; $count] {
                #[inline(always)]
                fn from(vec: $name) -> [$prim; $count] {
                    vec.to_array()
                }
            }

            $(impl $(<$eq>)? Eq for $name {})?

            $(
                impl $(<$hash>)? core::hash::Hash for $name {
                    #[inline]
                    fn hash<H>(&self, state: &mut H)
                    where
                        H: core::hash::Hasher,
                    {
                        self.as_array().hash(state)
                    }

                    #[inline]
                    fn hash_slice<H>(data: &[$name], state: &mut H)
                    where
                        H: core::hash::Hasher,
                        Self: Sized,
                    {
                        let slice: &[$prim] = bytemuck::must_cast_slice(data);

                        slice.hash(state)
                    }
                }
            )?
        )*
    };
}

pub(self) use impl_vecs;

macro_rules! impl_vec {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 1>
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 2>
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
            $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 4>
    ) => {

        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
        );
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 8>
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 16>
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
        );
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 32>
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
        );
    };


    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$prim:ty, 64>
    ) => {
        $(#[$attr])*
        $vis struct $name(
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
            $vis $prim,
        );
    };
}

pub(self) use impl_vec;
