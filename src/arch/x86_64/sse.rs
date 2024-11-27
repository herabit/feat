#![allow(unused_imports)]

use super::intrin::*;
use super::Sse;
use crate::macros::delegate;
use crate::muck::{cast, cast_mut, cast_ref};
use crate::ty::f32x4;

impl Sse {
    delegate! {
        fn undefined_f32x4() -> { f32x4 } = _mm_undefined_ps;

        fn add_f32x4(a: { f32x4 }, b: { f32x4 }) -> { f32x4 } = _mm_add_ps;
        fn add_lo_f32x4(a: { f32x4 }, b: { f32x4 }) -> { f32x4 } = _mm_add_ss;

        fn and_f32x4(a: { f32x4 }, b: { f32x4 }) -> { f32x4 } = _mm_and_ps;
        fn and_not_f32x4(a: { f32x4 }, b: { f32x4 }) -> { f32x4 } = _mm_andnot_ps;
    }
}
