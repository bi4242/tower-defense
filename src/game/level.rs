use nannou::{App, Draw, geom::Rect};

use crate::game::level::config::LevelConfig;

pub mod config;

#[derive(Debug)]
pub struct Level {
    tower_grid: TowerGrid,
    enemy_array:      Vec<Enemy>,
    projectile_array: Vec<Projectile>,
    render_area: Rect,
}

impl Level {
    pub fn new(config: LevelConfig, render_area: Rect) -> Self {
        Self {
            tower_grid: TowerGrid::new(config.width, config.height),
            enemy_array:      Vec::new(),
            projectile_array: Vec::new(),
            render_area,
        }
    }

    pub fn draw(&self, app: &App, draw: &Draw) {
        self.draw_tower_grid(app, draw);
    }

    pub fn draw_tower_grid(&self, app: &App, draw: &Draw) {
        for i in 0..self.tower_grid.height {
            for j in 0..self.tower_grid.width {
                let rect_width  = self.render_area.w() / self.tower_grid.width  as f32;
                let rect_height = self.render_area.h() / self.tower_grid.height as f32;
                let x = rect_width  * i as f32 - self.render_area.x() / 2.0;
                let y = rect_height * j as f32 - self.render_area.y() / 2.0;
                let rect = Rect::from_corner_points([x, y], [x + rect_width, y + rect_height]);
                draw.rect
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
        }
    }
}

#[derive(Debug)]
pub struct Tower();

#[derive(Debug)]
pub struct Enemy();

#[derive(Debug)]
pub struct Projectile();
