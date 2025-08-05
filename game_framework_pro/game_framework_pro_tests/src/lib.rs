#![no_implicit_prelude]

extern crate std;
extern crate game_framework_pro;

pub mod framework {
    use crate::std::rc::*;
    use crate::std::cell::*;
    use crate::std::option::*;
    use crate::game_framework_pro::framework::*;
    use crate::game_framework_pro::framework::extensions::*;
    include!("framework/00_program.rs");
    include!("framework/01_ui.rs");
    include!("framework/02_app.rs");
    include!("framework/03_game.rs");
}

pub mod tests_00;
