// https://github.com/segeljakt/derive_less/issues/4

use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct ... { pub(crate) ... }

    struct S {
        f: u8,
    }
}

fn main() {}
