// https://github.com/segeljakt/derive_less/issues/1

#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    pub struct __ { __:__ }

    struct S {
        f: u8,
    }
}

fn main() {}
