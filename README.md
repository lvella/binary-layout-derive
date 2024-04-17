This procedural macro crate provides the `#[derive(LayoutAsEnumRepr)]` macro, to
automatically implement
[`binary_layout::LayoutAs<U>`](https://docs.rs/binary-layout/latest/binary_layout/trait.LayoutAs.html)
for `#[repr(U)]` enums, provided it implements both `TryFrom<U>` and `Into<U>`.
For convenience, these two traits can be automatically derived by using the
[`num_enum`](https://crates.io/crates/num_enum) crate, but it also works if they
are manually provided.

License: MIT OR Apache-2.0
