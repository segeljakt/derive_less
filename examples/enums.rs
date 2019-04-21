// #![feature(trace_macros)]

// trace_macros!(true);

use derive_less::derive_less;

derive_less! {
    #[derive(Clone)] #[derive(Debug)]  pub enum ... { #[derive(Clone)] ... }

    enum Foo {
        A,
        B(i32),
        C { x:i32 },
    }

}

fn main() {}
