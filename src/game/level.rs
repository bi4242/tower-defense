use nannou::{App, Draw, color::rgb, geom::Rect};

use crate::game::level::config::LevelConfig;

pub mod config;

#[derive(Debug)]
pub struct Level {
    tower_grid: TowerGrid,
    enemy_array:      Vec<Enemy>,
    projectile_array: Vec<Projectile>,
}

impl Level {
    pub fn new(config: LevelConfig) -> Self {
        Self {
            tower_grid: TowerGrid::new(config.width, config.height),
            enemy_array:      Vec::new(),
            projectile_array: Vec::new(),
        }
    }

    pub fn draw(&self, app: &App, draw: &Draw, render_area: Rect) {
        self.draw_tower_grid(app, draw, render_area);
    }

    pub fn draw_tower_grid(&self, app: &App, draw: &Draw, render_area: Rect) {
        for i in 0..self.tower_grid.height {
            for j in 0..self.tower_grid.width {
                let rect_width  = render_area.w() / self.tower_grid.width  as f32;
                let rect_height = render_area.h() / self.tower_grid.height as f32;
                let x = rect_width  * i as f32 - render_area.w() / 2.0;
                let y = rect_height * j as f32 - render_area.h() / 2.0;
                let rect = Rect::from_corner_points([x, y], [x + rect_width, y + rect_height]);
                let color = if (i + j).is_multiple_of(2) {
                    rgb(255.0, 0.0, 0.0)
                } else {
                    rgb(0.0, 255.0, 0.0)
                };
                draw
                    .rect()
                    .x_y(rect.x(), rect.y()).w_h(rect.w(), rect.h())
                    .color(color);
            }
        }
    }
}

#[derive(Debug)]
pub struct TowerGrid {
    grid: Vec<Vec<Option<Tower>>>,
    width: usize,
    height: usize,
}

impl TowerGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| None)
                    .collect()
            })
            .collect();

        TowerGrid {
            grid,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub struct Tower();

#[derive(Debug)]
pub struct Enemy();

#[derive(Debug)]
pub struct Projectile();
