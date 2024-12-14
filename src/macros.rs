#![allow(unused_imports, unused_macros)]

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

macro_rules! create_tuple {
    ($dol:tt;
    $(
        ($($field:ident),* $(,)?) $(,)?

    )*) => {
        macro_rules! impl_marker_tuple {
            (
                $dol (unsafe $dol($dol unsafe:lifetime)?)?
                impl $dol trait:ident
            ) => {
                $(
                    $dol(unsafe $dol($dol unsafe)?)?
                    impl <$($field: $dol trait),*>
                    $dol trait for ($($field,)*) {}
                )*
            };
        }

        pub(crate) use impl_marker_tuple;
    };
}

create_tuple!($;
    (),
    (T0),
    (T0, T1),
    (T0, T1, T2),
    (T0, T1, T2, T3),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5, T6, T7),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),
);
