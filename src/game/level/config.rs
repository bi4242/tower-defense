use crate::game::level::{Cell, GridPoint};

#[derive(Clone, Copy, Debug)]
pub struct LevelConfig {
    pub width:  usize,
    pub height: usize,
    grid:       &'static [&'static [char]],
}

impl LevelConfig {
    pub const LEVEL: LevelConfig = LevelConfig {
        width:  11,
        height: 11,
        grid: &[
            &[' ', ' ', ' ', ' ', ' ', 'O', ' ', ' ', ' ', ' ', ' '], // 0
            &[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 1
            &[' ', ' ', 'X', '#', '#', ' ', '#', '#', 'X', ' ', ' '], // 2
            &[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 3
            &[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 4
            &[' ', ' ', ' ', ' ', ' ', '*', ' ', ' ', ' ', ' ', ' '], // 5
            &[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 6
            &[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 7
            &[' ', ' ', 'X', '#', '#', ' ', '#', '#', 'X', ' ', ' '], // 8
            &[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 9
            &[' ', ' ', ' ', ' ', ' ', 'O', ' ', ' ', ' ', ' ', ' '], // 10
            // 0    1    2    3    4    5    6    7    8    9    10
        ],
    };

    pub fn locations(&self, char: char) -> Vec<GridPoint> {
        let mut locations = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.grid[y][x] == char {
                    locations.push((x, y));
                }
            }
        }
        locations
    }

    pub fn spawns(&self) -> Vec<GridPoint> {
        let spawns = self.locations('O');
        if spawns.len() == 0 {
            panic!("must be at least 1 spawn");
        }
        spawns
    }

    pub fn goal(&self) -> GridPoint {
        let goals = self.locations('*');
        if goals.len() != 1 {
            panic!("must be exactly 1 goal in a level")
        }
        goals[0]
    }

    pub fn cell(&self, x: usize, y: usize) -> Cell {
        match self.grid[y][x] {
            ' ' => Cell::Empty,
            'O' => Cell::Spawn,
            '*' => Cell::Goal,
            '#' => Cell::Blocked,
            'X' => Cell::Unplacable,
            _   => panic!("illegal cell"),
        }
    }
}
