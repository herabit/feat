use crate::macros::vectors;

vectors! {
    pub struct i16x1 {
        arr: [i16; 1],
        bits: 16,
    }

    pub struct i16x2 {
        arr: [i16; 2],
        bits: 32,
        half: i16x1,
    }

    pub struct i16x4 {
        arr: [i16; 4],
        bits: 64,
        half: i16x2,

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int16x4_t,
    }

    pub struct i16x8 {
        arr: [i16; 8],
        bits: 128,
        half: i16x4,

        #[cfg(target_arch = "x86_64")]
        sse: x86_64::__m128i,
        #[cfg(target_arch = "x86")]
        sse: x86::__m128i,
        #[cfg(target_arch = "wasm32")]
        simd128: wasm32::v128,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        neon: aarch64::int16x8_t,

    }

    pub struct i16x16 {
        arr: [i16; 16],
        bits: 256,
        half: i16x8,

        #[cfg(target_arch = "x86_64")]
        avx: x86_64::__m256i,
        #[cfg(target_arch = "x86")]
        avx: x86::__m256i,
    }

    pub struct i16x32 {
        arr: [i16; 32],
        bits: 512,
        half: i16x16,

        #[cfg(target_arch = "x86_64")]
        avx512: x86_64::__m512i,
        #[cfg(target_arch = "x86")]
        avx512: x86::__m512i,
    }

    pub struct i16x64 {
        arr: [i16; 64],
        bits: 1024,
        half: i16x32,
    }
}
