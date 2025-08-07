#![no_implicit_prelude]
#![forbid(absolute_paths_not_starting_with_crate)]

extern crate std;
extern crate child_of;

pub mod game_framework_pro {
    include!("game_framework_pro/00.program.rs");
    include!("game_framework_pro/01.ui.rs");
    include!("game_framework_pro/02.app.rs");
    include!("game_framework_pro/03.domain.rs");
    pub mod extensions {
        use crate::std::rc::*;
        use crate::std::cell::*;
        use crate::std::option::*;
        use crate::std::clone::*;
        use crate::child_of::*;
        use crate::game_framework_pro::*;
        include!("game_framework_pro.extensions/00.program.rs");
        include!("game_framework_pro.extensions/01.ui.rs");
    }
}
