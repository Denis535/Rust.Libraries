use std::*;
use std::rc::*;
use std::cell::*;
use crate::game_framework_pro::_01_ui::*;
use crate::game_framework_pro::_02_app::*;
use crate::game_framework_pro::_03_game::*;

pub struct Program {
    theme: Rc<RefCell<Theme>>,
    screen: Rc<RefCell<Screen>>,
    router: Rc<RefCell<Router>>,
    application: Rc<RefCell<Application>>,
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
            theme,
            screen,
            router,
            application,
        }
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
