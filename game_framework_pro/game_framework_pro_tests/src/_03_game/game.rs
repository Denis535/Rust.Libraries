pub struct Game {
    #[allow(dead_code)]
    base: GameBase,
}

impl Game {
    pub fn new() -> Game {
        Game {
            base: GameBase::new(),
        }
    }
}
