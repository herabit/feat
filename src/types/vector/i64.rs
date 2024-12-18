use crate::macros::vectors;

vectors! {
    pub struct i64x1 {
        arr: [i64; 1],
        bits: 64,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int64x1_t,
    }

    pub struct i64x2 {
        arr: [i64; 2],
        bits: 128,
        half: i64x1,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int64x2_t,
    }

    pub struct i64x4 {
        arr: [i64; 4],
        bits: 256,
        half: i64x2,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }

    pub struct i64x8 {
        arr: [i64; 8],
        bits: 512,
        half: i64x4,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512i,
    }

    pub struct i64x16 {
        arr: [i64; 16],
        bits: 1024,
        half: i64x8,
    }


    pub struct i64x32 {
        arr: [i64; 32],
        bits: 2048,
        half: i64x16,
    }

    pub struct i64x64 {
        arr: [i64; 64],
        bits: 4096,
        half: i64x32,
    }
}
