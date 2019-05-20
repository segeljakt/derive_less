// https://github.com/segeljakt/derive_less/issues/1

use derive_less::derive_less;

derive_less! {
    pub struct ... { ... }

    struct S {
        f: u8,
    }
}

fn main() {}
