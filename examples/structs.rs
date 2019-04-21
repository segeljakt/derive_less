use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)] pub struct ... { #[derive(Clone)] pub ... }

    struct Foo {
        #[derive(PartialEq, PartialOrd)]
        a: i32,
    }
    #[derive(PartialEq, PartialOrd)]
    struct Bar (
        f32,
    );
    struct Baz;
}

fn main() {}
