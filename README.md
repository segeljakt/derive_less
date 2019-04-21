# derive_less

A macro for deriving macros. Use it to generate `#[...]` and `pub` code in item declarations.

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

You can also mix in derives that only apply to certain items/variants/fields, e.g:

```rust
use derive_less::derive_less;

derive_less! {
    #[derive(Debug)] pub struct ... { #[apple] pub ...  }
    #[derive(Clone)] pub enum   ... { #[orange]    ...  }

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

# Limitations

Currently supports:
* Structs (All kinds)
* Enums (All kinds)

While structs and enums can take multiple `#[...]`, e.g.:

```rust
#[hello] #[world] struct ... { ...  }
```

Fields and variants accept at most one `#[...]` due to current restrictions in `macro_rules!`. In other words, this does not work at the moment:

```rust
struct ... { #[hello] #[world] ...  }
```

# Future work

Add support for:
* Functions
* Traits
* Trait implementations
* Unions
* Type aliases
* Use declarations

Possibly rewrite `derive_less!` using Rust's procedural macro interface or through a build-script that generates declarative macro code. 
