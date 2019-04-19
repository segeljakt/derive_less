
use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)]  pub enum ... { #[derive(Clone)] ... }

    enum Foo {
        A,
        B,
        C,
    }

    enum Bar {
        A(i32),
        B(i32,f32),
        C(i32),
    }

    enum Baz {
        A {
            a: i32,
        },
        B {
            b: i32,
            p: f32,
        },
        C {
            c: i32
        },
    }
}

fn main() {}
