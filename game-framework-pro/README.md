# Overview

The framework that allows you to design high-quality architecture for your game project.

# Reference

The framework provides the following:

- Program
- UI
    - Theme
    - Screen
    - Router
- App
    - Application
- Domain (Business)
    - Game
    - Player
    - Entity

# Example

```
#[child_of(ProgramBase2<Theme, Screen, Router, Application>)]
pub struct Program {}

impl Program {
    pub fn new() -> Program {
        let application = Rc::new(RefCell::new(Application::new()));
        let router = Rc::new(RefCell::new(Router::new(Rc::downgrade(&application))));
        let screen = Rc::new(RefCell::new(Screen::new(Rc::downgrade(&router), Rc::downgrade(&application))));
        let theme = Rc::new(RefCell::new(Theme::new(Rc::downgrade(&router), Rc::downgrade(&application))));
        router.borrow_mut().init(Rc::downgrade(&theme), Rc::downgrade(&screen));
        Program {
            base: ProgramBase2::new(theme, screen, router, application)
        }
    }
}

// UI
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

// App
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

// Domain
#[child_of(GameBase)]
pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {
            base: GameBase::new(),
        }
    }
}
```

# Links

- https://github.com/Denis535/Rust.Libraries/tree/main/game-framework-pro
- https://crates.io/crates/game-framework-pro
