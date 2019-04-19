
use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)]  pub struct ... { #[derive(Clone)] pub ... }

    struct Foo;
    #[derive(PartialEq, PartialOrd)]
    struct Bar;
    struct Baz;
}

fn main() {}
