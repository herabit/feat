#![allow(unused_imports, unused_macros)]

macro_rules! seal {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::sealed::Sealed for $ty {}
        )*
    };
}

pub(crate) use seal;

macro_rules! seal_tuple {
    ($($ty:ident),* $(,)?) => {
        impl<$($ty: Sealed),*> Sealed for ($($ty,)*) {}
    };
}

pub(crate) use seal_tuple;

macro_rules! seal_nonzero {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::sealed::Sealed for core::num::NonZero<$ty> {}
        )*
    };
}

pub(crate) use seal_nonzero;

macro_rules! impl_marker_arch {
    (
        $(#[cfg($predicate:meta)])*
        $(unsafe $($unsafe:lifetime)?)?
        impl $trait:ident for $arch:ident :: {}
    ) => {};
    (
        $(#[cfg($predicate:meta)])*
        $(unsafe $($unsafe:lifetime)?)?
        impl $trait:ident for $arch:ident :: {
            $first:ident
            $(, $remaining:ident)*
            $(,)?
        }
    ) => {
        $(#[cfg($predicate)])*
        $(unsafe $($unsafe)?)? impl $trait for ::core::arch::$arch::$first {}

        $(#[cfg($predicate)])*
        $crate::macros::impl_marker_arch!(
            $(#[cfg($predicate)])*
            $(unsafe $($unsafe)?)? impl $trait for $arch:: { $($remaining),* }
        );
    };
}

pub(crate) use impl_marker_arch;
