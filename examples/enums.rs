#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)] pub enum __ { #[derive(Clone)] __ }

    enum Foo {
        A,
        B(i32),
        C { x:i32 },
    }

}

fn main() {}
