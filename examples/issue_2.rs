// https://github.com/segeljakt/derive_less/issues/2

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] struct ... { pub ... }

    struct S {
        f: u8,
    }
}

fn main() {}
