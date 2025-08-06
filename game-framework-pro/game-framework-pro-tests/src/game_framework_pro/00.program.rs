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
