use crate::game::level::GridPoint;

#[derive(Debug)]
pub struct LevelConfig {
    pub width:  usize,
    pub height: usize,
    pub goal:   GridPoint,
}

impl LevelConfig {
    pub const TEST: LevelConfig = LevelConfig {
        width:  20,
        height: 20,
        goal:   (10, 5),
    };
}
