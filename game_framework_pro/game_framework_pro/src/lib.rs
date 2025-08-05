#![no_implicit_prelude]

extern crate std;

pub mod framework {
    include!("framework/00_program.rs");
    include!("framework/01_ui.rs");
    include!("framework/02_app.rs");
    include!("framework/03_game.rs");
    pub mod extensions {
        use crate::framework::*;
        use crate::std::rc::*;
        use crate::std::cell::*;
        use crate::std::option::*;
        use crate::std::clone::*;
        include!("framework.extensions/00_program.rs");
        include!("framework.extensions/01_ui.rs");
    }
}
