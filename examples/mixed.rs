use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct ... { #[derive(Clone)] pub ... }

    struct Foo;
    #[derive(PartialEq, PartialOrd)]
    struct Bar(i32);
    struct Baz(f32);
    #[derive(PartialEq, PartialOrd)]
    struct Qux;
}

// Expands to:
//
// #[derive(Debug)]
// pub struct Foo;
// #[derive(Debug)]
// #[derive(PartialEq, PartialOrd)]
// pub struct Bar(#[derive(Clone) pub i32);
// pub struct Baz(pub i32);
// pub struct Qux;

fn main() {}
