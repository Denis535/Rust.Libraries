use crate::std::rc::*;
use crate::std::cell::*;
use crate::game_framework_pro::ui::*;
use crate::game_framework_pro::app::*;

pub struct ProgramBase {}

impl ProgramBase {
    pub fn new() -> ProgramBase {
        ProgramBase {}
    }
}

pub struct ProgramBase2 {
    base: ProgramBase,
    theme: Rc<RefCell<Theme>>,
    screen: Rc<RefCell<Screen>>,
    router: Rc<RefCell<Router>>,
    application: Rc<RefCell<Application>>,
}

impl ProgramBase2 {
    pub fn new(theme: Rc<RefCell<Theme>>, screen: Rc<RefCell<Screen>>, router: Rc<RefCell<Router>>, application: Rc<RefCell<Application>>) -> ProgramBase2 {
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
    pub fn theme(&self) -> Weak<RefCell<Theme>> {
        Rc::downgrade(&self.theme)
    }
    pub fn screen(&self) -> Weak<RefCell<Screen>> {
        Rc::downgrade(&self.screen)
    }
    pub fn router(&self) -> Weak<RefCell<Router>> {
        Rc::downgrade(&self.router)
    }
    pub fn application(&self) -> Weak<RefCell<Application>> {
        Rc::downgrade(&self.application)
    }
}

pub struct Program {
    base: ProgramBase2,
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
    pub fn base(&self) -> &ProgramBase2 {
        &self.base
    }
    pub fn base_mut(&mut self) -> &mut ProgramBase2 {
        &mut self.base
    }
}
