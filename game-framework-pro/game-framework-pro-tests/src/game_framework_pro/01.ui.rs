#[child_of(ThemeBase2<Router, Application>)]
pub struct Theme {}

impl Theme {
    pub fn new(router: Weak<RefCell<Router>>, application: Weak<RefCell<Application>>) -> Theme {
        Theme {
            base: ThemeBase2::new(router, application),
        }
    }
}

#[child_of(ScreenBase2<Router, Application>)]
pub struct Screen {}

impl Screen {
    pub fn new(router: Weak<RefCell<Router>>, application: Weak<RefCell<Application>>) -> Screen {
        Screen {
            base: ScreenBase2::new(router, application),
        }
    }
}

#[child_of(RouterBase2<Theme, Screen, Application>)]
pub struct Router {}

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
