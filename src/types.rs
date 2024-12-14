mod u8;
pub use u8::*;

pub(crate) mod align {
    use core::fmt::{Debug, Formatter, Result};
    // #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    // #[repr(C, align(1))]

    macro_rules! align {
        ($($name:ident: $align:tt),* $(,)?) => {
            $(
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
                #[repr(C, align($align))]
                #[allow(unused)]
                pub(crate) struct $name<T: ?Sized>(pub T);

                impl<T: ?Sized + Debug> Debug for $name<T> {
                    fn fmt(&self, f: &mut Formatter) -> Result {
                        self.0.fmt(f)
                    }
                }
            )*
        };
    }

    align!(Align1: 1, Align2: 2, Align4: 4, Align8: 8, Align16: 16, Align32: 32, Align64: 64, Align128: 128, Align256: 256);
}
