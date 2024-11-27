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
