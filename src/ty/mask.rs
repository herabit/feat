impl_lane_mask! {
    struct m8<8>(i8, u8);
    struct m16<16>(i16, u16);
    struct m32<32>(i32, u32, f32);
    struct m64<64>(i64, u64, f64);
}

macro_rules! impl_lane_mask {
    ($(
        $(#[$meta:meta])*
        struct $name:ident<$bits:literal>($repr:ident $(, $rest:ident)* $(,)?);
    )*) => {
        $(
            $(#[$meta])*
            #[allow(non_camel_case_types)]
            #[repr($repr)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            #[derive(bytemuck::Zeroable, bytemuck::NoUninit, bytemuck::CheckedBitPattern, bytemuck::Contiguous)]
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
                /// `false`: all zeros.
                #[default]
                False = 0,
                /// `true`: all ones.
                True = -1,
            }

            impl $name {
                /// Convert a boolean to it's corresponding mask.
                #[inline(always)]
                #[must_use]
                pub const fn from_bool(b: bool) -> $name {
                    match b {
                        true => $name::True,
                        false => $name::False,
                    }
                }

                /// Convert the mask to it's corresponding boolean value.
                #[inline(always)]
                #[must_use]
                pub const fn to_bool(self) -> bool {
                    match self {
                        $name::True => true,
                        $name::False => false,
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
                        -1 => Some($name::True),
                        0 => Some($name::False),
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

pub(self) use impl_lane_mask;
