#![allow(dead_code)]

use core::mem::ManuallyDrop;

pub(crate) mod contiguous;
pub(crate) mod forget;
pub(crate) mod no_uninit;
pub(crate) mod sealed;
pub(crate) mod zeroable;

#[doc(inline)]
pub use contiguous::*;
#[doc(inline)]
pub use forget::*;
#[doc(inline)]
pub use no_uninit::*;
#[doc(inline)]
pub use zeroable::*;

#[inline(always)]
#[must_use]
#[track_caller]
pub(crate) const unsafe fn transmute<Src, Dst>(src: Src) -> Dst {
    // assert!(size_of::<Src>() == size_of::<Dst>());

    let is_approx_same = const {
        let same_size = size_of::<Src>() == size_of::<Dst>();
        let same_align = align_of::<Src>() == align_of::<Dst>();

        // let opt_same_size = size_of::<Option<Src>>() == size_of::<Option<Dst>>();
        // let opt_same_align = align_of::<Option<Src>>() == align_of::<Option<Dst>>();

        same_size && same_align
    };

    if !is_approx_same {
        unreachable!();
    }

    #[repr(C)]
    union Transmuter<Src, Dst> {
        src: ManuallyDrop<Src>,
        dst: ManuallyDrop<Dst>,
    }

    unsafe {
        ManuallyDrop::into_inner(
            Transmuter::<Src, Dst> {
                src: ManuallyDrop::new(src),
            }
            .dst,
        )
    }
}

/// Marker trait for types that can safely have an `all zeros` bit pattern.
pub unsafe trait AllZeros: Sized {}

#[inline(always)]
#[must_use]
#[track_caller]
pub(crate) const unsafe fn must_transmute<Src, Dst>(src: Src) -> Dst {
    const { assert!(size_of::<Src>() == size_of::<Dst>()) };

    unsafe { self::transmute(src) }
}
