define_masks! {
    struct m8<8>(i8, u8);
    struct m16<16>(i16, u16);
    struct m32<32>(i32, u32, f32);
    struct m64<64>(i64, u64, f64);
}

macro_rules! define_masks {
    ($(
        $(#[$meta:meta])*
        struct $name:ident<$bits:literal>($repr:ident $(, $rest:ident)* $(,)?);
    )*) => {
        $(
            $(#[$meta])*
            #[allow(non_camel_case_types)]
            #[repr($repr)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            #[doc = concat!(
                "A lane-wise mask type for vectors whose lanes are ",
                stringify!($bits), "-bits wide."
            )]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = ""]
            #[doc = concat!(
                "A valid [`", stringify!($name), "`] is \
                 either all zeros (`false`), or all ones (`true`)."
            )]
            #[doc = ""]
            #[doc = concat!(
                "To construct a [`", stringify!($name), "`] with \
                 any value besides those mentioned above is \
                 *undefined behavior*."
            )]
            pub enum $name {
                /// All bits are set to zero.
                #[default]
                Unset = 0,
                /// All bits are set to one.
                Set = -1,
            }

            impl $name {
                /// Convert a boolean to it's corresponding mask.
                #[inline(always)]
                #[must_use]
                pub const fn from_bool(b: bool) -> $name {
                    match b {
                        true => $name::Set,
                        false => $name::Unset,
                    }
                }

                /// Convert the mask to it's corresponding boolean value.
                #[inline(always)]
                #[must_use]
                pub const fn to_bool(self) -> bool {
                    match self {
                        $name::Set => true,
                        $name::Unset => false,
                    }
                }

                #[doc = concat!(
                    "Create a [`", stringify!($name), "`] \
                     from its integer representation."
                )]
                #[doc = ""]
                #[doc = "# Returns"]
                #[doc = ""]
                #[doc = "Returns `None` if `repr` is invalid."]
                #[inline(always)]
                #[must_use]
                pub const fn try_from_repr(repr: $repr) -> Option<$name> {
                    match repr {
                        -1 => Some($name::Set),
                        0 => Some($name::Unset),
                        _ => None,
                    }
                }

                #[doc = concat!(
                    "Create a [`", stringify!($name), "`] \
                     from its integer representation."
                )]
                #[doc = ""]
                #[doc = "# Panics"]
                #[doc = ""]
                #[doc = "Panics if `repr` is invalid."]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const fn from_repr(repr: $repr) -> $name {
                    match $name::try_from_repr(repr) {
                        Some(mask) => mask,
                        None => panic!("invalid mask"),
                    }
                }

                #[doc = concat!(
                    "Create a [`", stringify!($name), "`] \
                     from its integer representation, \
                     without any checks."
                )]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = ""]
                #[doc = "The caller must ensure that `repr` is \
                         all zeros, or all ones. Failure to do \
                         so is *undefined behavior*."]
                #[inline(always)]
                #[must_use]
                #[track_caller]
                pub const unsafe fn from_repr_unchecked(repr: $repr) -> $name {
                    match $name::try_from_repr(repr) {
                        Some(mask) => mask,
                        None if cfg!(debug_assertions) => panic!("undefined behavior: mask is invalid"),
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }
            }

            unsafe impl bytemuck::Zeroable for $name {}
            unsafe impl bytemuck::NoUninit for $name {}
            unsafe impl bytemuck::CheckedBitPattern for $name {
                type Bits = $repr;

                #[inline(always)]
                fn is_valid_bit_pattern(bits: &$repr) -> bool {
                    $name::try_from_repr(*bits).is_some()
                }
            }
            unsafe impl bytemuck::Contiguous for $name {
                type Int = $repr;

                const MIN_VALUE: $repr = -1;
                const MAX_VALUE: $repr = 0;
            }

            impl From<bool> for $name {
                #[inline]
                fn from(b: bool) -> $name {
                    $name::from_bool(b)
                }
            }

            impl From<$name> for bool {
                #[inline]
                fn from(m: $name) -> bool {
                    m.to_bool()
                }
            }
        )*
    };
}

pub(self) use define_masks;
