use crate::{types::vector::f32x4, Func};

use super::raw::{
    _mm_add_ps, _mm_add_ss, _mm_div_ps, _mm_div_ss, _mm_mul_ps, _mm_mul_ss, _mm_sub_ps, _mm_sub_ss,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Sse {
    __private: (),
}

impl Sse {
    #[inline(always)]
    #[must_use]
    pub const unsafe fn new_unchecked() -> Sse {
        Sse { __private: () }
    }

    /// Call a given closure with SSE enabled.
    ///
    /// Despite being marked as unsafe, this is actually safe as
    /// [`Sse`] proves that sse is available.
    ///
    /// This however has to be marked as one cannot use the `target_feature`
    /// attribute without it being an unsafe method.
    #[inline]
    #[target_feature(enable = "sse")]
    pub unsafe fn execute<F: Func>(self, f: F) -> F::Output {
        f.call()
    }

    /// Call a given closure with SSE enabled.
    #[inline(always)]
    pub fn run<F: Func>(self, f: F) -> F::Output {
        // SAFETY: We know that SSE is enabled.
        unsafe { self.execute(f) }
    }
}

impl Sse {
    #[doc(alias = "_mm_add_ps")]
    #[inline(always)]
    #[must_use]
    pub fn add_f32x4(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_add_ps(a.sse, b.sse) }.into()
    }

    #[doc(alias = "_mm_sub_ps")]
    #[inline(always)]
    #[must_use]
    pub fn sub_f32x4(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_sub_ps(a.sse, b.sse) }.into()
    }

    #[doc(alias = "_mm_mul_ps")]
    #[inline(always)]
    #[must_use]
    pub fn mul_f32x4(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_mul_ps(a.sse, b.sse) }.into()
    }

    #[doc(alias = "_mm_div_ps")]
    #[inline(always)]
    #[must_use]
    pub fn div_f32x4(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_div_ps(a.sse, b.sse) }.into()
    }
}

impl Sse {
    #[doc(alias = "_mm_add_ss")]
    #[inline(always)]
    #[must_use]
    pub fn add_f32x4_s(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_add_ss(a.sse, b.sse) }.into()
    }

    #[doc(alias = "_mm_sub_ss")]
    #[inline(always)]
    #[must_use]
    pub fn sub_f32x4_s(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_sub_ss(a.sse, b.sse) }.into()
    }

    #[doc(alias = "_mm_mul_ss")]
    #[inline(always)]
    #[must_use]
    pub fn mul_f32x4_s(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_mul_ss(a.sse, b.sse) }.into()
    }

    #[doc(alias = "_mm_div_ss")]
    #[inline(always)]
    #[must_use]
    pub fn div_f32x4_s(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { _mm_div_ss(a.sse, b.sse) }.into()
    }
}
