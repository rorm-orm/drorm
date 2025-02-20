//! Helper function for the `const_concat` macros.

use std::mem::MaybeUninit;
/// Syntactic sugar for const functions
macro_rules! sugar {
    (for $i:ident in $slice:ident $then:block) => {
        let mut slices = $slice;
        while let [$i, tail @ ..] = slices {
            slices = tail;
            $then
        }
    };
}

/// A UTF-8–encoded, growable string for const expressions.
///
/// ## Required invariant
/// Every instance, which is observable from the public API, must have valid utf8 in its bytes.
#[derive(Copy, Clone, Debug)]
pub struct ConstString<const MAX_LEN: usize> {
    len: usize,
    bytes: [u8; MAX_LEN],
}

impl<const MAX_LEN: usize> ConstString<MAX_LEN> {
    /// Creates a new empty `ConstString`.
    #[expect(
        clippy::new_without_default,
        reason = "If you are able to call `Default`, then please you should use `String` instead of this type"
    )]
    pub const fn new() -> Self {
        Self {
            len: 0,
            bytes: [0; MAX_LEN],
        }
    }

    /// Extracts a string slice containing the entire `ConstString`.
    pub const fn as_str<'a>(&'a self) -> &'a str {
        let len = if self.len < MAX_LEN {
            self.len
        } else {
            MAX_LEN
        };

        // SAFETY: - `&self.bytes` is a valid reference
        //         - `len` is not larger than `self.bytes.len()`
        //         - the lifetime `'a` is bound to the input and output
        let bytes = unsafe { std::slice::from_raw_parts::<'a, u8>(self.bytes.as_ptr(), len) };

        // SAFETY:
        // `push_str` only adds a complete `str` or aborts.
        // Therefore whenever you've got an instance of `ConstString`, its bytes must form valid utf8
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }

    /// Appends a given string slice onto the end of this `ConstString`,
    ///
    /// returning `None` if the resulting string would be larger than `MAX_LEN`.
    pub const fn push_str(mut self, string: &str) -> Option<Self> {
        let bytes = string.as_bytes();
        sugar! {
            for byte in bytes {
                if self.len == MAX_LEN {
                    return None;
                }

                self.bytes[self.len] = *byte;
                self.len += 1;
            }
        }
        Some(self)
    }
}

impl ConstString<1024> {
    pub(crate) const OOM_ERROR: Self = {
        match ConstString::new().push_str("The error message is longer than 1024 bytes. Try using shorter names or contact the library authors.") {
            Some(ok) => ok,
            None => unreachable!(), // The error message is less than 1024 bytes
        }
    };

    pub(crate) const fn error(strings: &[&str]) -> Self {
        let mut string = Self::new();
        sugar! {
            for slice in strings {
                match string.push_str(slice) {
                    Some(ok) => {string = ok;},
                    None => return Self::OOM_ERROR,
                }
            }
        }
        string
    }
}

/// A contiguous growable array type for const expressions.
///
/// ## Required invariant
/// For every `ConstVec`, which is observable from the public API,
/// the first `self.len` elements of `self.ìnstances` have to be initialised.
#[derive(Copy, Clone, Debug)]
pub struct ConstVec<T: Copy, const MAX_LEN: usize> {
    len: usize,
    elems: [MaybeUninit<T>; MAX_LEN],
}

impl<T: Copy, const MAX_LEN: usize> ConstVec<T, MAX_LEN> {
    /// Creates a new empty `ConstString`.
    #[expect(
        clippy::new_without_default,
        reason = "If you are able to call `Default`, then please you should use `Vec` instead of this type"
    )]
    pub const fn new() -> Self {
        Self {
            len: 0,
            elems: {
                // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
                unsafe { MaybeUninit::<[MaybeUninit<T>; MAX_LEN]>::uninit().assume_init() }
            },
        }
    }

    /// Extracts a slice containing the entire vector.
    pub const fn as_slice<'a>(&'a self) -> &'a [T] {
        let len = if self.len < MAX_LEN {
            self.len
        } else {
            MAX_LEN
        };

        // SAFETY: - `&self.bytes` is a valid reference
        //         - `len` is not larger than `self.bytes.len()`
        //         - the lifetime `'a` is bound to the input and output
        //         - the first `len` elems are initialised
        unsafe { std::slice::from_raw_parts::<'a, T>(self.elems[0].as_ptr(), len) }
    }

    /// Copies and appends all elements in a slice to the `ConstVec`,
    ///
    /// returning `None` if the resulting string would be larger than `MAX_LEN`.
    pub const fn extend_from_slice(mut self, slice: &[T]) -> Option<Self> {
        sugar! {
            for elem in slice {
                if self.len == MAX_LEN {
                    return None;
                }

                self.elems[self.len] = MaybeUninit::new(*elem);
                self.len += 1;
            }
        }
        Some(self)
    }
}

impl ConstVec<&'static str, 1024> {
    pub(crate) const OOM_ERROR: ConstString<1024> = {
        match ConstString::new().push_str("rorm doesn't support more than 1024 columns") {
            Some(ok) => ok,
            None => unreachable!(), // The error message is less than 1024 bytes
        }
    };

    #[doc(hidden)]
    #[allow(clippy::result_large_err)] // there is no heap to escape to in const
    pub const fn columns(columns: &[&[&'static str]]) -> Result<Self, ConstString<1024>> {
        let mut vec = Self::new();
        sugar! {
            for column in columns {
                match vec.extend_from_slice(column) {
                    Some(some) => {vec = some;},
                    None => return Err(Self::OOM_ERROR),
                }
            }
        }
        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    use super::ConstString;

    #[test]
    fn error() {
        assert_eq!(
            ConstString::error(&["Hello ", "world"]).as_str(),
            "Hello world"
        );
    }
}
