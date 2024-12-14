use crate::macros::*;

arch_import! {
    #[cfg(target_arch = "x86_64")]
    use x86_64::{ __m128i, __m256i, __m512i  };

    #[cfg(target_arch = "x86")]
    use x86::{ __m128i, __m256i, __m512i };

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    use aarch64::{ uint8x8_t, uint8x16_t };
}

#[derive(Clone, Copy)]
#[repr(transparent)]
#[doc = vector_docs!([u8; 1]: 8)]
pub struct u8x1 {
    pub(crate) scalar: u8,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
#[doc = vector_docs!([u8; 2]: 16)]
pub struct u8x2 {
    // pub(crate) halves: [u8x1; 2],
    pub(crate) bits: u16,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
#[doc = vector_docs!([u8; 4]: 32)]
pub struct u8x4 {
    // pub(crate) halves: [u8x2; 2],
    pub(crate) bits: u32,
}

select! {
    cfg(target_arch = "aarch64") |
    cfg(target_arch = "arm64ec") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 8]: 64)]
        pub struct u8x8 {
            pub(crate) neon: uint8x8_t,
        }
    },
    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 8]: 64)]
        pub struct u8x8 {
            // pub(crate) halves: [u8x4; 2],
            pub(crate) bits: u64,
        }
    }
}

select! {
    cfg(target_arch = "x86_64") |
    cfg(target_arch = "x86") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 16]: 128)]
        pub struct u8x16 {
            pub(crate) sse: __m128i,
        }
    },

    cfg(target_arch = "aarch64") |
    cfg(target_arch = "arm64ec") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 16]: 128)]
        pub struct u8x16 {
            pub(crate) neon: uint8x16_t,
        }
    },

    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 16]: 128)]
        pub struct u8x16 {
            pub(crate) bits: u128,
            // pub(crate) halves: [u8x8; 2],
        }
    },
}

select! {
    cfg(target_arch = "x86_64") |
    cfg(target_arch = "x86") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 32]: 256)]
        pub struct u8x32 {
            pub(crate) avx: __m256i,
        }
    }

    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 32]: 256)]
        pub struct u8x32 {
            pub(crate) halves: [u8x16; 2],
        }
    }
}

select! {
    cfg(target_arch = "x86_64") |
    cfg(target_arch = "x86") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 64]: 512)]
        pub struct u8x64 {
            pub(crate) avx: __m512i,
        }
    }

    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        #[doc = vector_docs!([u8; 64]: 512)]
        pub struct u8x64 {
            pub(crate) halves: [u8x32; 2],
        }
    }
}

vector_base!(u8x1: [u8; 1]);
vector_base!(u8x2/u8x1: [u8; 2]);
vector_base!(u8x4/u8x2: [u8; 4]);
vector_base!(u8x8/u8x4: [u8; 8]);
vector_base!(u8x16/u8x8: [u8; 16]);
vector_base!(u8x32/u8x16: [u8; 32]);
vector_base!(u8x64/u8x32: [u8; 64]);
