# derive_less

A macro for templating item declarations.

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
    #[derive(Debug)] pub struct __ { #[apple] pub __:__ }
    #[derive(Clone)] pub enum   __ { #[orange]    __    }

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

You can also mix in derives that only apply to certain items/variants/fields, e.g:

```rust
use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct __ { #[apple] pub __:__  }
    #[derive(Clone)] pub enum   __ { #[orange]    __     }

    struct Foo(i32, i32);
    enum Bar {
        X(#[peanut] i32),
        Y(i32),
    }
    #[derive(PartialEq, PartialOrd)]
    struct Baz(i32, f32, i32);
    enum Qux {
        A(i32),
        B(i32),
        C(i32),
    }
}
```
