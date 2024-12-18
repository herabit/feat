use crate::macros::vectors;

vectors! {
    pub struct i32x1 {
        arr: [i32; 1],
        bits: 32,
    }

    pub struct i32x2 {
        arr: [i32; 2],
        bits: 64,
        half: i32x1,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int32x2_t,
    }

    pub struct i32x4 {
        arr: [i32; 4],
        bits: 128,
        half: i32x2,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int32x4_t,
    }

    pub struct i32x8 {
        arr: [i32; 8],
        bits: 256,
        half: i32x4,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }

    pub struct i32x16 {
        arr: [i32; 16],
        bits: 512,
        half: i32x8,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512i,
    }

    pub struct i32x32 {
        arr: [i32; 32],
        bits: 1024,
        half: i32x16,
    }

    pub struct i32x64 {
        arr: [i32; 64],
        bits: 2048,
        half: i32x32,
    }
}
