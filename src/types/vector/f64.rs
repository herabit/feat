use crate::macros::vectors;

vectors! {
    pub struct f64x1 {
        arr: [f64; 1],
        bits: 64,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::float64x1_t,
    }

    pub struct f64x2 {
        arr: [f64; 2],
        bits: 128,
        half: f64x1,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128d,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128d,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::float64x2_t,
    }

    pub struct f64x4 {
        arr: [f64; 4],
        bits: 256,
        half: f64x2,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256d,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256d,
    }

    pub struct f64x8 {
        arr: [f64; 8],
        bits: 512,
        half: f64x4,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512d,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512d,
    }

    pub struct f64x16 {
        arr: [f64; 16],
        bits: 1024,
        half: f64x8,
    }

    pub struct f64x32 {
        arr: [f64; 32],
        bits: 2048,
        half: f64x16,
    }

    pub struct f64x64 {
        arr: [f64; 64],
        bits: 4096,
        half: f64x32,
    }
}
