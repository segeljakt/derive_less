// https://github.com/segeljakt/derive_less/issues/4

#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct __ { pub __:__ }

    struct S<T>
    where
        T: Trait,
    {
        f: T,
    }
}

pub trait Trait {}

fn main() {}
