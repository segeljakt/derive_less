use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)] pub struct ... { #[derive(Clone)] pub ... }

    struct Foo(i32);
    #[derive(PartialEq, PartialOrd)]
    struct Bar(f32);
    struct Baz(
        #[derive(PartialEq, PartialOrd)]
        i32,
        f32,
    );
}

fn main() {}
