use std::*;
use std::rc::*;
use std::cell::*;
use crate::game_framework_pro::program::*;
use crate::game_framework_pro::app::*;
use crate::game_framework_pro::game::*;

pub struct Theme {
    router: Weak<RefCell<Router>>,
    application: Weak<RefCell<Application>>,
}

impl Theme {
    pub fn new(router: Weak<RefCell<Router>>, application: Weak<RefCell<Application>>) -> Theme {
        Theme {
            router,
            application,
        }
    }
    pub fn router(&self) -> Weak<RefCell<Router>> {
        self.router.clone()
    }
    pub fn application(&self) -> Weak<RefCell<Application>> {
        self.application.clone()
    }
}

pub struct Screen {
    router: Weak<RefCell<Router>>,
    application: Weak<RefCell<Application>>,
}

impl Screen {
    pub fn new(router: Weak<RefCell<Router>>, application: Weak<RefCell<Application>>) -> Screen {
        Screen {
            router,
            application,
        }
    }
    pub fn router(&self) -> Weak<RefCell<Router>> {
        self.router.clone()
    }
    pub fn application(&self) -> Weak<RefCell<Application>> {
        self.application.clone()
    }
}

pub struct Router {
    theme: Option<Weak<RefCell<Theme>>>,
    screen: Option<Weak<RefCell<Screen>>>,
    application: Weak<RefCell<Application>>,
}

impl Router {
    pub fn new(application: Weak<RefCell<Application>>) -> Router {
        Router {
            theme: None,
            screen: None,
            application,
        }
    }
    pub fn theme(&self) -> Weak<RefCell<Theme>> {
        self.theme.as_ref().unwrap().clone()
    }
    pub fn set_theme(&mut self, theme: Weak<RefCell<Theme>>) {
        self.theme = Some(theme);
    }
    pub fn screen(&self) -> Weak<RefCell<Screen>> {
        self.screen.as_ref().unwrap().clone()
    }
    pub fn set_screen(&mut self, screen: Weak<RefCell<Screen>>) {
        self.screen = Some(screen);
    }
    pub fn application(&self) -> Weak<RefCell<Application>> {
        self.application.clone()
    }
}
