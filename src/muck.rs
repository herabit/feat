//! Rexports various things from [`bytemuck`], as well as builds some functionality
//! not yet available within it for use in this crate.

use core::slice::from_ref;

/// Bytemuck crate, which is used extensively in this crate.
pub use bytemuck;

#[doc(inline)]
pub use bytemuck::{AnyBitPattern, CheckedBitPattern, NoUninit, Pod, TransparentWrapper, Zeroable};

/// Cast `A` into `B`, or fail to compile.
///
/// See [`bytemuck::must_cast`].
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn cast<A: NoUninit, B: AnyBitPattern>(a: A) -> B {
    bytemuck::must_cast(a)
}

/// Cast `&A` to `&B`, or fail to compile.
///
/// See [`bytemuck::must_cast_ref`].
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn cast_ref<A: NoUninit, B: AnyBitPattern>(a: &A) -> &B {
    bytemuck::must_cast_ref(a)
}

/// Cast `&mut A` to `&mut B`, or fail to compile.
///
/// See [`bytemuck::must_cast_mut`].
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn cast_mut<A: NoUninit + AnyBitPattern, B: NoUninit + AnyBitPattern>(
    a: &mut A,
) -> &mut B {
    const {
        // ASSERT_SIZE_EQUAL
        assert!(size_of::<A>() == size_of::<B>());
        // ASSERT_ALIGN_GREATER_THAN_EQUAL
        assert!(align_of::<A>() >= align_of::<B>());
    };
    // SAFETY: We did the checks that `must_cast_mut` does,
    //         and none failed, so this must be safe.
    unsafe { &mut *(a as *mut A as *mut B) }
}

/// Cast `&[A]` to `&[B]`, or fail to compile.
///
/// See [`bytemuck::must_cast_slice`].
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn cast_slice<A: NoUninit, B: AnyBitPattern>(a: &[A]) -> &[B] {
    bytemuck::must_cast_slice(a)
}

/// Cast `&mut [A]` to `&mut [B]`, or fail to compile.
///
/// See [`bytemuck::must_cast_slice_mut`].
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn cast_slice_mut<A: NoUninit + AnyBitPattern, B: NoUninit + AnyBitPattern>(
    a: &mut [A],
) -> &mut [B] {
    const {
        // ASSERT_SIZE_MULTIPLE_OF_OR_INPUT_ZST
        assert!(
            (size_of::<A>() == 0) || (size_of::<B>() != 0 && size_of::<A>() % size_of::<B>() == 0)
        );
        // ASSERT_ALIGN_GREATER_THAN_EQUAL
        assert!(align_of::<A>() >= align_of::<B>());
    };

    let new_len = if size_of::<A>() == size_of::<B>() {
        a.len()
    } else {
        a.len() * (size_of::<A>() / size_of::<B>())
    };

    // SAFETY: We did all the checks that `must_cast_slice_mut` does,
    //         and none failed, so the following must be safe.
    unsafe { core::slice::from_raw_parts_mut(a.as_mut_ptr() as *mut B, new_len) }
}

#[inline(always)]
#[must_use]
const fn _cast<A: NoUninit, B: AnyBitPattern>(a: A) -> B {
    assert!(size_of::<A>() == size_of::<B>());

    #[repr(C)]
    union Transmute<A: Copy, B: Copy> {
        a: A,
        b: B,
    }

    unsafe { Transmute::<A, B> { a }.b }
}

/// Determine whether a given value is all zeros.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn all_zeros<A: NoUninit>(a: A) -> bool {
    match size_of::<A>() {
        0 => false,
        1 => _cast::<_, u8>(a) == 0,
        2 => _cast::<_, u16>(a) == 0,
        4 => _cast::<_, u32>(a) == 0,
        8 => _cast::<_, u64>(a) == 0,
        16 => _cast::<_, u128>(a) == 0,
        _ => {
            let mut bytes: &[u8] = cast_slice(from_ref(&a));
            let mut all_zeros = true;

            while let [b, rest @ ..] = bytes {
                all_zeros &= *b == 0x00;
                bytes = rest;
            }

            all_zeros
        }
    }
}

/// Determine whether a given value is all ones.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn all_ones<A: NoUninit>(a: A) -> bool {
    match size_of::<A>() {
        0 => false,
        1 => _cast::<_, u8>(a) == !0,
        2 => _cast::<_, u16>(a) == !0,
        4 => _cast::<_, u32>(a) == !0,
        8 => _cast::<_, u64>(a) == !0,
        16 => _cast::<_, u128>(a) == !0,
        _ => {
            let mut bytes: &[u8] = cast_slice(from_ref(&a));
            let mut all_ones = true;

            while let [b, rest @ ..] = bytes {
                all_ones &= *b == 0xFF;
                bytes = rest;
            }

            all_ones
        }
    }
}

/// Identify what kind of mask this is.
///
/// Returns `Some(true)` for all ones,
/// `Some(false)` for all zeros,
/// and `None` if it is not a valid mask.
#[inline(always)]
#[must_use]
#[track_caller]
pub const fn identify_mask<A: NoUninit>(a: A) -> Option<bool> {
    if all_ones(a) {
        Some(true)
    } else if all_zeros(a) {
        Some(false)
    } else {
        None
    }
}
