pub struct Program {
    base: ProgramBase2<Theme, Screen, Router, Application>,
}

impl Program {
    pub fn new() -> Program {
        let application = Rc::new(RefCell::new(Application::new()));
        let router = Rc::new(RefCell::new(Router::new(Rc::downgrade(&application))));
        let screen = Rc::new(RefCell::new(Screen::new(Rc::downgrade(&router), Rc::downgrade(&application))));
        let theme = Rc::new(RefCell::new(Theme::new(Rc::downgrade(&router), Rc::downgrade(&application))));
        router.borrow_mut().set_theme(Rc::downgrade(&theme));
        router.borrow_mut().set_screen(Rc::downgrade(&screen));
        Program {
            base: ProgramBase2::new(theme, screen, router, application)
        }
    }
    fn base(&self) -> &ProgramBase2<Theme, Screen, Router, Application> {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ProgramBase2<Theme, Screen, Router, Application> {
        &mut self.base
    }
}
