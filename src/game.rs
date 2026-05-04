use nannou::{App, Draw, Event, color::rgb, event::{Key, WindowEvent}};

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
    use nannou::{App, glam::{Vec2, vec2}};

    use crate::game::level::config::LevelConfig;

    use super::*;

    #[derive(Debug)]
    pub struct MainMenu;

    impl MainMenu {
        pub fn new() -> Self {
            MainMenu
        }

        pub fn draw(&self, app: &App, draw: &Draw) {
            draw.background().color(rgb(255.0, 255.0, 255.0));
            draw.text("main menu").color(rgb(0.0, 0.0, 0.0)).font_size(30).xy(vec2(0.0, 0.0));
        }

        pub fn tick(self) -> Game {
            Game::MainMenu(self)
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
            let config = LevelConfig::TEST;
            InLevel {
                level: Level::new(config),
            }
        }

        pub fn draw(&self, app: &App, draw: &Draw) {
            draw.background().color(rgb(255.0, 255.0, 255.0));
            let window = app.window_rect();
            let render_area = window.pad_top(50.0).pad_bottom(50.0);
            self.level.draw(app, draw, render_area);
            draw.text("in level").color(rgb(0.0, 0.0, 0.0)).font_size(30).xy(vec2(0.0, 0.0));
        }

        pub fn tick(mut self) -> Game {
            self.level.tick();
            Game::InLevel(self)
        }

        pub fn keypress(mut self, key: Key) -> Game {
            match key {
                Key::Space => {
                    self.level.spawn();
                },
                _ => (),
            }
            Game::InLevel(self)
        }
    }

    #[derive(Debug)]
    pub struct WonLevel {
        level: Level,
    }

    impl WonLevel {
        pub fn draw(&self, app: &App, draw: &Draw) {
            draw.background().color(rgb(255.0, 255.0, 255.0));
            draw.text("won level").color(rgb(0.0, 0.0, 0.0)).font_size(30).xy(vec2(0.0, 0.0));
        }

        pub fn tick(self) -> Game {
            Game::WonLevel(self)
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
        pub fn draw(&self, app: &App, draw: &Draw) {
            draw.background().color(rgb(255.0, 255.0, 255.0));
            draw.text("lost level").color(rgb(0.0, 0.0, 0.0)).font_size(30).xy(vec2(0.0, 0.0));
        }

        pub fn tick(self) -> Game {
            Game::LostLevel(self)
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
        match self {
            Self::MainMenu (main_menu)    => main_menu .tick(),
            Self::InLevel  (in_level)      => in_level  .tick(),
            Self::WonLevel (won_level)    => won_level .tick(),
            Self::LostLevel(lost_level)  => lost_level.tick(),
        }
    }

    pub fn draw(&self, app: &App, draw: &Draw) {
        match self {
            Self::MainMenu (main_menu)    => main_menu .draw(app, draw),
            Self::InLevel  (in_level)      => in_level  .draw(app, draw),
            Self::WonLevel (won_level)    => won_level .draw(app, draw),
            Self::LostLevel(lost_level)  => lost_level.draw(app, draw),
        }
    }
}
