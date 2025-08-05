pub struct ThemeBase2<TRouter, TApplication> {
    base: ThemeBase,
    router: Weak<RefCell<TRouter>>,
    application: Weak<RefCell<TApplication>>,
}

impl<TRouter, TApplication> ThemeBase2<TRouter, TApplication> {
    pub fn new(router: Weak<RefCell<TRouter>>, application: Weak<RefCell<TApplication>>) -> ThemeBase2<TRouter, TApplication> {
        ThemeBase2 {
            base: ThemeBase::new(),
            router,
            application,
        }
    }
    pub fn base(&self) -> &ThemeBase {
        &self.base
    }
    pub fn base_mut(&mut self) -> &mut ThemeBase {
        &mut self.base
    }
    pub fn router(&self) -> Weak<RefCell<TRouter>> {
        self.router.clone()
    }
    pub fn application(&self) -> Weak<RefCell<TApplication>> {
        self.application.clone()
    }
}

pub struct ScreenBase2<TRouter, TApplication> {
    base: ScreenBase,
    router: Weak<RefCell<TRouter>>,
    application: Weak<RefCell<TApplication>>,
}

impl<TRouter, TApplication> ScreenBase2<TRouter, TApplication> {
    pub fn new(router: Weak<RefCell<TRouter>>, application: Weak<RefCell<TApplication>>) -> ScreenBase2<TRouter, TApplication> {
        ScreenBase2 {
            base: ScreenBase::new(),
            router,
            application,
        }
    }
    pub fn base(&self) -> &ScreenBase {
        &self.base
    }
    pub fn base_mut(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    pub fn router(&self) -> Weak<RefCell<TRouter>> {
        self.router.clone()
    }
    pub fn application(&self) -> Weak<RefCell<TApplication>> {
        self.application.clone()
    }
}

pub struct RouterBase2<TTheme, TScreen, TApplication> {
    base: RouterBase,
    theme: Option<Weak<RefCell<TTheme>>>,
    screen: Option<Weak<RefCell<TScreen>>>,
    application: Weak<RefCell<TApplication>>,
}

impl<TTheme, TScreen, TApplication> RouterBase2<TTheme, TScreen, TApplication> {
    pub fn new(application: Weak<RefCell<TApplication>>) -> RouterBase2<TTheme, TScreen, TApplication> {
        RouterBase2 {
            base: RouterBase::new(),
            theme: Option::None,
            screen: Option::None,
            application,
        }
    }
    pub fn base(&self) -> &RouterBase {
        &self.base
    }
    pub fn base_mut(&mut self) -> &mut RouterBase {
        &mut self.base
    }
    pub fn theme(&self) -> Weak<RefCell<TTheme>> {
        self.theme.as_ref().unwrap().clone()
    }
    pub fn set_theme(&mut self, theme: Weak<RefCell<TTheme>>) {
        self.theme = Option::Some(theme);
    }
    pub fn screen(&self) -> Weak<RefCell<TScreen>> {
        self.screen.as_ref().unwrap().clone()
    }
    pub fn set_screen(&mut self, screen: Weak<RefCell<TScreen>>) {
        self.screen = Option::Some(screen);
    }
    pub fn application(&self) -> Weak<RefCell<TApplication>> {
        self.application.clone()
    }
}
