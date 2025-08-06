#[child_of(GameBase)]
pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {
            base: GameBase::new(),
        }
    }
}
