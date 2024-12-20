
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get<T: Copy>(&self, grid: &Vec<Vec<T>>) -> T {
        grid[self.x as usize][self.y as usize]
    }

    pub fn set<T>(&self, grid: &mut Vec<Vec<T>>, val: T) {
        grid[self.x as usize][self.y as usize] = val;
    }

    pub fn add_dir(&self, dir: (i8, i8)) -> Self {
        Point::new(self.x + dir.0 as i32, self.y + dir.1 as i32)
    }

    pub fn taxicab_dist(&self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}