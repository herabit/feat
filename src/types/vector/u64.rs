use crate::macros::vectors;

vectors! {
    pub struct u64x1 {
        arr: [u64; 1],
        bits: 64,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint64x1_t,
    }

    pub struct u64x2 {
        arr: [u64; 2],
        bits: 128,
        half: u64x1,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::uint64x2_t,
    }

    pub struct u64x4 {
        arr: [u64; 4],
        bits: 256,
        half: u64x2,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }

    pub struct u64x8 {
        arr: [u64; 8],
        bits: 512,
        half: u64x4,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512i,
    }

    pub struct u64x16 {
        arr: [u64; 16],
        bits: 1024,
        half: u64x8,
    }


    pub struct u64x32 {
        arr: [u64; 32],
        bits: 2048,
        half: u64x16,
    }

    pub struct u64x64 {
        arr: [u64; 64],
        bits: 4096,
        half: u64x32,
    }
}
