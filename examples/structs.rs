#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)] pub struct __ { #[derive(Clone)] pub __:__ }

    struct Foo {
        #[derive(PartialEq, PartialOrd)]
        a: i32,
    }
    #[derive(PartialEq, PartialOrd)]
    struct Bar (
        f32,
    );
    struct Baz;
}

fn main() {}
