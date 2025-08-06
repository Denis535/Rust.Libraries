#![cfg(test)]

use crate::child_of::*;

struct Base {}

impl Base {
    pub fn new() -> Base {
        Base {}
    }
}

#[child_of(Base)]
struct Child {}

impl Child {
    pub fn new() -> Child {
        Child {
            base: Base::new(),
        }
    }
    #[allow(dead_code)]
    pub fn base(&self) -> &Base {
        &self.base
    }
    #[allow(dead_code)]
    pub fn base_mut(&mut self) -> &mut Base {
        &mut self.base
    }
}

#[test]
fn test_00() {
    let _ = Child::new();
}
