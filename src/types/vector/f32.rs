use crate::macros::vectors;

vectors! {
    pub struct f32x1 {
        arr: [f32; 1],
        bits: 32,
    }

    pub struct f32x2 {
        arr: [f32; 2],
        bits: 64,
        half: f32x1,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::float32x2_t,
    }

    pub struct f32x4 {
        arr: [f32; 4],
        bits: 128,
        half: f32x2,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::float32x4_t,
    }

    pub struct f32x8 {
        arr: [f32; 8],
        bits: 256,
        half: f32x4,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256,
    }

    pub struct f32x16 {
        arr: [f32; 16],
        bits: 512,
        half: f32x8,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512,
    }

    pub struct f32x32 {
        arr: [f32; 32],
        bits: 1024,
        half: f32x16,
    }

    pub struct f32x64 {
        arr: [f32; 64],
        bits: 2048,
        half: f32x32,
    }
}
