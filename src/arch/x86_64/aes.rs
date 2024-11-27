use super::intrin::{
    _mm_aesdec_si128, _mm_aesdeclast_si128, _mm_aesenc_si128, _mm_aesenclast_si128,
    _mm_aesimc_si128, _mm_aeskeygenassist_si128,
};
use super::Aes;
use crate::ty::u8x16;
use bytemuck::must_cast;

impl Aes {
    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_aesdec_si128")]
    pub fn decrypt_128(self, a: u8x16, round_key: u8x16) -> u8x16 {
        // SAFETY: We know that AES is enabled.
        must_cast(unsafe { _mm_aesdec_si128(must_cast(a), must_cast(round_key)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_aesdeclast_si128")]
    pub fn decrypt_last_128(self, a: u8x16, round_key: u8x16) -> u8x16 {
        // SAFETY: We know that AES is enabled.
        must_cast(unsafe { _mm_aesdeclast_si128(must_cast(a), must_cast(round_key)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_aesenc_si128")]
    pub fn encrypt_128(self, a: u8x16, round_key: u8x16) -> u8x16 {
        // SAFETY: We know that AES is enabled.
        must_cast(unsafe { _mm_aesenc_si128(must_cast(a), must_cast(round_key)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_aesenclast_si128")]
    pub fn encrypt_last_128(self, a: u8x16, round_key: u8x16) -> u8x16 {
        // SAFETY: We know that AES is enabled.
        must_cast(unsafe { _mm_aesenclast_si128(must_cast(a), must_cast(round_key)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_aesimc_si128")]
    pub fn inv_mix_column_128(self, a: u8x16) -> u8x16 {
        // SAFETY: We know that AES is enabled.
        must_cast(unsafe { _mm_aesimc_si128(must_cast(a)) })
    }

    #[inline(always)]
    #[must_use]
    #[doc(alias = "_mm_aeskeygenassist_si128")]
    pub fn keygen_assist_128<const IMM8: i32>(self, a: u8x16) -> u8x16 {
        // SAFETY: We know that AES is enabled.
        must_cast(unsafe { _mm_aeskeygenassist_si128::<IMM8>(must_cast(a)) })
    }
}
