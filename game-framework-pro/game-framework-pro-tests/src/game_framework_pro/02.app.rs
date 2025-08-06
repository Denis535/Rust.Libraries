#[child_of(ApplicationBase)]
pub struct Application {
    game: Option<Rc<RefCell<Game>>>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            base: ApplicationBase::new(),
            game: Option::Some(Rc::new(RefCell::new(Game::new()))),
        }
    }
    pub fn game(&self) -> Option<Weak<RefCell<Game>>> {
        self.game.as_ref().map(Rc::downgrade)
    }
}
