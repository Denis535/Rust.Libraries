#![no_implicit_prelude]

extern crate std;

pub mod game_framework_pro {
    use crate::std::rc::*;
    use crate::std::cell::*;
    use crate::std::option::*;
    use crate::std::clone::*;

    include!("game_framework_pro/_00_program/program.rs");
    include!("game_framework_pro/_01_ui/ui.rs");
    include!("game_framework_pro/_02_app/app.rs");
    include!("game_framework_pro/_03_game/game.rs");
    pub mod tests_00;
}
