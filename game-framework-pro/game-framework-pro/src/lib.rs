#![no_implicit_prelude]
#![forbid(absolute_paths_not_starting_with_crate)]

extern crate std;

pub mod game_framework_pro {
    include!("game_framework_pro/00_program.rs");
    include!("game_framework_pro/01_ui.rs");
    include!("game_framework_pro/02_app.rs");
    include!("game_framework_pro/03_game.rs");
    pub mod extensions {
        use crate::game_framework_pro::*;
        use crate::std::rc::*;
        use crate::std::cell::*;
        use crate::std::option::*;
        use crate::std::clone::*;
        include!("game_framework_pro.extensions/00_program.rs");
        include!("game_framework_pro.extensions/01_ui.rs");
    }
}
