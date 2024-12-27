#![allow(unused_macros, unused_imports)]

macro_rules! vector_docs {
    (@plurality 1) => { "" };
    (@plurality $tt:tt) => { "s" };

    (@consonant 8) => { "An" };
    (@consonant $tt:tt) => { "A" };

    (
        [$scalar:ident; $lanes:tt]: $bits:tt
        $(,)?
    ) => {
        ::core::concat!(
            $crate::macros::vector_docs!(@consonant $bits), " ",
            ::core::stringify!($bits), "-bit vector containing ",
            ::core::stringify!($lanes), " [`", ::core::stringify!($scalar), "`]",
            $crate::macros::vector_docs!(@plurality $lanes), ".",
        )
    };
}

pub(crate) use vector_docs;

macro_rules! vector_base {
    ($name:ident $(/ $half:ident)? : [$scalar:ident; $lanes:tt], $bits:tt) => {
        // Layout checks
        const _: () = {
            assert!(
                size_of::<$name>() == size_of::<[$scalar; $lanes]>(),
                "vector does not have the same size as its corresponding array"
            );

            assert!(
                align_of::<$name>() >= align_of::<[$scalar; $lanes]>(),
                "vector must not have a lower alignment than its corresponding array"
            );

            $(
                assert!(
                    size_of::<$name>() == size_of::<[$half; 2]>(),
                    "vector is not double the size of the vector half its length"
                );

                assert!(
                    align_of::<$name>() >= align_of::<$half>(),
                    "the alignment of a vector must not be less than the alignment \
                     of the vector half its length"
                );
            )?
        };

        // Ensure that the vector bit width is correct.
        const _: () = {
            match size_of::<$name>().checked_mul(8) {
                Some($bits) => {},
                _ => panic!("{}", concat!(
                    "The bitwidth of ", stringify!($name), " is not ",
                    stringify!($bits), " bits.",
                )),
            }
        };

        impl $name {
            /// Create a new vector from an array of scalars.
            #[inline]
            #[must_use]
            pub const fn from_array(array: [$scalar; $lanes]) -> $name {
                // SAFETY: We know they have the same in-memory representation.
                unsafe { ::core::mem::transmute(array) }
            }

            $(
                /// Create a new vector by joining two halves of a vector.
                #[inline]
                #[must_use]
                pub const fn from_halves(a: $half, b: $half) -> $name {
                    // SAFETY: We know they have the same in-memory representation.
                    unsafe { ::core::mem::transmute([a, b]) }
                }
            )?

            /// Create a new vector by reading a slice of scalars.
            ///
            /// # Panics
            ///
            /// Panics if the slice isn't large enough.
            #[inline]
            #[must_use]
            pub const fn from_slice(slice: &[$scalar]) -> $name {
                if slice.len() >= $lanes {
                    $name::from_array(unsafe { (slice as *const [$scalar] as *const [$scalar; $lanes]).read() })
                } else {
                    panic!("`slice` is not long enough")
                }
            }

            /// Get an array of scalars from a vector.
            #[inline]
            #[must_use]
            pub const fn to_array(self) -> [$scalar; $lanes] {
                // SAFETY: We know they have the same in-memory representation.
                unsafe { ::core::mem::transmute(self) }
            }

            $(
                /// Split this vector into an array of halves.
                #[inline]
                #[must_use]
                pub const fn to_halves(self) -> [$half; 2] {
                    // SAFETY: We know they have the same in-memory representation.
                    unsafe { ::core::mem::transmute(self) }
                }

                /// Split this vector into two halves.
                #[inline]
                #[must_use]
                pub const fn split(self) -> ($half, $half) {
                    let [a, b] = self.to_halves();

                    (a, b)
                }
            )?

            /// Get a reference to the inner array of scalars.
            #[inline]
            #[must_use]
            pub const fn as_array(&self) -> &[$scalar; $lanes] {
                // SAFETY: We know they have the same layout and
                //         we check at compile time that this type is
                //         properly aligned.
                unsafe { &*(self as *const $name as *const [$scalar; $lanes]) }
            }

            $(
                /// Get a reference to this vector's halves.
                #[inline]
                #[must_use]
                pub const fn as_halves(&self) -> &[$half; 2] {
                    // SAFETY: We do compile time checks to ensure they have
                    //         the same layout and that this type
                    //         is properly aligned.
                    unsafe { &*(self as *const $name as *const [$half; 2]) }
                }
            )?

            /// Get a mutable reference to the inner array of scalars.
            #[inline]
            #[must_use]
            pub const fn as_array_mut(&mut self) -> &mut [$scalar; $lanes] {
                // SAFETY: We know they have the same layout and
                //         we check at compile time that this type is
                //         properly aligned.
                unsafe { &mut *(self as *mut $name as *mut [$scalar; $lanes]) }
            }

            $(
                /// Get a mutable reference to this vector's halves.
                #[inline]
                #[must_use]
                pub const fn as_halves_mut(&mut self) -> &mut [$half; 2] {
                    // SAFETY: We know that this vector is properly
                    //         aligned for the vector half it's size,
                    //         and that this vector has the same
                    //         memory layout as an array of two halves
                    //         of it.
                    unsafe { &mut *(self as *mut $name as *mut [$half; 2]) }
                }
            )?

            /// Get a reference to the inner slice of scalars.
            #[inline]
            #[must_use]
            pub const fn as_slice(&self) -> &[$scalar] {
                self.as_array()
            }

            /// Get a mutable reference to the inner slice of scalars.
            #[inline]
            #[must_use]
            pub const fn as_slice_mut(&mut self) -> &mut [$scalar] {
                self.as_array_mut()
            }
        }

        impl ::core::borrow::Borrow<[$scalar; $lanes]> for $name {
            #[inline]
            fn borrow(&self) -> &[$scalar; $lanes] {
                self.as_array()
            }
        }

        impl ::core::borrow::Borrow<[$scalar]> for $name {
            #[inline]
            fn borrow(&self) -> &[$scalar] {
                self.as_slice()
            }
        }

        $(
            impl ::core::borrow::Borrow<[$half; 2]> for $name {
                #[inline]
                fn borrow(&self) -> &[$half; 2] {
                    self.as_halves()
                }
            }

            impl ::core::borrow::Borrow<[$half]> for $name {
                #[inline]
                fn borrow(&self) -> &[$half] {
                    self.as_halves()
                }
            }
        )?

        impl ::core::borrow::BorrowMut<[$scalar; $lanes]> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut [$scalar; $lanes] {
                self.as_array_mut()
            }
        }


        impl ::core::borrow::BorrowMut<[$scalar]> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut [$scalar] {
                self.as_slice_mut()
            }
        }


        $(
            impl ::core::borrow::BorrowMut<[$half; 2]> for $name {
                #[inline]
                fn borrow_mut(&mut self) -> &mut [$half; 2] {
                    self.as_halves_mut()
                }
            }

            impl ::core::borrow::BorrowMut<[$half]> for $name {
                #[inline]
                fn borrow_mut(&mut self) -> &mut [$half] {
                    self.as_halves_mut()
                }
            }
        )?

        impl AsRef<[$scalar; $lanes]> for $name {
            #[inline]
            fn as_ref(&self) -> &[$scalar; $lanes] {
                self.as_array()
            }
        }

        impl AsRef<[$scalar]> for $name {
            #[inline]
            fn as_ref(&self) -> &[$scalar] {
                self.as_slice()
            }
        }

        $(
            impl AsRef<[$half; 2]> for $name {
                #[inline]
                fn as_ref(&self) -> &[$half; 2] {
                    self.as_halves()
                }
            }

            impl AsRef<[$half]> for $name {
                #[inline]
                fn as_ref(&self) -> &[$half] {
                    self.as_halves()
                }
            }
        )?

        impl AsMut<[$scalar; $lanes]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [$scalar; $lanes] {
                self.as_array_mut()
            }
        }

        impl AsMut<[$scalar]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [$scalar] {
                self.as_slice_mut()
            }
        }

        $(
            impl AsMut<[$half; 2]> for $name {
                #[inline]
                fn as_mut(&mut self) -> &mut [$half; 2] {
                    self.as_halves_mut()
                }
            }

            impl AsMut<[$half]> for $name {
                #[inline]
                fn as_mut(&mut self) -> &mut [$half] {
                    self.as_halves_mut()
                }
            }
        )?

        impl IntoIterator for $name {
            type Item = $scalar;
            type IntoIter = ::core::array::IntoIter<$scalar, $lanes>;

            #[inline]
            fn into_iter(self) -> ::core::array::IntoIter<$scalar, $lanes> {
                self.to_array().into_iter()
            }
        }

        impl<'a> IntoIterator for &'a $name {
            type Item = &'a $scalar;
            type IntoIter = ::core::slice::Iter<'a, $scalar>;

            #[inline]
            fn into_iter(self) -> ::core::slice::Iter<'a, $scalar> {
                self.as_slice().iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $name {
            type Item = &'a mut $scalar;
            type IntoIter = ::core::slice::IterMut<'a, $scalar>;
            #[inline]
            fn into_iter(self) -> ::core::slice::IterMut<'a, $scalar> {
                self.as_slice_mut().iter_mut()
            }
        }

        impl ::core::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut debug = &mut f.debug_tuple("");

                for value in self.as_array() {
                    debug = debug.field(value);
                }

                debug.finish()
            }
        }
    };
}

pub(crate) use vector_base;

macro_rules! vector {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            arr: [$scalar:ident; 1],
            bits: $bits:tt

            $(,
                $(#[doc = $docs:tt])*
                $(#[cfg($cfg_pred:meta)])+
                $field:ident: $arch:ident :: $type:ident
            )*

            $(,)?
        }
    ) => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = $crate::macros::vector_docs!(
            [$scalar; 1]: $bits
        )]
        $(#[$meta])*
        $vis struct $name {
            $(
                $(#[cfg($cfg_pred)])*
                $(#[doc = $docs])*
                pub(crate) $field: ::core::arch::$arch::$type,
            )*

            #[cfg(not(any(
                $(all($($cfg_pred),*)),*
            )))]
            pub(crate) scalar: $scalar,
        }

        $crate::macros::vector_base!($name: [$scalar; 1], $bits);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            arr: [$scalar:ident; $lanes:tt],
            bits: $bits:tt,
            half: $half:ident

            $(,
                $(#[doc = $docs:tt])*
                $(#[cfg($cfg_pred:meta)])+
                $field:ident: $arch:ident :: $type:ident
            )*

            $(,)?
        }
    ) => {
        const _: () = {
            assert!($lanes != 1, "vectors with one lane have no vectors half their size");
        };

        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = $crate::macros::vector_docs!(
            [$scalar; $lanes]: $bits
        )]
        $(#[$meta])*
        $vis struct $name {
            $(
                $(#[cfg($cfg_pred)])*
                $(#[doc = $docs])*
                pub(crate) $field: ::core::arch::$arch::$type,
            )*

            #[cfg(not(any($(
                all($($cfg_pred),*)
            ),*)))]
            pub(crate) halves: [$half; 2],
        }

        $(
            $(#[cfg($cfg_pred)])*
            impl From<::core::arch::$arch::$type> for $name {
                #[inline]
                fn from(value: ::core::arch::$arch::$type) -> $name {
                    $name { $field: value }
                }
            }

            $(#[cfg($cfg_pred)])*
            impl From<$name> for ::core::arch::$arch::$type {
                #[inline]
                fn from(value: $name) -> ::core::arch::$arch::$type {
                    value.$field
                }
            }
        )*

        $crate::macros::vector_base!($name/$half: [$scalar; $lanes], $bits);
    };
}

pub(crate) use vector;

macro_rules! vectors {
    ($(
        $(#[$meta:meta])*
        $vis:vis struct $name:ident { $($body:tt)* }
    )*) => {
        $(
            $crate::macros::vector!(
                $(#[$meta])*
                $vis struct $name {
                    $($body)*
                }
            );
        )*
    };
}

pub(crate) use vectors;

macro_rules! exports {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis mod $name:ident $delim:tt
        )*
    ) => {
        $(
            $(#[$attr])*
            $vis mod $name $delim
        )*

        pub(crate) mod exports {
            #![allow(unused_imports)]

            $(
                #[doc(inline)]
                $vis use super::$name::*;
            )*
        }
    };
}

pub(crate) use exports;

macro_rules! mask_docs {
    () => {};
}

pub(crate) use mask_docs;

macro_rules! mask_type {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            repr: $repr:ident,
            bits: $bits:tt

            $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr($repr)]
        $vis enum $name {
            All = -1,
            None = 0,
        }
    };
}

pub(crate) use mask_type;

macro_rules! masks {
    () => {};
}

pub(crate) use masks;
