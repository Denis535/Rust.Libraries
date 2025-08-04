use std::*;
use std::rc::*;
use std::cell::*;
use crate::game_framework_pro::program::*;
use crate::game_framework_pro::ui::*;
use crate::game_framework_pro::game::*;

pub struct Application {
    game: Option<Rc<RefCell<Game>>>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            game: Some(Rc::new(RefCell::new(Game::new()))),
        }
    }
    pub fn game(&self) -> Option<Weak<RefCell<Game>>> {
        self.game.as_ref().map(Rc::downgrade)
    }
}
