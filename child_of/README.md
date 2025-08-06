# Overview

A library that allows you to make one struct a child of another.

# Example

```
use crate::child_of::*;

struct Base {}

impl Base {
    pub fn new() -> Base {
        Base {}
    }
}

#[child_of(Base)]
struct Child {
    //base: Base, - field is implicitly declared by #[child_of(Base)] attribute
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

- https://github.com/Denis535/Rust.Libraries/tree/main/child_of
- https://crates.io/crates/child_of
