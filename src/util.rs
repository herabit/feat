#![allow(dead_code)]

#[inline(always)]
#[track_caller]
pub const fn assert_same_layout<Src, Dst>() {
    const {
        assert!(size_of::<Src>() == size_of::<Dst>(), "size mismatch");
        assert!(align_of::<Src>() == align_of::<Dst>(), "alignment mismatch");
    }
}

#[inline(always)]
#[track_caller]
pub const unsafe fn transmute<Src, Dst>(src: Src) -> Dst {
    const { assert!(size_of::<Src>() == size_of::<Dst>()) };

    use core::mem::ManuallyDrop;

    #[repr(C)]
    union Transmute<A, B> {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>,
    }

    unsafe {
        ManuallyDrop::into_inner(
            Transmute::<Src, Dst> {
                a: ManuallyDrop::new(src),
            }
            .b,
        )
    }
}
