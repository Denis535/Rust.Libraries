#![no_implicit_prelude]

extern crate std;

include!("00_program.rs");
include!("01_ui.rs");
include!("02_app.rs");
include!("03_game.rs");

pub mod extensions {
    use crate::std::rc::*;
    use crate::std::cell::*;
    use crate::std::option::*;
    use crate::std::clone::*;
    use crate::*;

    include!("extensions/00_program.rs");
    include!("extensions/01_ui.rs");
}
