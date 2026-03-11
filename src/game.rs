use nannou::Draw;

use crate::game::level::Level;

mod level;

#[derive(Debug)]
pub enum Game {
    MainMenu,
    InLevel {
        level: Level,
    },
    WonLevel {
        level: Level,
    },
    LostLevel {
        level: Level,
    },
}

impl Game {
    pub fn new() -> Self {
        Self::MainMenu
    }

    pub fn tick() {

    }

    pub fn draw(draw: Draw) {
        draw.background().color(nannou::color::AQUAMARINE);
    }
}
