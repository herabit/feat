use crate::sealed;

use super::Mask;

/// Marker trait for types that are lanes.
///
/// # Safety
///
/// A lane type must meet the following invariants:
///
/// - `Lane::ZERO`'s bit pattern is all zeros in memory.
///
/// - A lane must be able to be set to all zeros in memory without
///   causing *undefined behavior*.
///
///   In other words, any given `Lane` must implement the `Zeroable`
///   trait from bytemuck.
///
/// - `Lane::Mask` must have the same memory layout as this lane:
///
///    - Same alignment.
///    - Same size.
///
/// - A lane must have no uninitialized bytes.
///
/// - A lane must have no interior mutability.
pub unsafe trait Lane: sealed::Sealed + Send + Sync + Copy + Default {
    /// The mask type for this lane.
    type Mask: Mask;

    /// A zeroed out lane.
    //
    // SAFETY: See the invariants of [`Lane`].
    const ZERO: Self = unsafe { core::mem::zeroed() };

    #[doc(hidden)]
    const _SANITY_CHECK: () = {
        assert!(
            size_of::<Self::Mask>() == size_of::<Self>(),
            "mask size doesn't match lane size"
        );
        assert!(
            align_of::<Self::Mask>() == align_of::<Self>(),
            "mask alignment doesn't match lane alignment"
        );
    };
}

macro_rules! impl_lane {
    ($($lane:ident: $mask:ident),* $(,)?) => {
        $(
            unsafe impl Lane for $lane {
                type Mask = super::$mask;
            }

            const _: () = <$lane as Lane>::_SANITY_CHECK;
        )*
    };
}

impl_lane!(u8: m8, u16: m16, u32: m32, u64: m64, usize: msize);
impl_lane!(i8: m8, i16: m16, i32: m32, i64: m64, isize: msize);
impl_lane!(f32: m32, f64: m64);
// impl_lane!(bool: m8);
