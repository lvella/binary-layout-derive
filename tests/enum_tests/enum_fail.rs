use binary_layout_derive::LayoutAsEnumRepr;

#[derive(LayoutAsEnumRepr)]
#[repr(u32)]
enum MyBadEnum {
    Variant1 = 1,
}

// Missing Into<u32> and TryFrom<u32> implementations, should fail.

fn main() {}
