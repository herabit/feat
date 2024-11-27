use super::intrin::*;
use super::Aes;
use crate::macros::delegate;
use crate::muck::cast;
use crate::ty::u8x16;

impl Aes {
    delegate! {
        fn decrypt_128(a: { u8x16 }, round_key: { u8x16 }) -> { u8x16 } = _mm_aesdec_si128;
        fn decrypt_last_128(a: { u8x16 }, round_key: { u8x16 }) -> { u8x16 } = _mm_aesdeclast_si128;

        fn encrypt_128(a: { u8x16 }, round_key: { u8x16 }) -> { u8x16 } = _mm_aesenc_si128;
        fn encrypt_last_128(a: { u8x16 }, round_key: { u8x16 }) -> { u8x16 } = _mm_aesenclast_si128;

        fn inv_mix_column_128(a: { u8x16 }) -> { u8x16 } = _mm_aesimc_si128;
        fn keygen_assist_128<const IMM8: i32>(a: { u8x16 }) -> { u8x16 } = _mm_aeskeygenassist_si128;
    }
}
