use crate::macros::vectors;

vectors! {
    pub struct u16x1 {
        arr: [u16; 1],
        bits: 16,
    }

    pub struct u16x2 {
        arr: [u16; 2],
        bits: 32,
        half: u16x1,
    }

    pub struct u16x4 {
        arr: [u16; 4],
        bits: 64,
        half: u16x2,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint16x4_t,
    }

    pub struct u16x8 {
        arr: [u16; 8],
        bits: 128,
        half: u16x4,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint16x8_t,

    }

    pub struct u16x16 {
        arr: [u16; 16],
        bits: 256,
        half: u16x8,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }

    pub struct u16x32 {
        arr: [u16; 32],
        bits: 512,
        half: u16x16,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512i,
    }

    pub struct u16x64 {
        arr: [u16; 64],
        bits: 1024,
        half: u16x32,
    }
}
