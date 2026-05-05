use nannou::{App, Draw, color::rgb, geom::Rect, glam::Vec2};

use crate::game::level::{TowerGrid, GridPoint};

#[derive(Debug)]
pub struct Enemy {
    path: Vec<GridPoint>,
    segment_distance: f32,
}

impl Enemy {
    pub fn new(position: (usize, usize), tower_grid: &TowerGrid) -> Self {
        let path = tower_grid.get_path(position);
        Self {
            path,
            segment_distance: 0.0,
        }
    }

    pub fn position(&self, tower_grid: &TowerGrid) -> Vec2 {
        if self.path.len() == 1 {
            // Should be removed by level soon.
            return tower_grid.coords(self.path[0]);
        }
        let prev_point = tower_grid.coords(self.path[self.path.len() - 1]);
        let next_point = tower_grid.coords(self.path[self.path.len() - 2]);
        Vec2::new(
            prev_point.x + (next_point.x - prev_point.x) * self.segment_distance,
            prev_point.y + (next_point.y - prev_point.y) * self.segment_distance,
        )
    }

    pub fn tick(&mut self, tower_grid: &TowerGrid) {
        if self.path.len() == 1 {
            return;
        }
        self.segment_distance += 0.1;
        if self.segment_distance >= 1.0 {
            self.path.pop();
            self.segment_distance = 0.1;
        }
    }

    pub fn draw(&self, app: &App, draw: &Draw, render_area: Rect, tower_grid: &TowerGrid) {
        let x = render_area.left()   +
            render_area.w() * self.position(tower_grid).x;
        let y = render_area.bottom() +
            render_area.h() * self.position(tower_grid).y;
        draw.ellipse()
            .x_y(x, y)
            .radius(10.0)
            .color(rgb(0.0, 0.0, 255.0));
    }
}
