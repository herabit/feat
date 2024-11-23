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

macro_rules! impl_vecs {
    ($align:literal, $bits:literal;
        $( $(#[$attr:meta])* $vis:vis struct $name:ident<$prim:ty, $count:tt> $(Eq $($eq:lifetime)?)? $(Hash $($hash:lifetime)?)? ;)*
    ) => {
        $(
            self::impl_vec!(
                #[allow(non_camel_case_types)]
                #[repr(C, align($align))]
                #[derive(Debug, Clone, Copy, PartialEq, Default, ::bytemuck::Zeroable, ::bytemuck::Pod)]
                $(#[$attr])*
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
                assert!(size_of::<$name>() == $align, "unexpected size");
                assert!(align_of::<$name>() == $align, "unexpected alignment");
            };

            impl $name {
                /// Create a new vector from an array of elements.
                #[inline(always)]
                #[must_use]
                pub const fn from_array(arr: [$prim; $count]) -> $name {
                    bytemuck::must_cast(arr)
                }

                /// Return an array containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn to_array(self) -> [$prim; $count] {
                    bytemuck::must_cast(self)
                }

                /// Return an array reference containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub const fn as_array(&self) -> &[$prim; $count] {
                    bytemuck::must_cast_ref(self)
                }

                /// Return a mutable array reference containing all elements within the vector.
                #[inline(always)]
                #[must_use]
                pub fn as_array_mut(&mut self) -> &mut [$prim; $count] {
                    bytemuck::must_cast_mut(self)
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
                pub fn as_slice_mut(&mut self) -> &mut [$prim] {
                    self.as_array_mut()
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
