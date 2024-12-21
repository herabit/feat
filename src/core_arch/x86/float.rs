use crate::util::string::{starts_with, strip_prefix};
use core::fmt;

#[rustfmt::skip]
use super::raw::{
    _CMP_EQ_OQ, _CMP_EQ_OS, _CMP_EQ_UQ, _CMP_EQ_US,
    _CMP_NEQ_OQ, _CMP_NEQ_OS, _CMP_NEQ_UQ, _CMP_NEQ_US,
    
    _CMP_GE_OQ, _CMP_GE_OS, 
    _CMP_GT_OQ, _CMP_GT_OS, 
    _CMP_LE_OQ, _CMP_LE_OS,
    _CMP_LT_OQ, _CMP_LT_OS,
    _CMP_NGE_UQ, _CMP_NGE_US,
    _CMP_NGT_UQ, _CMP_NGT_US,
    _CMP_NLE_UQ, _CMP_NLE_US,
    _CMP_NLT_UQ, _CMP_NLT_US,
    _CMP_ORD_Q, _CMP_ORD_S,
    _CMP_UNORD_Q, _CMP_UNORD_S,

    
    _CMP_FALSE_OQ, _CMP_FALSE_OS,
    _CMP_TRUE_UQ, _CMP_TRUE_US,
};

macro_rules! define {
    (

        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(
                #[doc = $pred_doc:tt]
                $pred_name:ident = $raw_name:ident
            ),*
            $(,)?
        }
    ) => {
            $(#[$attr])*
            #[repr(i32)]
            $vis enum $name {
                $(
                    #[doc = $pred_doc]
                    #[doc = ""]
                    #[doc = concat!(
                        "Maps to [`", stringify!($raw_name), "`]."
                    )]
                    $pred_name = ($raw_name as _),
                )*
            }

            impl $name {
                /// Get a description of this AVX floating point comparison
                /// predicate.
                #[inline]
                #[must_use]
                pub const fn description(self) -> &'static str {
                    match self {
                        $(
                            $name::$pred_name => $pred_doc,
                        )*
                    }
                }

                /// Get the name of this AVX floating point comparison predicate.
                #[inline]
                #[must_use]
                pub const fn name(self) -> &'static str {
                    match self {
                        $(
                            $name::$pred_name => stringify!($name),
                        )*
                    }
                }

                /// Get the name of this AVX floating point comparison predicate
                /// used in C intrinsics.
                #[inline]
                #[must_use]
                pub const fn c_name(self) -> &'static str {
                    // We're doing this in a weird way to help anyone doing string comparisons.
                    match self {
                        $(
                            $name::$pred_name => const {
                                let name = stringify!($raw_name);

                                assert!(starts_with("_CMP_", name));

                                name
                            },
                        )*
                    }
                }

                /// Get the name of this AVX floating point comparison predicate
                /// used in C intrinsics without the `_CMP_` prefix.
                #[inline]
                #[must_use]
                pub const fn c_name_stripped(self) -> &'static str {
                    // We're doing this in a weird way to help anyone doing string comparisons.
                    match self {
                        $(
                            $name::$pred_name => const {
                                match strip_prefix("_CMP_", $name::$pred_name.c_name()) {
                                    Some(name) => name,
                                    None => unreachable!(),
                                }
                            }
                        )*
                    }
                }

                /// Create an AVX floating point predicate from an [`i32`].
                #[inline]
                #[must_use]
                pub const fn from_i32(i: i32) -> Option<$name> {
                    match i {
                        $(
                            $raw_name => Some($name::$pred_name),
                        )*
                        // We do this so that we can ensure we've
                        // covered all predicates.
                        ..0 | 32.. => None,
                    }
                }
            }
    };
}

const RESULT_BIT: i32 = 0x04;
const ORDER_BIT: i32 = 0x08;
const SIGNAL_BIT: i32 = 0x10;

// I feel like it is better here to use C-style enum names
// as, it is significantly easier for me to read.
define! {
    /// An AVX Floating Point Predicate.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Cmp {
        /// Equal (ordered, quiet)
        EQ     = _CMP_EQ_OQ,
        /// Equal (ordered, loud)
        EQ_LOUD = _CMP_EQ_OS,
        /// Equal (unordered, quiet)
        EQ_UNORD = _CMP_EQ_UQ,
        /// Equal (unordered, loud)
        EQ_UNORD_LOUD = _CMP_EQ_US,

        /// Not-equal (ordered, quiet)
        NEQ = _CMP_NEQ_OQ,
        /// Not-equal (ordered, loud)
        NEQ_LOUD = _CMP_NEQ_OS,
        /// Not-equal (unordered, quiet)
        NEQ_UNORD = _CMP_NEQ_UQ,
        /// Not-equal (unordered, loud)
        NEQ_UNORD_LOUD = _CMP_NEQ_US,

        /// Greater-than-or-equal (ordered, quiet)
        GE = _CMP_GE_OQ,
        /// Greater than-or-equal (ordered, loud)
        GE_LOUD = _CMP_GE_OS,

        /// Not-greater-than-or-equal (unordered, quiet)
        NGE = _CMP_NGE_UQ,
        /// Not-greater-than-or-equal (unordered, loud)
        NGE_LOUD = _CMP_NGE_US,

        /// Greater-than (ordered, quiet)
        GT = _CMP_GT_OQ,
        /// Greater-than (ordered, loud)
        GT_LOUD = _CMP_GT_OS,

        /// Not-greater-than (unordered, quiet)
        NGT = _CMP_NGT_UQ,
        /// Not-greater-than (unordered, loud)
        NGT_LOUD = _CMP_NGT_US,

        /// Less-than-or-equal (ordered, quiet)
        LE = _CMP_LE_OQ,
        /// Less-than-or-equal (ordered, loud)
        LE_LOUD = _CMP_LE_OS,

        /// Not-less-than-or-equal (unordered, quiet)
        NLE = _CMP_NLE_UQ,
        /// Not-less-than-or-equal (unordered, loud)
        NLE_LOUD = _CMP_NLE_US,

        /// Less-than (ordered, quiet)
        LT = _CMP_LT_OQ,
        /// Less-than (ordered, loud)
        LT_LOUD = _CMP_LT_OS,

        /// Not-less-than (unordered, quiet)
        NLT = _CMP_NLT_UQ,
        /// Not-less-than (unordered, loud)
        NLT_LOUD = _CMP_NLT_US,

        /// True (unordered, quiet)
        TRUE = _CMP_TRUE_UQ,
        /// True (unordered, loud)
        TRUE_LOUD = _CMP_TRUE_US,

        /// False (ordered, quiet)
        FALSE = _CMP_FALSE_OQ,
        /// False (ordered, loud)
        FALSE_LOUD = _CMP_FALSE_OS,

        /// Ordered (quiet)
        ORD = _CMP_ORD_Q,
        /// Ordered (loud)
        ORD_LOUD = _CMP_ORD_S,

        /// Unordered (quiet)
        UNORD = _CMP_UNORD_Q,
        /// Unordered (loud)
        UNORD_LOUD = _CMP_UNORD_S,
    }
}

impl Cmp {
    /// Get the noise level of this predicate.
    #[inline]
    #[must_use]
    pub const fn noise(self) -> Noise {
        match self {
            Cmp::EQ
            | Cmp::EQ_UNORD
            | Cmp::NEQ
            | Cmp::NEQ_UNORD
            | Cmp::GE
            | Cmp::NGE
            | Cmp::GT
            | Cmp::NGT
            | Cmp::LE
            | Cmp::NLE
            | Cmp::LT
            | Cmp::NLT
            | Cmp::TRUE
            | Cmp::FALSE
            | Cmp::ORD
            | Cmp::UNORD => Noise::Quiet,

            Cmp::EQ_LOUD
            | Cmp::EQ_UNORD_LOUD
            | Cmp::NEQ_LOUD
            | Cmp::NEQ_UNORD_LOUD
            | Cmp::GE_LOUD
            | Cmp::NGE_LOUD
            | Cmp::GT_LOUD
            | Cmp::NGT_LOUD
            | Cmp::LE_LOUD
            | Cmp::NLE_LOUD
            | Cmp::LT_LOUD
            | Cmp::NLT_LOUD
            | Cmp::TRUE_LOUD
            | Cmp::FALSE_LOUD
            | Cmp::ORD_LOUD
            | Cmp::UNORD_LOUD => Noise::Loud,
        }
    }

    /// Get the [`Order`] of this predicate.
    #[inline]
    #[must_use]
    pub const fn order(self) -> Order {
        match self {
            Cmp::EQ
            | Cmp::EQ_LOUD
            | Cmp::NEQ
            | Cmp::NEQ_LOUD
            | Cmp::GE
            | Cmp::GE_LOUD
            | Cmp::GT
            | Cmp::GT_LOUD
            | Cmp::LE
            | Cmp::LE_LOUD
            | Cmp::LT
            | Cmp::LT_LOUD
            | Cmp::FALSE
            | Cmp::FALSE_LOUD
            | Cmp::ORD
            | Cmp::ORD_LOUD => Order::Ordered,

            Cmp::EQ_UNORD
            | Cmp::EQ_UNORD_LOUD
            | Cmp::NEQ_UNORD
            | Cmp::NEQ_UNORD_LOUD
            | Cmp::NGE
            | Cmp::NGE_LOUD
            | Cmp::NGT
            | Cmp::NGT_LOUD
            | Cmp::NLE
            | Cmp::NLE_LOUD
            | Cmp::NLT
            | Cmp::NLT_LOUD
            | Cmp::TRUE
            | Cmp::TRUE_LOUD
            | Cmp::UNORD
            | Cmp::UNORD_LOUD => Order::Unorderd,
        }
    }

    /// Get this predicate with the inverse [`Noise`].
    #[inline]
    #[must_use]
    pub const fn inverse_noise(self) -> Cmp {
        match Cmp::from_i32((self as i32) ^ SIGNAL_BIT) {
            Some(cmp) => cmp,
            None => unreachable!(),
        }
    }

    /// Get this predicate with the inverse [`Order`].
    #[inline]
    #[must_use]
    pub const fn inverse_order(self) -> Cmp {
        match Cmp::from_i32((self as i32) ^ ORDER_BIT) {
            Some(cmp) => cmp,
            None => unreachable!(),
        }
    }

    /// Get the inverse of this predicate.
    #[inline]
    #[must_use]
    pub const fn inverse(self) -> Cmp {
        match Cmp::from_i32((self as i32) ^ RESULT_BIT) {
            Some(cmp) => cmp,
            None => unreachable!(),
        }
    }

    /// Get the inverse of this predicate while preserving its [`Order`].
    #[inline]
    #[must_use]
    pub const fn reverse(self) -> Cmp {
        self.inverse().inverse_order()
    }
}

impl Cmp {
    #[inline]
    #[must_use]
    pub const fn is_eq(self) -> bool {
        matches!(
            self,
            Cmp::EQ | Cmp::EQ_LOUD | Cmp::EQ_UNORD | Cmp::EQ_UNORD_LOUD
        )
    }

    #[inline]
    #[must_use]
    pub const fn is_neq(self) -> bool {
        matches!(
            self,
            Cmp::NEQ | Cmp::NEQ_LOUD | Cmp::NEQ_UNORD | Cmp::NEQ_UNORD_LOUD
        )
    }

    #[inline]
    #[must_use]
    pub const fn is_ge(self) -> bool {
        matches!(self, Cmp::GE | Cmp::GE_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_nge(self) -> bool {
        matches!(self, Cmp::NGE | Cmp::NGE_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_gt(self) -> bool {
        matches!(self, Cmp::GT | Self::GT_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_ngt(self) -> bool {
        matches!(self, Cmp::NGT | Self::NGT_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_le(self) -> bool {
        matches!(self, Cmp::LE | Cmp::LE_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_nle(self) -> bool {
        matches!(self, Cmp::NLE | Cmp::NLE_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_lt(self) -> bool {
        matches!(self, Cmp::LT | Cmp::LT_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_nlt(self) -> bool {
        matches!(self, Cmp::NLT | Cmp::NLT_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_false(self) -> bool {
        matches!(self, Cmp::FALSE | Cmp::FALSE_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_true(self) -> bool {
        matches!(self, Cmp::TRUE | Cmp::TRUE_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_ord(self) -> bool {
        matches!(self, Cmp::ORD | Cmp::ORD_LOUD)
    }

    #[inline]
    #[must_use]
    pub const fn is_unord(self) -> bool {
        matches!(self, Cmp::UNORD | Cmp::UNORD_LOUD)
    }
}

impl fmt::Display for Cmp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The [`Order`] of a predicate dictates whether it returns `false` or `true` upon
/// encountering a `NaN`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Order {
    /// The comparison results in `false` when comparing with a `NaN`.
    #[default]
    Ordered,
    /// The comparison results in `true` when comparing with a `NaN`.
    Unorderd,
}

impl Order {
    #[inline]
    #[must_use]
    pub const fn is_ordered(self) -> bool {
        matches!(self, Order::Ordered)
    }

    #[inline]
    #[must_use]
    pub const fn is_unordered(self) -> bool {
        matches!(self, Order::Unorderd)
    }
}

/// The [`Noise`] of a predicate dictates whether it is a ***signalling*** or ***non-signalling***
/// operation.
///
/// TODO: Finish describing what it means for something to signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Noise {
    /// The comparison is quiet.
    #[default]
    Quiet,
    /// The comparison is loud.
    Loud,
}

impl Noise {
    #[inline]
    #[must_use]
    pub const fn is_quiet(self) -> bool {
        matches!(self, Noise::Quiet)
    }

    #[inline]
    #[must_use]
    pub const fn is_loud(self) -> bool {
        matches!(self, Noise::Loud)
    }
}
