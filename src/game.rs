use nannou::{Draw, Event, color::rgb, event::{Key, WindowEvent}};

use crate::game::level::Level;

use game_states::*;

mod level;

#[derive(Debug)]
pub enum Game {
    MainMenu (MainMenu),
    InLevel  (InLevel),
    WonLevel (WonLevel),
    LostLevel(LostLevel),
}

mod game_states {
    use super::*;

    #[derive(Debug)]
    pub struct MainMenu;

    impl MainMenu {
        pub fn new() -> Self {
            MainMenu
        }

        pub fn draw(&self, draw: &Draw) {
            draw.background().color(rgb(0.0, 255.0, 0.0));
        }

        pub fn keypress(self, key: Key) -> Game {
            match key {
                Key::Return => {
                    Game::InLevel(InLevel::new())
                },
                _ => Game::MainMenu(self),
            }
        }
    }

    #[derive(Debug)]
    pub struct InLevel {
        level: Level,
    }

    impl InLevel {
        pub(super) fn new() -> Self {
            InLevel {
                level: Level::new(),
            }
        }

        pub fn draw(&self, draw: &Draw) {
            draw.background().color(rgb(10.0, 10.0, 10.0));
        }

        pub fn keypress(self, key: Key) -> Game {
            Game::InLevel(self)
        }
    }

    #[derive(Debug)]
    pub struct WonLevel {
        level: Level,
    }

    impl WonLevel {
        pub fn draw(&self, draw: &Draw) {

        }

        pub fn keypress(self, key: Key) -> Game {
            Game::WonLevel(self)
        }
    }

    #[derive(Debug)]
    pub struct LostLevel {
        level: Level,
    }

    impl LostLevel {
        pub fn draw(&self, draw: &Draw) {

        }

        pub fn keypress(self, key: Key) -> Game {
            Game::LostLevel(self)
        }
    }
}

impl Game {
    pub const DUMMY: Self = Game::MainMenu(MainMenu);

    pub fn new() -> Self {
        Self::MainMenu(MainMenu::new())
    }

    pub fn event(self, event: Event) -> Game {
        match event {
            Event::Update(_) => {
                self.tick()
            },
            Event::WindowEvent { simple: Some(WindowEvent::KeyPressed(key)), .. } => {
                self.keypress(key)
            },
            _ => self,
        }
    }

    pub fn keypress(self, key: Key) -> Game {
        match self {
            Self::MainMenu (main_menu)    => main_menu .keypress(key),
            Self::InLevel  (in_level)      => in_level  .keypress(key),
            Self::WonLevel (won_level)    => won_level .keypress(key),
            Self::LostLevel(lost_level)  => lost_level.keypress(key),
        }
    }

    pub fn tick(self) -> Game {
        self
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
