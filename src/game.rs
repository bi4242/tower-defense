use nannou::{Draw, color::rgb};

use crate::game::level::Level;

mod level;

#[derive(Debug)]
pub enum Game {
    MainMenu (MainMenu),
    InLevel  (InLevel),
    WonLevel (WonLevel),
    LostLevel(LostLevel),
}

#[derive(Debug)]
pub struct MainMenu;

impl MainMenu {
    pub fn new() -> Self {
        MainMenu
    }

    pub fn draw(&self, draw: &Draw) {
        draw.background().color(rgb(0.0, 255.0, 0.0));
    }
}

#[derive(Debug)]
pub struct InLevel {
    level: Level,
}

impl InLevel {
    pub fn draw(&self, draw: &Draw) {

    }
}

#[derive(Debug)]
pub struct WonLevel {
    level: Level,
}

impl WonLevel {
    pub fn draw(&self, draw: &Draw) {

    }
}

#[derive(Debug)]
pub struct LostLevel {
    level: Level,
}

impl LostLevel {
    pub fn draw(&self, draw: &Draw) {

    }
}

impl Game {
    pub fn new() -> Self {
        Self::MainMenu(MainMenu::new())
    }

    pub fn tick() {

    }

    pub fn draw(&self, draw: &Draw) {
        match self {
            Self::MainMenu (main_menu)    => main_menu .draw(draw),
            Self::InLevel  (in_level)      => in_level  .draw(draw),
            Self::WonLevel (won_level)    => won_level .draw(draw),
            Self::LostLevel(lost_level)  => lost_level.draw(draw),
        }
    }
}
