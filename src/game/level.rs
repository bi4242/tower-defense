#[derive(Debug)]
pub struct Level {
    tower_grid: TowerGrid,
    enemy_array:      Vec<Enemy>,
    projectile_array: Vec<Projectile>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            tower_grid: TowerGrid::new(),
            enemy_array:      Vec::new(),
            projectile_array: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct TowerGrid();

impl TowerGrid {
    pub fn new() -> Self {
        TowerGrid()
    }
}

#[derive(Debug)]
pub struct Enemy();

#[derive(Debug)]
pub struct Projectile();
