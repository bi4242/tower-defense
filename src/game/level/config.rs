#[derive(Debug)]
pub struct LevelConfig {
    pub width:  usize,
    pub height: usize,
}

impl LevelConfig {
    pub const TEST: LevelConfig = LevelConfig {
        width:  20,
        height: 20
    };
}
