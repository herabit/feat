use crate::macros::vectors;

vectors! {
    pub struct i8x1 {
        arr: [i8; 1],
        bits: 8,
    }

    pub struct i8x2 {
        arr: [i8; 2],
        bits: 16,
        half: i8x1,
    }

    pub struct i8x4 {
        arr: [i8; 4],
        bits: 32,
        half: i8x2,
    }

    pub struct i8x8 {
        arr: [i8; 8],
        bits: 64,
        half: i8x4,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int8x8_t,
    }

    pub struct i8x16 {
        arr: [i8; 16],
        bits: 128,
        half: i8x8,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int8x16_t,
    }

    pub struct i8x32 {
        arr: [i8; 32],
        bits: 256,
        half: i8x16,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }


    pub struct i8x64 {
        arr: [i8; 64],
        bits: 512,
        half: i8x32,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m512i,
    }
}
