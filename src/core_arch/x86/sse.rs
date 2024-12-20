#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Sse {
    __private: (),
}

impl Sse {
    #[inline(always)]
    #[must_use]
    pub const unsafe fn new_unchecked() -> Sse {
        Sse { __private: () }
    }
}
