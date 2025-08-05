#![no_implicit_prelude]

extern crate std;
extern crate game_framework_pro;

use std::rc::*;
use std::cell::*;
use std::option::*;
use game_framework_pro::*;
use game_framework_pro::extensions::*;

include!("00_program.rs");
include!("01_ui.rs");
include!("02_app.rs");
include!("03_game.rs");

pub mod tests_00;
