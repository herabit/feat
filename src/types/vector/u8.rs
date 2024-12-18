use crate::macros::vectors;

vectors! {
    pub struct u8x1 {
        arr: [u8; 1],
        bits: 8,
    }

    pub struct u8x2 {
        arr: [u8; 2],
        bits: 16,
        half: u8x1,
    }

    pub struct u8x4 {
        arr: [u8; 4],
        bits: 32,
        half: u8x2,
    }

    pub struct u8x8 {
        arr: [u8; 8],
        bits: 64,
        half: u8x4,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint8x8_t,
    }

    pub struct u8x16 {
        arr: [u8; 16],
        bits: 128,
        half: u8x8,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint8x16_t,
    }

    pub struct u8x32 {
        arr: [u8; 32],
        bits: 256,
        half: u8x16,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }


    pub struct u8x64 {
        arr: [u8; 64],
        bits: 512,
        half: u8x32,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m512i,
    }
}
