# Simple Hash

<!-- cargo-rdme start -->

A simple interface for hashing rust values

This crates defines two traits: [Hasher] and [Hashable].

The first represents an hashing algorithm and state, and is currently implemented
for [sha2::Sha256].

The second is implemented for any rust value that needs to be hashed.
An Helper derive macro with the same name is provided to avoid boilerplate.

The current set of std types that implement [Hashable] is limited. PRs are welcome.

Example use:

```rust
use simple_hash::Hashable;
#[derive(Hashable)]
struct Foo {
    a: u8,
    b: u16,
    c: Vec<u32>,
}
let foo = Foo {
    a: 8,
    b: 99,
    c: vec![0,1,2,3],
};
let res = foo.digest::<sha2::Sha256>();
assert_eq!(res, hex_literal::hex!("929863ce588951eae0cc88755216f96951d431e7d15adbb836d8f1960bb65a9d"));
```

<!-- cargo-rdme end -->
