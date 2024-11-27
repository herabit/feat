use crate::macros::tokens;

tokens! {
    Aes = "aes";
    Sha = "sha";
}

pub use core::arch::x86_64 as intrin;

mod aes;
mod sha;
