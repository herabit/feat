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
pub struct u8x1 {
    pub(crate) scalar: u8,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct u8x2 {
    pub(crate) halves: [u8x1; 2],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct u8x4 {
    pub(crate) halves: [u8x2; 2],
}

select! {
    cfg(target_arch = "aarch64") |
    cfg(target_arch = "arm64ec") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x8 {
            pub(crate) neon: uint8x8_t,
        }
    },
    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x8 {
            pub(crate) halves: [u8x4; 2],
        }
    }
}

select! {
    cfg(target_arch = "x86_64") |
    cfg(target_arch = "x86") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x16 {
            pub(crate) sse: __m128i,
        }
    },

    cfg(target_arch = "aarch64") |
    cfg(target_arch = "arm64ec") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x16 {
            pub(crate) neon: uint8x16_t,
        }
    },

    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x16 {
            pub(crate) halves: [u8x8; 2],
        }
    },
}

select! {
    cfg(target_arch = "x86_64") |
    cfg(target_arch = "x86") => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x32 {
            pub(crate) avx: __m256i,
        }
    }

    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
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
        pub struct u8x64 {
            pub(crate) avx: __m512i,
        }
    }

    _ => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct u8x64 {
            pub(crate) halves: [u8x32; 2],
        }
    }
}
