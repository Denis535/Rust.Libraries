pub struct ProgramBase {}

impl ProgramBase {
    pub fn new() -> ProgramBase {
        ProgramBase {}
    }
}

pub struct ProgramBase2<TTheme, TScreen, TRouter, TApplication> {
    base: ProgramBase,
    theme: Rc<RefCell<TTheme>>,
    screen: Rc<RefCell<TScreen>>,
    router: Rc<RefCell<TRouter>>,
    application: Rc<RefCell<TApplication>>,
}

impl<TTheme, TScreen, TRouter, TApplication> ProgramBase2<TTheme, TScreen, TRouter, TApplication> {
    pub fn new(theme: Rc<RefCell<TTheme>>, screen: Rc<RefCell<TScreen>>, router: Rc<RefCell<TRouter>>, application: Rc<RefCell<TApplication>>) -> ProgramBase2<TTheme, TScreen, TRouter, TApplication> {
        ProgramBase2 {
            base: ProgramBase::new(),
            theme,
            screen,
            router,
            application,
        }
    }
    pub fn base(&self) -> &ProgramBase {
        &self.base
    }
    pub fn base_mut(&mut self) -> &mut ProgramBase {
        &mut self.base
    }
    pub fn theme(&self) -> Weak<RefCell<TTheme>> {
        Rc::downgrade(&self.theme)
    }
    pub fn screen(&self) -> Weak<RefCell<TScreen>> {
        Rc::downgrade(&self.screen)
    }
    pub fn router(&self) -> Weak<RefCell<TRouter>> {
        Rc::downgrade(&self.router)
    }
    pub fn application(&self) -> Weak<RefCell<TApplication>> {
        Rc::downgrade(&self.application)
    }
}
