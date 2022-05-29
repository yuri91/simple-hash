//! A simple interface for hashing rust values
//!
//! This crates defines two traits: [Hasher] and [Hashable].
//!
//! The first represents an hashing algorithm and state, and is currently implemented
//! for [sha2::Sha256].
//!
//! The second is implemented for any rust value that needs to be hashed.
//! An Helper derive macro with the same name is provided to avoid boilerplate.
//!
//! The current set of std types that implement [Hashable] is limited. PRs are welcome.
//!
//! Example use:
//!
//! ```
//! use simple_hash::Hashable;
//! #[derive(Hashable)]
//! struct Foo {
//!     a: u8,
//!     b: u16,
//!     c: Vec<u32>,
//! }
//! let foo = Foo {
//!     a: 8,
//!     b: 99,
//!     c: vec![0,1,2,3],
//! };
//! let res = foo.digest::<sha2::Sha256>();
//! assert_eq!(res, hex_literal::hex!("929863ce588951eae0cc88755216f96951d431e7d15adbb836d8f1960bb65a9d"));
//! ```
//!
use sha2::Sha256;
use sha2::Digest;
use byteorder::{LittleEndian, WriteBytesExt};
use paste::paste;

pub use simple_hash_macro::Hashable;

pub trait Hasher {
    type Output;
    fn update<D: AsRef<[u8]>>(&mut self, data: D);
    fn finish(self) -> Self::Output;
    fn digest<H: Hashable>(data: &H) -> Self::Output;
}

pub trait Hashable {
    fn update<H: Hasher>(&self, h: &mut H);
    fn digest<H: Hasher>(&self) -> <H as Hasher>::Output where Self: Sized {
        H::digest(self)
    }
}

impl Hasher for Sha256 {
    type Output = [u8; 32];

    fn update<D: AsRef<[u8]>>(&mut self, data: D) {
        Digest::update(self, data);
    }
    fn finish(self) -> Self::Output {
        let res = self.finalize();
        let mut out = [0; 32];
        for i in 0..res.len() {
            out[i] = res[i];
        }
        out
    }
    fn digest<H: Hashable>(data: &H) -> Self::Output {
        let mut sha = Sha256::new();
        data.update(&mut sha);
        sha.finish()
    }
}

impl Hashable for u8 {
    fn update<H: Hasher>(&self, h: &mut H) {
        let mut buf = [0u8; std::mem::size_of::<u8>()];
        let mut b = &mut buf[..];
        b.write_u8(*self).unwrap();
        h.update(buf.as_slice());
    }
}
impl Hashable for bool {
    fn update<H: Hasher>(&self, h: &mut H) {
        let mut buf = [0u8; std::mem::size_of::<u8>()];
        let mut b = &mut buf[..];
        b.write_u8(*self as u8).unwrap();
        h.update(buf.as_slice());
    }
}
impl Hashable for i8 {
    fn update<H: Hasher>(&self, h: &mut H) {
        let mut buf = [0u8; std::mem::size_of::<u8>()];
        let mut b = &mut buf[..];
        b.write_i8(*self).unwrap();
        h.update(buf.as_slice());
    }
}

macro_rules! impl_hashable_for {
    ($t:ty) => {
        impl crate::Hashable for $t {
            fn update<H: Hasher>(&self, h: &mut H) {
                let mut buf = [0u8; std::mem::size_of::<$t>()];
                let mut b = &mut buf[..];
                paste! {
                    b.[<write_ $t>]::<LittleEndian>(*self).unwrap();
                }
                h.update(buf.as_slice());
            }
        }
    };
}

impl_hashable_for!(i16);
impl_hashable_for!(u16);
impl_hashable_for!(i32);
impl_hashable_for!(u32);
impl_hashable_for!(i64);
impl_hashable_for!(u64);


impl<T: Hashable> Hashable for Vec<T> {
    fn update<H: Hasher>(&self, h: &mut H) {
        for t in self {
            t.update(h);
        }
    }
}
impl Hashable for String {
    fn update<H: Hasher>(&self, h: &mut H) {
        for t in self.as_bytes() {
            t.update(h);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as simple_hash;

    #[derive(Hashable)]
    struct Foo {
        a: u8,
        b: u16,
        c: Vec<u32>,
    }
    #[test]
    fn test_u8() {
        let res = (9u8).digest::<sha2::Sha256>();
        assert_eq!(res, hex_literal::hex!("2b4c342f5433ebe591a1da77e013d1b72475562d48578dca8b84bac6651c3cb9"));
    }
    #[test]
    fn test_derive() {
        let foo = Foo {
            a: 8,
            b: 99,
            c: vec![0,1,2,3],
        };
        let res = foo.digest::<sha2::Sha256>();
        assert_eq!(res, hex_literal::hex!("929863ce588951eae0cc88755216f96951d431e7d15adbb836d8f1960bb65a9d"));
    }
}
