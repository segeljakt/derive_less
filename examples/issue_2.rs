// https://github.com/segeljakt/derive_less/issues/2

#![allow(dead_code)]
#![allow(unused_attributes)]

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] struct __ { pub __:__ }

    struct S {
        f: u8,
    }
}

fn main() {}
