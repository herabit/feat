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
        concat!(
            $crate::macros::vector_docs!(@consonant $bits), " ",
            stringify!($bits), "-bit vector containing ",
            stringify!($lanes), " [`", stringify!($scalar), "`]",
            $crate::macros::vector_docs!(@plurality $lanes), ".",
        )
    };
}

pub(crate) use vector_docs;

macro_rules! vector_base {
    ($name:ident $(/ $half:ident)?: [$scalar:ident; $lanes:tt], $bits:tt) => {
        // Layout checks
        const _: () = {
            #[allow(unused)]
            use ::core::mem::{ size_of, align_of };
            use ::core::assert;

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
            #[allow(unused)]
            use ::core::mem::size_of;

            #[allow(unused)]
            use ::core::{ panic, concat, stringify };

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
            #[inline(always)]
            #[must_use]
            pub const fn from_array(array: [$scalar; $lanes]) -> $name {
                // SAFETY: We know they have the same in-memory representation.
                unsafe { ::core::mem::transmute(array) }
            }

            /// Create a new vector by reading a slice of scalars.
            ///
            /// # Panics
            ///
            /// Panics if the slice isn't large enough.
            #[inline(always)]
            #[must_use]
            pub const fn from_slice(slice: &[$scalar]) -> $name {
                if slice.len() >= $lanes {
                    $name::from_array(unsafe { (slice as *const [$scalar] as *const [$scalar; $lanes]).read() })
                } else {
                    panic!("`slice` is not long enough")
                }
            }

            /// Get an array of scalars from a vector.
            #[inline(always)]
            #[must_use]
            pub const fn to_array(self) -> [$scalar; $lanes] {
                // SAFETY: We know they have the same in-memory representation.
                unsafe { ::core::mem::transmute(self) }
            }

            /// Get a reference to the inner array of scalars.
            #[inline(always)]
            #[must_use]
            pub const fn as_array(&self) -> &[$scalar; $lanes] {
                // SAFETY: We know they have the same layout and
                //         we check at compile time that this type is
                //         properly aligned.
                unsafe { &*(self as *const $name as *const [$scalar; $lanes]) }
            }

            /// Get a mutable reference to the inner array of scalars.
            #[inline(always)]
            #[must_use]
            pub const fn as_array_mut(&mut self) -> &mut [$scalar; $lanes] {
                // SAFETY: We know they have the same layout and
                //         we check at compile time that this type is
                //         properly aligned.
                unsafe { &mut *(self as *mut $name as *mut [$scalar; $lanes]) }
            }

            /// Get a reference to the inner slice of scalars.
            #[inline(always)]
            #[must_use]
            pub const fn as_slice(&self) -> &[$scalar] {
                self.as_array()
            }

            /// Get a mutable reference to the inner slice of scalars.
            #[inline(always)]
            #[must_use]
            pub const fn as_slice_mut(&mut self) -> &mut [$scalar] {
                self.as_array_mut()
            }
        }

        impl core::borrow::Borrow<[$scalar; $lanes]> for $name {
            #[inline(always)]
            fn borrow(&self) -> &[$scalar; $lanes] {
                self.as_array()
            }
        }

        impl core::borrow::BorrowMut<[$scalar; $lanes]> for $name {
            #[inline(always)]
            fn borrow_mut(&mut self) -> &mut [$scalar; $lanes] {
                self.as_array_mut()
            }
        }

        impl core::borrow::Borrow<[$scalar]> for $name {
            #[inline(always)]
            fn borrow(&self) -> &[$scalar] {
                self.as_slice()
            }
        }

        impl core::borrow::BorrowMut<[$scalar]> for $name {
            #[inline(always)]
            fn borrow_mut(&mut self) -> &mut [$scalar] {
                self.as_slice_mut()
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
            use core::assert;

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

macro_rules! mask_type {
    () => {};
}

pub(crate) use mask_type;
