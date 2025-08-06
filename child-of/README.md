# Overview

A library that allows you to make one struct a child of another.
The ```#[child_of(Base)]``` attribute clearly marks your struct as a child of base struct and implicitly declares a private ```base``` field.

# Example

```
use crate::child_of::*;

struct Base {}

impl Base {
    pub fn new() -> Base {
        Base {}
    }
}

#[child_of(Base)] This attribute marks the "Child" struct as a child of the "Base" struct.
struct Child {
    base: Base, // This field is implicitly declared by the #[child_of(Base)] attribute.
}

impl Child {
    pub fn new() -> Child {
        Child {
            base: Base::new(),
        }
    }
    pub fn base(&self) -> &Base {
        &self.base
    }
    pub fn base_mut(&mut self) -> &mut Base {
        &mut self.base
    }
}
```

# Links

- https://github.com/Denis535/Rust.Libraries/tree/main/child-of
- https://crates.io/crates/child-of
