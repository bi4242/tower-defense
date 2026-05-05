use nannou::{App, Draw, color::rgb, geom::Rect, glam::Vec2};

use crate::game::level::{config::LevelConfig, enemy::Enemy, projectile::Projectile, tower::Tower};

pub mod config;
mod enemy;
mod tower;
mod projectile;

#[derive(Debug)]
pub struct Level {
    tower_grid:       TowerGrid,
    enemy_array:      Vec<Enemy>,
    projectile_array: Vec<Projectile>,
    enemies_spawned:  usize,
    config:           LevelConfig,
}

impl Level {
    pub fn new(config: LevelConfig) -> Self {
        Self {
            tower_grid: TowerGrid::new(config),
            enemy_array:      Vec::new(),
            projectile_array: Vec::new(),
            enemies_spawned:  0,
            config,
        }
    }

    pub fn tick(&mut self) {
        self.enemy_array
            .iter_mut()
            .for_each(|enemy| enemy.tick(&self.tower_grid));
    }

    pub fn spawn(&mut self) {
        let spawns = self.config.spawns();
        let spawn = spawns[self.enemies_spawned % spawns.len()];
        self.enemy_array.push(Enemy::new(spawn, &self.tower_grid));
        self.enemies_spawned += 1;
    }

    pub fn draw(&self, app: &App, draw: &Draw, render_area: Rect) {
        self.draw_tower_grid(app, draw, render_area);

        self.enemy_array
            .iter()
            .for_each(|enemy| enemy.draw(app, draw, render_area, &self.tower_grid));
    }

    fn draw_tower_grid(&self, app: &App, draw: &Draw, render_area: Rect) {
        self.tower_grid.draw(app, draw, render_area);
    }
}

#[derive(Debug)]
pub enum Cell {
    Empty,
    Tower(Tower),
    Spawn,
    Goal,
    Blocked,
    Unplacable,
}

impl Cell {
    pub fn is_movable(&self) -> bool {
        matches!(self, Self::Empty | Self::Spawn | Self::Goal | Self::Unplacable)
    }

    pub fn is_placable(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug)]
pub struct TowerGrid {
    grid:        Grid,
    width:       usize,
    height:      usize,
    goal:        GridPoint,
    baked_paths: Vec<Vec<Option<GridPoint>>>,
}

pub type GridPoint = (usize, usize);
type Grid = Vec<Vec<Cell>>;
impl TowerGrid {
    pub fn new(config: LevelConfig) -> Self {
        let width = config.width;
        let height = config.height;

        // List of rows instead of list of rows for [x][y]-indexing.
        let grid = (0..width).map(|x| {
            (0..height).map(|y| config.cell(x, y))
                .collect()
        }).collect();
        let baked_paths = Self::bake_paths(&grid, width, height, config.goal());

        TowerGrid {
            grid,
            width,
            height,
            goal: config.goal(),
            baked_paths,
        }
    }

    pub fn coords(&self, p: GridPoint) -> Vec2 {
        Vec2::new(
            (p.0 as f32 + 0.5) / self.width  as f32,
            (p.1 as f32 + 0.5) / self.height as f32,
        )
    }

    fn neighbors(
        grid:   &Grid,
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
            .filter(|point| grid[point.0][point.1].is_movable())
            .cloned()
            .collect()
    }

    fn bake_paths(
        grid:   &Grid,
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
        // Reverse path such that the enemy can pop segments off the end.
        path.reverse();
        path
    }

    fn draw(&self, app: &App, draw: &Draw, render_area: Rect) {
        for i in 0..self.height {
            for j in 0..self.width {
                let rect_width  = render_area.w() / self.width  as f32;
                let rect_height = render_area.h() / self.height as f32;

                let x = render_area.x() + rect_width  * i as f32 - render_area.w() / 2.0;
                let y = render_area.y() + rect_height * j as f32 - render_area.h() / 2.0;

                let rect = Rect::from_corner_points([x, y], [x + rect_width, y + rect_height]);

                let color = match self.grid[i][j] {
                    Cell::Empty if (i + j).is_multiple_of(2) => rgb(0.0,   1.0,   0.0),
                    Cell::Empty                              => rgb(0.0,   0.5,   0.0),
                    Cell::Tower(_)                           => rgb(1.0,   0.0,   0.0),
                    Cell::Spawn                              => rgb(0.5,   0.0,   0.5),
                    Cell::Goal                               => rgb(0.0,   0.0,   0.0),
                    Cell::Blocked                            => rgb(0.25,  0.25,  0.25),
                    Cell::Unplacable                         => rgb(0.8,   0.5,   0.25),
                };

                draw
                    .rect()
                    .x_y(rect.x(), rect.y()).w_h(rect.w(), rect.h())
                    .color(color);
            }
        }
    }
}



