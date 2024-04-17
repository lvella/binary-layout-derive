use binary_layout_derive::LayoutAsEnumRepr;
use std::convert::{TryFrom, From};

#[derive(LayoutAsEnumRepr)]
#[repr(u32)]
enum MyEnum {
    Variant1 = 1,
    Variant2 = 2,
    Variant3 = 3,
}

impl TryFrom<u32> for MyEnum {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MyEnum::Variant1),
            2 => Ok(MyEnum::Variant2),
            3 => Ok(MyEnum::Variant3),
            _ => Err(()),
        }
    }
}

impl From<MyEnum> for u32 {
    fn from(value: MyEnum) -> u32 {
        value as u32
    }
}

fn main() {}
