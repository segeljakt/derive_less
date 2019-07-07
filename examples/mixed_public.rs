#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct __ { #[derive(Clone)] pub __:__ }

    struct Foo;

    struct Bar(i32, f32, u32);

    struct Baz {
        a: i32,
        b: f32,
        c: u32,
    }

    enum Zot {
        A,
        B(i32,f32),
        C {
            a: i32,
            b: f32,
            c: i32
        }
    }
}

fn main() {}
