/// Define a collection of type level proofs for the existence of some, "fundamental"
/// CPU feature.
macro_rules! tokens {
    ($(
        $(#[$meta:meta])*
        $name:ident = $target_features:literal;
    )*) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(transparent)]
            pub struct $name(());

            impl $name {
                #[doc = concat!("Create a new [`", stringify!($name), "`] without any checks.")]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = ""]
                #[doc = concat!(
                    "The caller must ensure that the feature(s) \
                     required by [`", stringify!($name), "`] are present \
                     before calling this function."
                )]
                #[doc = ""]
                #[doc = "Failure to ensure this is condidered *Undefined Behavior*."]
                #[inline(always)]
                #[must_use]
                #[allow(unused_unsafe)]
                pub const unsafe fn new_unchecked() -> $name {
                    unsafe { $name(()) }
                }

                #[doc = concat!(
                    "Execute a closure where the feature(s) \
                    required by [`", stringify!($name), "`] are known to \
                    be present. \
                    This allows for further optimization opportunities."
                )]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "For best results, the given closure should be \
                         marked as `#[inline(always)]`."]
                #[inline(always)]
                #[must_use]
                pub fn run<F: $crate::CallOnce>(self, f: F) -> F::Output {
                    #[inline]
                    #[target_feature(enable = $target_features)]
                    unsafe fn execute<F: $crate::CallOnce>(f: F) -> F::Output {
                        f.call_once()
                    }

                    // SAFETY: We have an instance of this token, so the target
                    //         features it implies are known to be present, allowing
                    //         for codegen that is dependent upon them.
                    unsafe { execute::<F>(f) }
                }
            }
        )*
    };
}

pub(crate) use tokens;

/// Maps a given type to the idenfier of the type needed for it to perform casts.
macro_rules! param_ty {
    (&[$ty:ty]) => {
        cast_slice
    };
    (&mut [$ty:ty]) => {
        cast_slice_mut
    };
    (&$ty:ty) => {
        cast_ref
    };
    (&mut $ty:ty) => {
        cast_mut
    };
    (*const $ty:ty) => {
        <*const $ty>::cast
    };
    (*mut $ty:ty) => {
        <*mut $ty>::cast
    };
    ($ty:ty) => {
        cast
    };
    () => {
        $crate::macros::param_ty!(())
    };
}

pub(crate) use param_ty;

/// Constructs a delegated method.
macro_rules! delegate {
    ($(
        $(#[$meta:meta])*
        $(unsafe $($unsafe:lifetime)?)?
        fn $name:ident
            $(< $(const $gen:ident: $gen_ty:ty),* $(,)? >)?
        (
            $($arg:ident: { $($arg_ty:tt)+ }),* $(,)?
        ) $(-> { $($ret_ty:tt)+ })? = $intrin:ident;
    )*) => {
        $(
            $(#[$meta])*
            #[inline(always)]
            #[must_use]
            // #[doc(alias = stringify!($intrin))]
            #[doc = ""]
            #[doc = "# Intrinsic"]
            #[doc = ""]
            #[doc = concat!(
                "This method utilizes the [`",
                stringify!($intrin),
                "`] intrinsic."
            )]
            pub $(unsafe $($unsafe)?)? fn $name $(< $(const $gen: $gen_ty),*  >)?
                (self, $($arg: $($arg_ty)*),* ) $(-> $($ret_ty)*)?
            {
                let result = unsafe { $intrin $(::<$($gen),*>)?($($crate::macros::param_ty!($($arg_ty)*)($arg)),*) };
                $crate::macros::param_ty!($($($ret_ty)*)?)(result)
            }
        )*
    };
}

pub(crate) use delegate;
