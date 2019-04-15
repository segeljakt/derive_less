# derive_less

A library for generating `#[...]` and `pub` code in item declarations.

# Example

Suppose you needed the following code:

```rust
#[derive(Debug)]
pub struct Foo(
    #[apple]
    pub i32,
    #[apple]
    pub i32,
);
#[derive(Clone)]
pub enum Bar {
    #[orange]
    X(i32),
    #[orange]
    Y(i32),
}
#[derive(Debug)]
pub struct Baz(
    #[apple]
    pub i32,
    #[apple]
    pub f32,
    #[apple]
    pub i32,
);
#[derive(Clone)]
pub enum Qux {
    #[orange]
    A(i32),
    #[orange]
    B(i32),
    #[orange]
    C(i32),
}
```

Instead of typing out `#[orange]`, `#[apple]`, and `pub`, repeatedly for each item declaration, you could simply write:

```rust
use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct ... { #[apple] pub ...  }
    #[derive(Clone)] pub enum   ... { #[orange]    ...  }

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
```

# Caveats

Currently only supports and tuple structs and enums with unnamed fields.

Fields and variants only accept one `#[...]` due to current limitations in `macro_rules!`.

# Future work

Handle more cases by either rewriting the `derive_less!` macro using Rust's procedural macro interface or through a build-script that generates declarative macro code. 
