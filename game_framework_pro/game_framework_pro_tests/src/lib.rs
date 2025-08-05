#![no_implicit_prelude]

extern crate std;
extern crate game_framework_pro;

use std::rc::*;
use std::cell::*;
use std::option::*;
use game_framework_pro::*;

include!("_00_program/program.rs");
include!("_01_ui/ui.rs");
include!("_02_app/app.rs");
include!("_03_game/game.rs");
pub mod tests_00;
