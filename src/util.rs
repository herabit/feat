#![allow(dead_code)]

use core::{cmp::Ordering, marker::PhantomData};

/// Type level usize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Usize<const N: usize>;

/// Type level proof of type equality.
#[repr(transparent)]
pub struct TypeEq<L: ?Sized, R: ?Sized> {
    _left: PhantomData<fn() -> L>,
    _right: PhantomData<fn() -> R>,
}

impl<T: ?Sized> TypeEq<T, T> {
    #[inline(always)]
    #[must_use]
    pub const fn new() -> TypeEq<T, T> {
        // SAFETY: `T == T`.
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L: ?Sized, R: ?Sized> TypeEq<L, R> {
    /// The caller must ensure that `L` and `R` are equal.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn new_unchecked() -> TypeEq<L, R> {
        #![allow(unused_unsafe)]

        const {
            // if `L` and `R` are actually the same type,
            // while we cannot check that they have the same size and alignment
            // since both may be unsized, we can ensure that their pointers
            // have the same size and alignment.
            //
            // For example this will catch faulty type equalities of `*const T` and `*const [T]`.
            assert!(size_of::<*const L>() == size_of::<*const R>());
            assert!(align_of::<*const L>() == align_of::<*const R>());
        };

        unsafe {
            TypeEq {
                _left: PhantomData,
                _right: PhantomData,
            }
        }
    }

    /// Flip the types.
    #[inline(always)]
    #[must_use]
    pub const fn flip(self) -> TypeEq<R, L> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Join two [`TypeEq`]s.
    #[inline(always)]
    #[must_use]
    pub const fn join<U: ?Sized>(self, _other: TypeEq<R, U>) -> TypeEq<L, U> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap both types in a const pointer.
    #[inline(always)]
    #[must_use]
    pub const fn in_ptr(self) -> TypeEq<*const L, *const R> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap both types in a mut pointer.
    #[inline(always)]
    #[must_use]
    pub const fn in_ptr_mut(self) -> TypeEq<*mut L, *mut R> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap both types in a immutable reference.
    #[inline(always)]
    #[must_use]
    pub const fn in_ref<'a>(self) -> TypeEq<&'a L, &'a R> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap both types in a immutable reference.
    #[inline(always)]
    #[must_use]
    pub const fn in_mut<'a>(self) -> TypeEq<&'a mut L, &'a mut R> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L: ?Sized, R: ?Sized> TypeEq<*const L, *const R> {
    /// Get the pointee of the pointer types.
    #[inline(always)]
    #[must_use]
    pub const fn pointee(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L: ?Sized, R: ?Sized> TypeEq<*mut L, *mut R> {
    /// Get the pointee of the pointer types.
    #[inline(always)]
    #[must_use]
    pub const fn pointee(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<'a, L: ?Sized, R: ?Sized> TypeEq<&'a L, &'a R> {
    /// Get the pointee of the pointer types.
    #[inline(always)]
    #[must_use]
    pub const fn pointee(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<'a, L: ?Sized, R: ?Sized> TypeEq<&'a mut L, &'a mut R> {
    /// Get the pointee of the pointer types.
    #[inline(always)]
    #[must_use]
    pub const fn pointee(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L, R> TypeEq<L, R> {
    /// Coerce a `R` to a `L`.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn to_left(self, right: R) -> L {
        // We only check option stuff because `transmute` does other checks itself.
        let is_approx = const {
            size_of::<Option<L>>() == size_of::<Option<R>>()
                && align_of::<Option<L>>() == align_of::<Option<R>>()
        };

        if !is_approx {
            unreachable!()
        }
        unsafe { crate::muck::transmute(right) }
    }

    /// Coerce a `L` to a `R`.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn to_right(self, left: L) -> R {
        unsafe { crate::muck::transmute(left) }
    }

    /// Wrap the types in a slice.
    #[inline(always)]
    #[must_use]
    pub const fn in_slice(self) -> TypeEq<[L], [R]> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap the types in an array.
    #[inline(always)]
    #[must_use]
    pub const fn in_array<const N0: usize, const N1: usize>(
        self,
        _len: TypeEq<Usize<N0>, Usize<N1>>,
    ) -> TypeEq<[L; N0], [R; N1]> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap the types in an option.
    #[inline(always)]
    #[must_use]
    pub const fn in_option(self) -> TypeEq<Option<L>, Option<R>> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Wrap the types in a result.
    #[inline(always)]
    #[must_use]
    pub const fn in_result<E0, E1>(
        self,
        _error: TypeEq<E0, E1>,
    ) -> TypeEq<Result<L, E0>, Result<R, E1>> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L, R> TypeEq<[L], [R]> {
    /// Get the type of the slice elements.
    #[inline(always)]
    #[must_use]
    pub const fn element(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L, R, const N0: usize, const N1: usize> TypeEq<[L; N0], [R; N1]> {
    /// Get the type of the array elements.
    #[inline(always)]
    #[must_use]
    pub const fn element(self) -> TypeEq<L, R> {
        const {
            assert!(N0 == N1);
        };
        unsafe { TypeEq::new_unchecked() }
    }

    /// Get the type of the array lengths.
    #[inline(always)]
    #[must_use]
    pub const fn len(self) -> TypeEq<Usize<N0>, Usize<N1>> {
        const {
            assert!(N0 == N1);
        };

        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L, R> TypeEq<Option<L>, Option<R>> {
    /// Get the types of the some variant.
    #[inline(always)]
    #[must_use]
    pub const fn some(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Convert the option types to a result.
    #[inline(always)]
    #[must_use]
    pub const fn to_result<E0, E1>(
        self,
        _error: TypeEq<E0, E1>,
    ) -> TypeEq<Result<L, E0>, Result<R, E1>> {
        unsafe { TypeEq::new_unchecked() }
    }
}

impl<L, R, E0, E1> TypeEq<Result<L, E0>, Result<R, E1>> {
    /// Get the types of the ok variants.
    #[inline(always)]
    #[must_use]
    pub const fn ok(self) -> TypeEq<L, R> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Get the types of the error variants.
    #[inline(always)]
    #[must_use]
    pub const fn err(self) -> TypeEq<E0, E1> {
        unsafe { TypeEq::new_unchecked() }
    }

    /// Flip the ok variant types.
    #[inline(always)]
    #[must_use]
    pub const fn flip_ok(self) -> TypeEq<Result<R, E0>, Result<L, E1>> {
        self.ok().flip().in_result(self.err())
    }

    /// Flip the err variant types.
    #[inline(always)]
    #[must_use]
    pub const fn flip_err(self) -> TypeEq<Result<L, E1>, Result<R, E0>> {
        self.ok().in_result(self.err().flip())
    }

    /// Invert the variants.
    #[inline(always)]
    #[must_use]
    pub const fn invert(self) -> TypeEq<Result<E0, L>, Result<E1, R>> {
        self.err().in_result(self.ok())
    }

    /// Convert to an option.
    #[inline(always)]
    #[must_use]
    pub const fn to_option(self) -> TypeEq<Option<L>, Option<R>> {
        self.ok().in_option()
    }
}

impl<L: ?Sized, R: ?Sized> Clone for TypeEq<L, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: ?Sized, R: ?Sized> Copy for TypeEq<L, R> {}
