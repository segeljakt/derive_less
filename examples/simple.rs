use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct ... { #[derive(Debug)] pub ... }
    #[derive(Clone)] pub enum   ... { #[derive(Debug)]     ... }

    struct Foo;
    struct Bar(f32, i32);
    struct Baz {
        a: i32,
        b: i32,
    }
    enum Qux {
        A(i32),
        B,
        C(i32),
    }
    enum Zot {
        A {
            a: i32,
        },
        B {
            b: i32,
        },
        C {
            c: i32
        }
    }
}
 
fn main() {}
