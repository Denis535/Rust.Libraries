pub struct Theme {
    #[allow(dead_code)]
    base: ThemeBase2<Router, Application>,
}

impl Theme {
    pub fn new(router: Weak<RefCell<Router>>, application: Weak<RefCell<Application>>) -> Theme {
        Theme {
            base: ThemeBase2::new(router, application),
        }
    }
}

pub struct Screen {
    #[allow(dead_code)]
    base: ScreenBase2<Router, Application>,
}

impl Screen {
    pub fn new(router: Weak<RefCell<Router>>, application: Weak<RefCell<Application>>) -> Screen {
        Screen {
            base: ScreenBase2::new(router, application),
        }
    }
}

pub struct Router {
    base: RouterBase2<Theme, Screen, Application>,
}

impl Router {
    pub fn new(application: Weak<RefCell<Application>>) -> Router {
        Router {
            base: RouterBase2::new(application)
        }
    }
    pub fn init(&mut self, theme: Weak<RefCell<Theme>>, screen: Weak<RefCell<Screen>>) {
        self.base.set_theme(theme);
        self.base.set_screen(screen);
    }
}
