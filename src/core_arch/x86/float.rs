use super::raw::*;

/// A more Rusty way of representing AVX floating point comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatCmp {
    /// Equals (`a == b`)
    Eq(Order, Noise),
    /// Not-equals (`a != b`)
    Neq(Order, Noise),

    /// Greater-than-or-equal (`a >= b`)
    Ge(Noise),
    /// Not-greater-than-or-equal (`!(a >= b)`)
    Nge(Noise),

    /// Greater-than (`a > b`)
    Gt(Noise),
    /// Not-greater-than (`!(a > b)`)
    Ngt(Noise),

    /// Less-than-or-equal (`a <= b`)
    Le(Noise),
    /// Not-less-than-or-equal (`!(a <= b)`)
    Nle(Noise),

    /// Less-than (`a < b`)
    Lt(Noise),
    /// Not-less-than (`!(a < b)`)
    Nlt(Noise),

    /// False (`false`)
    False(Noise),
    /// True (`true`)
    True(Noise),

    /// Ordered (`!a.is_nan() && !b.is_nan()`)
    Ord(Noise),
    /// Unordered (`a.is_nan() || b.is_nan()`)
    Unord(Noise),
}

macro_rules! constructors {
    ($(
        $(#[$meta:meta])*
        $vis:vis const fn $name:ident = $variant:ident;
    )*) => {
        $(
            $(#[$meta])*
            #[allow(unused)]
            $vis use self::FloatCmp::$variant as $name;
        )*
    };
}

constructors! {
    const fn eq = Eq;
    const fn not_eq = Neq;

    const fn ge = Ge;
    const fn not_ge = Nge;

    const fn gt = Gt;
    const fn not_gt = Ngt;

    const fn le = Le;
    const fn not_le = Nle;

    const fn lt = Lt;
    const fn not_lt = Nlt;

    const fn tru = True;
    const fn fals = False;

    const fn ord = Ord;
    const fn unord = Unord;
}

impl FloatCmp {
    /// Create a [`FloatCmp`] from an AVX immediate.
    #[inline]
    #[must_use]
    pub const fn from_imm(imm: i32) -> Option<FloatCmp> {
        match imm {
            _CMP_EQ_OQ => Some(eq(ORD, QUIET)),
            _CMP_EQ_OS => Some(eq(ORD, LOUD)),

            _CMP_EQ_UQ => Some(eq(UNORD, QUIET)),
            _CMP_EQ_US => Some(eq(UNORD, LOUD)),

            _CMP_NEQ_OQ => Some(not_eq(ORD, QUIET)),
            _CMP_NEQ_OS => Some(not_eq(ORD, LOUD)),

            _CMP_NEQ_UQ => Some(not_eq(UNORD, QUIET)),
            _CMP_NEQ_US => Some(not_eq(UNORD, LOUD)),

            _CMP_GE_OQ => Some(ge(QUIET)),
            _CMP_GE_OS => Some(ge(LOUD)),

            _CMP_NGE_UQ => Some(not_ge(QUIET)),
            _CMP_NGE_US => Some(not_ge(LOUD)),

            _CMP_GT_OQ => Some(gt(QUIET)),
            _CMP_GT_OS => Some(gt(LOUD)),

            _CMP_NGT_UQ => Some(not_gt(QUIET)),
            _CMP_NGT_US => Some(not_gt(LOUD)),

            _CMP_LE_OQ => Some(le(QUIET)),
            _CMP_LE_OS => Some(le(LOUD)),

            _CMP_NLE_UQ => Some(not_le(QUIET)),
            _CMP_NLE_US => Some(not_le(LOUD)),

            _CMP_LT_OQ => Some(lt(QUIET)),
            _CMP_LT_OS => Some(lt(LOUD)),

            _CMP_NLT_UQ => Some(not_lt(QUIET)),
            _CMP_NLT_US => Some(not_lt(LOUD)),

            _CMP_FALSE_OQ => Some(fals(QUIET)),
            _CMP_FALSE_OS => Some(fals(LOUD)),

            _CMP_TRUE_UQ => Some(tru(QUIET)),
            _CMP_TRUE_US => Some(tru(LOUD)),

            _CMP_ORD_Q => Some(ord(QUIET)),
            _CMP_ORD_S => Some(ord(LOUD)),

            _CMP_UNORD_Q => Some(unord(QUIET)),
            _CMP_UNORD_S => Some(unord(LOUD)),

            // We do this just to ensure we actually cover all possibilities.
            //
            // all predicates lie within `0..32`.
            ..=0 | 32.. => None,
        }
    }

    /// Get an AVX immediate value.
    #[inline]
    #[must_use]
    pub const fn to_imm(self) -> i32 {
        match self {
            eq(ORD, QUIET) => _CMP_EQ_OQ,
            eq(ORD, LOUD) => _CMP_EQ_OS,

            eq(UNORD, QUIET) => _CMP_EQ_UQ,
            eq(UNORD, LOUD) => _CMP_EQ_US,

            not_eq(ORD, QUIET) => _CMP_NEQ_OQ,
            not_eq(ORD, LOUD) => _CMP_NEQ_OS,

            not_eq(UNORD, QUIET) => _CMP_NEQ_UQ,
            not_eq(UNORD, LOUD) => _CMP_NEQ_US,

            ge(QUIET) => _CMP_GE_OQ,
            ge(LOUD) => _CMP_GE_OS,

            not_ge(QUIET) => _CMP_NGE_UQ,
            not_ge(LOUD) => _CMP_NGE_US,

            gt(QUIET) => _CMP_GT_OQ,
            gt(LOUD) => _CMP_GT_OS,

            not_gt(QUIET) => _CMP_NGT_UQ,
            not_gt(LOUD) => _CMP_NGT_US,

            le(QUIET) => _CMP_LE_OQ,
            le(LOUD) => _CMP_LE_OS,

            not_le(QUIET) => _CMP_NLE_UQ,
            not_le(LOUD) => _CMP_NLE_US,

            lt(QUIET) => _CMP_LT_OQ,
            lt(LOUD) => _CMP_LT_OS,

            not_lt(QUIET) => _CMP_NLT_UQ,
            not_lt(LOUD) => _CMP_NLT_US,

            tru(QUIET) => _CMP_TRUE_UQ,
            tru(LOUD) => _CMP_TRUE_US,

            fals(QUIET) => _CMP_FALSE_OQ,
            fals(LOUD) => _CMP_FALSE_OS,

            ord(QUIET) => _CMP_ORD_Q,
            ord(LOUD) => _CMP_ORD_S,

            unord(QUIET) => _CMP_UNORD_Q,
            unord(LOUD) => _CMP_UNORD_S,
        }
    }

    /// Get the [`Noise`] of this operator.
    #[inline]
    #[must_use]
    pub const fn noise(self) -> Noise {
        match self {
            eq(_, noise)
            | not_eq(_, noise)
            | ge(noise)
            | not_ge(noise)
            | gt(noise)
            | not_gt(noise)
            | le(noise)
            | not_le(noise)
            | lt(noise)
            | not_lt(noise)
            | fals(noise)
            | tru(noise)
            | ord(noise)
            | unord(noise) => noise,
        }
    }

    /// Get a reference to the [`Noise`] of this operator.
    #[inline]
    #[must_use]
    pub const fn noise_ref(&self) -> &Noise {
        match self {
            eq(_, noise)
            | not_eq(_, noise)
            | ge(noise)
            | not_ge(noise)
            | gt(noise)
            | not_gt(noise)
            | le(noise)
            | not_le(noise)
            | lt(noise)
            | not_lt(noise)
            | fals(noise)
            | tru(noise)
            | ord(noise)
            | unord(noise) => noise,
        }
    }

    /// Get a mutable reference to the [`Noise`] of this operator.
    #[inline]
    #[must_use]
    pub const fn noise_mut(&mut self) -> &mut Noise {
        match self {
            eq(_, noise)
            | not_eq(_, noise)
            | ge(noise)
            | not_ge(noise)
            | gt(noise)
            | not_gt(noise)
            | le(noise)
            | not_le(noise)
            | lt(noise)
            | not_lt(noise)
            | fals(noise)
            | tru(noise)
            | ord(noise)
            | unord(noise) => noise,
        }
    }

    /// Create a new operator the same as this one but with the inverse [`Noise`].
    #[inline]
    #[must_use]
    #[no_mangle]
    pub const fn inverted_noise(mut self) -> FloatCmp {
        let noise = self.noise_mut();
        *noise = noise.invert();

        self
    }

    /// Get the [`Order`] of this operator.
    #[inline(always)]
    #[must_use]
    pub const fn order(self) -> Order {
        match self {
            eq(order, _) | not_eq(order, _) => order,
            ge(_) | gt(_) | le(_) | lt(_) | fals(_) | ord(_) => ORD,
            not_ge(_) | not_gt(_) | not_le(_) | not_lt(_) | tru(_) | unord(_) => UNORD,
        }
    }

    /// Create a new operator with the reverse [`Order`].
    #[inline]
    #[must_use]
    pub const fn reversed_order(self) -> FloatCmp {
        match self {
            eq(order, noise) => eq(order.reverse(), noise),
            not_eq(order, noise) => not_eq(order.reverse(), noise),
            ge(noise) => not_lt(noise),
            not_ge(noise) => todo!(),
            gt(noise) => todo!(),
            not_gt(noise) => todo!(),
            le(noise) => todo!(),
            not_le(noise) => todo!(),
            lt(noise) => todo!(),
            not_lt(noise) => todo!(),
            fals(noise) => todo!(),
            tru(noise) => todo!(),
            ord(noise) => unord(noise),
            unord(noise) => ord(noise),
        }
    }
}

/// Specifies whether comparisons to `NaN`s results in a `true` or a `false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Order {
    /// Comparison with a `NaN` results in `false`.
    #[default]
    Ordered,
    /// Comparison with a `NaN` results in `true`.
    Unordered,
}

impl Order {
    #[inline(always)]
    #[must_use]
    pub const fn reverse(self) -> Order {
        match self {
            ORD => UNORD,
            UNORD => ORD,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_ordered(self) -> bool {
        matches!(self, ORD)
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_unordered(self) -> bool {
        matches!(self, UNORD)
    }
}

/// Specifies whether a floating point operation signals or not.
///
/// # What is Signalling
///
/// Signalling means that the floating point "invalid" will be set
/// when comparing quiet `NaN`s.
///
/// See [here](https://stackoverflow.com/questions/16988199/how-to-choose-avx-compare-predicate-variants/64191351#comment113511368_17665386)
/// for more info.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Noise {
    /// The operation is non-signalling.
    #[default]
    Quiet,
    /// The operation is signalling.
    Loud,
}

impl Noise {
    #[inline(always)]
    #[must_use]
    pub const fn invert(self) -> Noise {
        match self {
            QUIET => LOUD,
            LOUD => QUIET,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_quiet(self) -> bool {
        matches!(self, QUIET)
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_loud(self) -> bool {
        matches!(self, LOUD)
    }
}

pub const QUIET: Noise = Noise::Quiet;
pub const LOUD: Noise = Noise::Loud;

pub const ORD: Order = Order::Ordered;
pub const UNORD: Order = Order::Unordered;
