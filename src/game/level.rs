use std::thread::current;

use nannou::{App, Draw, color::rgb, geom::Rect, glam::Vec2, rand::random};

use crate::game::level::{config::LevelConfig, enemy::Enemy, projectile::Projectile, tower::Tower};

pub mod config;
mod enemy;
mod tower;
mod projectile;

#[derive(Debug)]
pub struct Level {
    tower_grid: TowerGrid,
    enemy_array:      Vec<Enemy>,
    projectile_array: Vec<Projectile>,
}

impl Level {
    pub fn new(config: LevelConfig) -> Self {
        Self {
            tower_grid: TowerGrid::new(config),
            enemy_array:      Vec::new(),
            projectile_array: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.enemy_array
            .iter_mut()
            .for_each(|enemy| enemy.tick(&self.tower_grid));
    }

    pub fn spawn(&mut self) {
        self.enemy_array.push(Enemy::new((15, 15), &self.tower_grid));
    }

    pub fn draw(&self, app: &App, draw: &Draw, render_area: Rect) {
        self.draw_tower_grid(app, draw, render_area);

        self.enemy_array
            .iter()
            .for_each(|enemy| enemy.draw(app, draw, render_area, &self.tower_grid));
    }

    fn draw_tower_grid(&self, app: &App, draw: &Draw, render_area: Rect) {
        for i in 0..self.tower_grid.height {
            for j in 0..self.tower_grid.width {
                let rect_width  = render_area.w() / self.tower_grid.width  as f32;
                let rect_height = render_area.h() / self.tower_grid.height as f32;

                let x = render_area.x() + rect_width  * i as f32 - render_area.w() / 2.0;
                let y = render_area.y() + rect_height * j as f32 - render_area.h() / 2.0;

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
    grid:        Vec<Vec<Option<Tower>>>,
    width:       usize,
    height:      usize,
    goal:        GridPoint,
    baked_paths: Vec<Vec<Option<GridPoint>>>,
}

pub type GridPoint = (usize, usize);
impl TowerGrid {
    pub fn new(config: LevelConfig) -> Self {
        let width = config.width;
        let height = config.height;

        let grid = vec![vec![None; width]; height];
        let baked_paths = Self::bake_paths(&grid, width, height, config.goal);

        TowerGrid {
            grid,
            width,
            height,
            goal: config.goal,
            baked_paths,
        }
    }

    pub fn coords(&self, p: GridPoint) -> Vec2 {
        Vec2::new(
            p.0 as f32 / self.width  as f32,
            p.1 as f32 / self.height as f32,
        )
    }

    fn neighbors(
        grid:   &Vec<Vec<Option<Tower>>>,
        width:  usize,
        height: usize,
        point:   GridPoint,
    ) -> Vec<GridPoint> {
        let mut neighbors = Vec::new();
        let (x, y) = point;
        
        if x != 0 {
            neighbors.push((x - 1, y));
        }
        if y != 0 {
            neighbors.push((x, y - 1));
        }
        if x != width  - 1 {
            neighbors.push((x + 1, y));
        }
        if y != height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
            .iter()
            .filter(|point| grid[point.0][point.1].is_none())
            .cloned()
            .collect()
    }

    fn bake_paths(
        grid:   &Vec<Vec<Option<Tower>>>,
        width:  usize,
        height: usize,
        goal:   GridPoint,
    ) -> Vec<Vec<Option<GridPoint>>> {
        let mut baked_paths = vec![vec![None; width]; height];

        // Depth-first search from goal outwards.
        let mut queue = vec![goal];
        let mut i = 0;
        loop {
            let Some(&point) = queue.get(i) else {
                break;
            };
            let neighbors = Self::neighbors(grid, width, height, point);
            for neighbor in neighbors {
                if baked_paths[neighbor.0][neighbor.1].is_some() {
                    continue;
                }
                baked_paths[neighbor.0][neighbor.1] = Some(point);
                queue.push(neighbor);
            }
            i += 1;
        }
        // Avoid infinite loop when tracing path to goal.
        baked_paths[goal.0][goal.1] = None;

        baked_paths
    }

    pub fn get_path(&self, position: GridPoint) -> Vec<GridPoint> {
        let mut path = vec![position];
        let mut current_point = position;

        while let Some(next_point) = self.baked_paths[current_point.0][current_point.1] {
            path.push(next_point);
            current_point = next_point;
        }
        path
    }
}



