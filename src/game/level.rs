use nannou::Draw;

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

    pub fn draw(&self, draw: &Draw) {
        
    }
}

#[derive(Debug)]
pub struct TowerGrid {
    grid: Vec<Vec<Option<Tower>>>,
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
