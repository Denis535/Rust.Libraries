#![cfg(test)]

use crate::std::boxed::*;
use crate::game_framework_pro::*;

#[test]
fn test_00() {
    let _ = Box::new(Program::new());
}
