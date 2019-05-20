// https://github.com/segeljakt/derive_less/issues/3

use derive_less::derive_less;

trait Trait {}

derive_less! {
    #[derive(Debug)] pub struct ... { pub ... }

    struct S<T: Trait> {
        f: T,
    }
}

fn main() {}
