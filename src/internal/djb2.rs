//! Const implementation of [djb2](http://www.cse.yorku.ca/~oz/hash.html)
//!
//! This is the implementation detail behind [`PathId`]s.

/// A simple const time hasher
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Hasher(pub u64);

impl Hasher {
    /// Construct a new `Hasher` with the recommended seed
    pub const fn new() -> Self {
        Self(5381)
    }

    /// Write additional bytes to the hasher
    pub const fn write(&mut self, bytes: &[u8]) {
        let mut i = 0;
        while i < bytes.len() {
            self.0 = self.0.wrapping_mul(33) ^ bytes[i] as u64;
            i += 1;
        }
    }
}
