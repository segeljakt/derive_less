use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct ... { #[derive(Debug)] pub ...  }
    #[derive(Clone)] pub enum   ... { #[derive(Debug)]    ...  }

    struct Foo(i32, i32);
    enum Bar {
        X(i32),
        Y(i32),
    }
    struct Baz(i32, f32, i32);
    enum Qux {
        A(i32),
        B(i32),
        C(i32),
    }
}


