#![no_implicit_prelude]
#![forbid(absolute_paths_not_starting_with_crate)]

extern crate std;
extern crate game_framework_pro as game_framework_pro_;

pub mod game_framework_pro {
    use crate::std::rc::*;
    use crate::std::cell::*;
    use crate::std::option::*;
    use crate::game_framework_pro_::game_framework_pro::*;
    use crate::game_framework_pro_::game_framework_pro::extensions::*;
    include!("game_framework_pro/00_program.rs");
    include!("game_framework_pro/01_ui.rs");
    include!("game_framework_pro/02_app.rs");
    include!("game_framework_pro/03_game.rs");
}

pub mod tests_00;
