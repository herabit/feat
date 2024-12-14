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
