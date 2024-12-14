#![allow(unused_macros, unused_imports)]

macro_rules! arch_import {
    {$(
        $(#[cfg($pred:meta)])+
        $(#[$meta:meta])*
        $vis:vis use $arch:ident::{ $($import:ident $(as $alias:ident)? ),* };
    )*} => {
        $(
            $(#[cfg($pred)])+
            $(#[$meta])*
            #[allow(unused)]
            $vis use ::core::arch::$arch::{
                $(
                    $import $(as $alias)?
                ),*
            };
        )*
    };
}

pub(crate) use arch_import;

macro_rules! select {
    ($(,)?) => { compile_error!("unsupported platform") };
    ($(,)?  _ => { $($item:item)* } $(,)?) => { $($item)* };
    ($(,)? _ => $expr:expr $(,)?) => { $expr };
    ($(,)? _ => $item:item $(,)?) => { $item };

    ($(,)?
        $(cfg($cfg:meta))|+ => {
            $($item:item)*
        }
        $($tt:tt)*
    ) => {
        $crate::macros::select!{
            @acc(any($($cfg),*))

            $([ $item ])*

            {
                $crate::macros::select!{
                    $($tt)*
                }
            }
        }
    };

    ($(,)?
        $(cfg($cfg:meta))|+ => $expr:expr
        $(, $($tt:tt)*)?
    ) => {
        match () {
            #[cfg(any($($cfg),*))] _ => $expr,
            #[cfg(not(any($($cfg),*)))] _ => { $crate::macros::select!($($($tt)*)?) },
        }
    };

    ($(,)?
        $(cfg($cfg:meta))|+ => $item:item
        $($tt:tt)*
    ) => {
        $crate::macros::select!{
            @acc(any($($cfg),*))

            [ $item ]

            { $crate::macros::select!{
                $($tt)*
            } }
        }
    };

    (
        @acc($cfg:meta) $([ $($enable:tt)* ])* $({ $($disable:tt)* })*
    ) => {
        $(
            #[cfg($cfg)]
            $($enable)*
        )*

        $(
            #[cfg(not($cfg))]
            $($disable)*
        )*
    };
}

pub(crate) use select;

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
            $crate::macros::vector_docs!(@plurality $lanes),
            "."
        )
    };
}

#[allow(unused)]
pub(crate) use vector_docs;

macro_rules! vector_base {
    ($name:ident $(/ $half:ident)?: [$scalar:ident; $lanes:tt]) => {
        const _: () = {
            #[allow(unused)]
            use ::core::mem::{ size_of, align_of };

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

        impl $name {
            #[doc = concat!(
                "Construct a new [`", stringify!($name), "`] from a \
                `[", stringify!($scalar), "; ", stringify!($lanes), "]`."
            )]
            #[inline(always)]
            #[must_use]
            pub const fn from_array(array: [$scalar; $lanes]) -> $name {
                // SAFETY: We know that `array` and `name` are the same size
                //         and that vectors are just arrays of their scalar type.
                unsafe { ::core::mem::transmute(array) }
            }
        }
    };
}

#[allow(unused)]
pub(crate) use vector_base;
