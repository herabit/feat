use crate::macros::vectors;

vectors! {
    pub struct u32x1 {
        arr: [u32; 1],
        bits: 32,
    }

    pub struct u32x2 {
        arr: [u32; 2],
        bits: 64,
        half: u32x1,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint32x2_t,
    }

    pub struct u32x4 {
        arr: [u32; 4],
        bits: 128,
        half: u32x2,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint32x4_t,
    }

    pub struct u32x8 {
        arr: [u32; 8],
        bits: 256,
        half: u32x4,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }

    pub struct u32x16 {
        arr: [u32; 16],
        bits: 512,
        half: u32x8,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512i,
    }

    pub struct u32x32 {
        arr: [u32; 32],
        bits: 1024,
        half: u32x16,
    }

    pub struct u32x64 {
        arr: [u32; 64],
        bits: 2048,
        half: u32x32,
    }
}
