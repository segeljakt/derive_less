use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)] pub struct ... { #[derive(Clone)] pub ... }

    struct Foo {
        a: i32,
    }
    #[derive(PartialEq, PartialOrd)]
    struct Bar {
        a: f32,
    }
    struct Baz {
        #[derive(PartialEq, PartialOrd)]
        a: i32,
        b: f32,
    }
}

fn main() {}
