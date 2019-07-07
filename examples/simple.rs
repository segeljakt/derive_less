#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct __ { #[derive(Debug)] pub __:__ }
    #[derive(Clone)] pub enum   __ { #[derive(Debug)]     __    }

    struct Foo;
    struct Bar(f32, i32);
    struct Baz {
        a: i32,
        b: i32,
    }
    enum Qux {
        A(i32),
        B,
        C(i32),
    }
    enum Zot {
        A {
            a: i32,
        },
        B {
            b: i32,
        },
        C {
            c: i32
        }
    }
}

fn main() {}
