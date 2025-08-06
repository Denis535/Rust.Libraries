#![cfg(test)]

use crate::child_of::*;

struct Base {}

#[child_of(Base)]
struct Child {}

impl Child {
    pub fn new() -> Child {
        Child {
            base: Base {},
        }
    }
}

#[test]
fn test_00() {
    let _ = Child::new();
}
