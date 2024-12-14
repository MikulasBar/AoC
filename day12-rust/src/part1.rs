use std::collections::HashSet;

// already counted points are marked as n - 32 where n is the previous plat type
// because we need to mark each region differently
const MARK: u8 = 32;

const INPUT: &str = include_str!("./input.txt");
// const INPUT: &str = include_str!("./test_input.txt");

pub fn run() {
    let mut grid = parse_input(INPUT);
    let regions = split_to_regions(&mut grid);

    let mut res = 0;
    for r in regions {
        // println!("REGION: {}", r.plant as char);
        // println!("PERIMETER: {}", r.perimeter);
        // println!("AREA: {}", r.points.len());
        // println!();
        res += r.perimeter * r.points.len();
    }

    println!("RESULT: {}", res);
}


fn split_to_regions(grid: &mut [Vec<u8>]) -> Vec<Region> {
    let mut regions = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] < 59 {continue} // 59 is the highest mark number

            let reg = find_region(grid, Point::new(i as i32, j as i32));
            regions.push(reg);
        }
    }

    regions
}

fn find_region(grid: &mut [Vec<u8>], start: Point) -> Region {
    let plant = start.get(grid);
    let mut perimeter = 0;
    let mut points = HashSet::new();

    find_region_helper(grid, &mut points, start, plant, &mut perimeter);

    Region::new(plant, perimeter, points)
}

fn find_region_helper(
    grid: &mut [Vec<u8>],
    points: &mut HashSet<Point>,
    point: Point,
    plant: u8,
    perimeter: &mut usize
) {
    point.set(grid, plant - MARK); // this point is already counted so we need to mark it
    points.insert(point);
    for i in -1..=1_i32 {
        for j in -1..=1_i32 {
            if i.abs() + j.abs() != 1 {continue}
            let new = Point::new(point.x + i, point.y + j);

            if !new.is_inside(grid) {
                *perimeter += 1;
                continue;
            }
            
            let new_plant = new.get(grid);
            if new_plant != plant {
                if new_plant != plant - MARK {
                    *perimeter += 1;
                }
                continue;
            }

            find_region_helper(grid, points, new, plant, perimeter);
        }
    }
}


fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars().map(|ch| ch as u8).collect())
        .collect()
}


#[derive(Clone, Debug)]
struct Region {
    pub plant: u8, // this is just for debugging
    pub perimeter: usize,
    pub points: HashSet<Point>
}

impl Region {
    pub fn new(plant: u8, perimeter: usize, points: HashSet<Point>) -> Self {
        Self {
            plant,
            perimeter,
            points,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new<X, Y>(x: X, y: Y) -> Self
    where 
        X: Into<i32>,
        Y: Into<i32>
    {
        Self {
            x: x.into(),
            y: y.into()
        }
    }


    pub fn get<T: Copy>(&self, grid: &[Vec<T>]) -> T {
        grid[self.x as usize][self.y as usize]
    }

    pub fn set<T>(&self, grid: &mut [Vec<T>], val: T) {
        grid[self.x as usize][self.y as usize] = val;
    }

    pub fn is_inside<T>(&self, grid: &mut [Vec<T>]) -> bool {
        if self.x >= grid.len() as i32 || self.x < 0
        || self.y >= grid[0].len() as i32 || self.y < 0 {
            false
        } else {
            true
        }
    }
}