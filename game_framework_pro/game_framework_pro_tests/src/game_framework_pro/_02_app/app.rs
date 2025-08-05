use crate::std::rc::*;
use crate::std::cell::*;
use crate::std::option::*;
use crate::game_framework_pro::game::*;

pub struct Application {
    game: Option<Rc<RefCell<Game>>>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            game: Option::Some(Rc::new(RefCell::new(Game::new()))),
        }
    }
    pub fn game(&self) -> Option<Weak<RefCell<Game>>> {
        self.game.as_ref().map(Rc::downgrade)
    }
}
