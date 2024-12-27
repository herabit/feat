#![allow(dead_code)]

pub(crate) mod mem {
    /// A thin wrapper around [`core::mem::transmute_copy`]
    #[inline(always)]
    #[must_use]
    pub(crate) const unsafe fn transmute_unchecked<Src, Dst>(src: Src) -> Dst {
        unsafe { ::core::mem::transmute_copy(&::core::mem::ManuallyDrop::new(src)) }
    }
}

pub(crate) mod array {
    use core::mem::MaybeUninit;

    #[inline(always)]
    pub(crate) const fn each_ref<'a, T, const N: usize>(arr: &'a [T; N]) -> [&'a T; N] {
        let mut remaining: &'a [T] = arr;

        let output: [MaybeUninit<&'a T>; N] = {
            let mut output = [MaybeUninit::uninit(); N];
            let mut out: &mut [MaybeUninit<&'a T>] = &mut output;

            // Both slices are always going to be exactly the same size.
            //
            // Hence this will always initialize `output` with valid references.
            while let ([o, o_rest @ ..], [r, r_rest @ ..]) = (out, remaining) {
                // MaybeUninit::write is not stable in const yet.
                *o = MaybeUninit::<&'a T>::new(r);

                out = o_rest;
                remaining = r_rest;
            }

            output
        };

        // Circumvent some issues with transmute.
        //
        // We know that all of the references within `output` have been initialized
        // with valid references.
        unsafe { super::mem::transmute_unchecked(output) }
    }

    #[inline(always)]
    pub(crate) const fn each_mut<'a, T, const N: usize>(arr: &'a mut [T; N]) -> [&'a mut T; N] {
        let mut remaining: &'a mut [T] = arr;

        let output: [MaybeUninit<&'a mut T>; N] = {
            let mut output = [const { MaybeUninit::uninit() }; N];
            let mut out: &mut [MaybeUninit<&'a mut T>] = &mut output;

            // Both slices are always going to be exactly the same size.
            //
            // Hence this will always initialize `output` with valid references.
            //
            // Additionally, these references will not be aliased at all due to us
            // treating `remaining` like a mutable slice's iterator, so once an element is taken
            // from `remaining`, it can only be observed in the output array.
            while let ([o, o_rest @ ..], [r, r_rest @ ..]) = (out, remaining) {
                *o = MaybeUninit::<&'a mut T>::new(r);

                out = o_rest;
                remaining = r_rest;
            }

            output
        };

        // Circumvent some issues with transmute.
        //
        // We know that all of the references within `output` have been initialized and not aliased.
        unsafe { super::mem::transmute_unchecked(output) }
    }
}

pub(crate) mod string {
    use super::bytes;

    #[inline(always)]
    pub(crate) const fn is_eq(lhs: &str, rhs: &str) -> bool {
        bytes::is_eq(lhs.as_bytes(), rhs.as_bytes())
    }

    #[inline(always)]
    pub(crate) const fn is_ne(lhs: &str, rhs: &str) -> bool {
        bytes::is_ne(lhs.as_bytes(), rhs.as_bytes())
    }

    #[inline(always)]
    pub(crate) const fn starts_with(prefix: &str, haystack: &str) -> bool {
        bytes::starts_with(prefix.as_bytes(), haystack.as_bytes())
    }

    #[inline(always)]
    pub(crate) const fn ends_with(suffix: &str, haystack: &str) -> bool {
        bytes::ends_with(suffix.as_bytes(), haystack.as_bytes())
    }

    #[inline(always)]
    pub(crate) const fn strip_prefix<'a>(prefix: &str, haystack: &'a str) -> Option<&'a str> {
        match bytes::strip_prefix(prefix.as_bytes(), haystack.as_bytes()) {
            // SAFETY: If we can strip a UTF-8 string from the start of another, the result is
            //         also a UTF-8 string.
            Some(x) => Some(unsafe { ::core::str::from_utf8_unchecked(x) }),
            None => None,
        }
    }

    #[inline(always)]
    pub(crate) const fn strip_suffix<'a>(suffix: &str, haystack: &'a str) -> Option<&'a str> {
        match bytes::strip_suffix(suffix.as_bytes(), haystack.as_bytes()) {
            // SAFETY: If we can strip UTF-8 string from the end of another, the result is
            //         also a UTF-8 string.
            Some(x) => Some(unsafe { ::core::str::from_utf8_unchecked(x) }),
            None => None,
        }
    }

    #[inline(always)]
    pub(crate) const fn find(needle: &str, haystack: &str) -> Option<usize> {
        bytes::find(needle.as_bytes(), haystack.as_bytes())
    }

    #[inline(always)]
    pub(crate) const fn split<'a>(needle: &str, haystack: &'a str) -> Option<(&'a str, &'a str)> {
        match bytes::split(needle.as_bytes(), haystack.as_bytes()) {
            // SAFETY: Getting the surrounding parts of a valid UTF-8 string is always
            //         going to result in two other valid UTF-8 strings.
            Some((a, b)) => Some(unsafe {
                (
                    ::core::str::from_utf8_unchecked(a),
                    ::core::str::from_utf8_unchecked(b),
                )
            }),
            None => None,
        }
    }

    #[inline(always)]
    pub(crate) const fn split_char(needle: char, haystack: &str) -> Option<(&str, &str)> {
        #[inline(always)]
        const fn to_utf8<const LEN: usize>(buf: &mut [u8; LEN], c: char) -> &mut str {
            assert!(c.len_utf8() == LEN);
            c.encode_utf8(buf)
        }

        const unsafe fn inner<'a, const LEN: usize>(
            needle: &str,
            haystack: &'a str,
        ) -> Option<(&'a str, &'a str)> {
            unsafe { ::core::hint::assert_unchecked(needle.len() == LEN) };

            split(needle, haystack)
        }

        match needle.len_utf8() {
            1 => unsafe { inner::<1>(to_utf8(&mut [0; 1], needle), haystack) },
            2 => unsafe { inner::<2>(to_utf8(&mut [0; 2], needle), haystack) },
            3 => unsafe { inner::<3>(to_utf8(&mut [0; 3], needle), haystack) },
            4 => unsafe { inner::<4>(to_utf8(&mut [0; 4], needle), haystack) },
            _ => unreachable!(),
        }
    }
}

pub(crate) mod bytes {
    #[inline(always)]
    pub(crate) const fn is_eq(lhs: &[u8], rhs: &[u8]) -> bool {
        #[inline(always)]
        const fn inner(lhs: &[u8], rhs: &[u8]) -> bool {
            match (lhs, rhs) {
                ([], []) => true,
                ([l, lhs @ ..], [r, rhs @ ..]) => (*l == *r) & inner(lhs, rhs),
                _ => unreachable!(),
            }
        }

        lhs.len() == rhs.len() && inner(lhs, rhs)
    }

    #[inline(always)]
    pub(crate) const fn is_ne(lhs: &[u8], rhs: &[u8]) -> bool {
        #[inline(always)]
        const fn inner(lhs: &[u8], rhs: &[u8]) -> bool {
            match (lhs, rhs) {
                ([], []) => false,
                ([l, lhs @ ..], [r, rhs @ ..]) => (*l != *r) | inner(lhs, rhs),
                _ => unreachable!(),
            }
        }

        lhs.len() != rhs.len() || inner(lhs, rhs)
    }

    #[inline(always)]
    pub(crate) const fn starts_with(prefix: &[u8], haystack: &[u8]) -> bool {
        prefix.len() <= haystack.len() && {
            let (haystack, _) = haystack.split_at(prefix.len());

            is_eq(haystack, prefix)
        }
    }

    #[inline(always)]
    pub(crate) const fn ends_with(suffix: &[u8], haystack: &[u8]) -> bool {
        suffix.len() <= haystack.len() && {
            let (_, haystack) = haystack.split_at(haystack.len() - suffix.len());

            is_eq(haystack, suffix)
        }
    }

    #[inline(always)]
    pub(crate) const fn strip_prefix<'a>(prefix: &[u8], haystack: &'a [u8]) -> Option<&'a [u8]> {
        if starts_with(prefix, haystack) {
            let (_, haystack) = haystack.split_at(prefix.len());

            Some(haystack)
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) const fn strip_suffix<'a>(suffix: &[u8], haystack: &'a [u8]) -> Option<&'a [u8]> {
        if ends_with(suffix, haystack) {
            let (haystack, _) = haystack.split_at(haystack.len() - suffix.len());

            Some(haystack)
        } else {
            None
        }
    }

    #[inline(always)]
    const fn inc(x: &mut usize) -> &mut usize {
        *x += 1;

        x
    }

    #[inline(always)]
    pub(crate) const fn find(needle: &[u8], haystack: &[u8]) -> Option<usize> {
        #[inline(always)]
        const fn inner(index: &mut usize, needle: &[u8], haystack: &[u8]) -> bool {
            let Some((_, rest)) = haystack.split_at_checked(*index) else {
                return false;
            };

            let Some((prefix, _)) = rest.split_at_checked(needle.len()) else {
                return false;
            };

            is_eq(prefix, needle) || inner(inc(index), needle, haystack)
        }

        let mut index = 0_usize;

        if inner(&mut index, needle, haystack) {
            Some(index)
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) const fn split<'a>(
        needle: &[u8],
        haystack: &'a [u8],
    ) -> Option<(&'a [u8], &'a [u8])> {
        let Some(start) = find(needle, haystack) else {
            return None;
        };

        // SAFETY: If `find` succeeds, then we know that `start..start + needle.len()` is a
        //         valid subslice of `haystack`.
        let (before, rest) = unsafe { haystack.split_at_unchecked(start) };
        let (_rest, after) = unsafe { rest.split_at_unchecked(needle.len()) };

        Some((before, after))
    }

    const _: () = assert!(matches!(find(b"_", b"help_me"), Some(4)));
    const _: () = assert!(matches!(split(b"_", b"help_me"), Some((b"help", b"me"))));
}
