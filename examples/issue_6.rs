// https://github.com/segeljakt/derive_less/issues/6

#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct __ { pub(crate) __:__ }

    struct S {
        f: u8,
    }
}

fn main() {}
