use std::collections::HashSet;

// I MUST ADMIT THAT I DID NOT EXPECT THIS TO WORK
// AND IT IS PROPABLY VERY BAD WAY TO SOLVE IT  

// already counted points are marked as n - 32 where n is the previous plat type
// because we need to mark each region differently
const MARK: u8 = 32;

const INPUT: &str = include_str!("./input.txt");
// const INPUT: &str = include_str!("./test_input.txt");

pub fn run() {
    let mut grid = parse_input(INPUT);
    let regions = split_to_regions(&mut grid);
    // let g: Vec<Vec<char>> = grid.iter().map(|l| l.iter().map(|n| (*n + 32) as char).collect()).collect();
    // println!("{:?}", g);
    // return;
    let mut res = 0;
    for mut r in regions {
        let perimeter = r.get_perimeter();
        // println!("REGION: {}", r.plant as char);
        // println!("PERIMETER: {}", perimeter);
        // println!("AREA: {}", r.points.len());
        // println!();
        res += perimeter * r.points.len();
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
    let mut edges = HashSet::new();
    let mut points = HashSet::new();

    find_region_helper(grid, &mut points, start, plant, &mut edges);

    Region::new(plant, edges, points)
}

fn find_region_helper(
    grid: &mut [Vec<u8>],
    points: &mut HashSet<Point>,
    point: Point,
    plant: u8,
    edges: &mut HashSet<(Edge, Point)>,
) {
    point.set(grid, plant - MARK); // this point is already counted so we need to mark it
    points.insert(point);
    for i in -1..=1_i32 {
        for j in -1..=1_i32 {
            if i.abs() + j.abs() != 1 {continue}
            let new = point.add(Point::new(i, j));

            if !new.is_inside(grid) {
                insert_edge(edges, j, i, new);
                continue;
            }
            
            let new_plant = new.get(grid);
            if new_plant != plant {
                if new_plant != plant - MARK {
                    insert_edge(edges, j, i, new);
                }
                continue;
            }

            find_region_helper(grid, points, new, plant, edges);
        }
    }
}

fn insert_edge(edges: &mut HashSet<(Edge, Point)>, j: i32, i: i32, point: Point) {
    let edge = if j == 1 {
        Edge::Left
    } else if j == -1 {
        Edge::Right
    } else if i == 1 {
        Edge::Top
    } else {
        Edge::Bottom
    };

    edges.insert((edge, point));
}


fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars().map(|ch| ch as u8).collect())
        .collect()
}


#[derive(Clone, Debug)]
struct Region {
    pub plant: u8, // this is just for debugging
    pub points: HashSet<Point>,
    edges: HashSet<(Edge, Point)>,
}

impl Region {
    pub fn new(plant: u8, edges: HashSet<(Edge, Point)>, points: HashSet<Point>) -> Self {
        Self {
            plant,
            edges,
            points,
        }
    }

    pub fn get_perimeter(&mut self) -> usize {
        dir_perimeter(&mut self.edges)
    }
}

fn dir_perimeter(points: &mut HashSet<(Edge, Point)>) -> usize {
    let mut perimeter = 0;
    while !points.is_empty() {
        let current = *points.iter().next().unwrap();
        // println!("R: {:?}", current.1);
        remove_points_in_dir(current, points, current.0.get_dir());
        remove_points_in_dir(current, points, current.0.get_dir().inv());
        points.remove(&current);
        perimeter += 1;
        // println!();
    }

    perimeter
}

fn remove_points_in_dir(pair: (Edge, Point), points: &mut HashSet<(Edge, Point)>, dir: Point) {
    let mut current = pair;
    current.1 = current.1.add(dir);
    while points.contains(&current) {
        points.remove(&current);
        // println!("R: {:?}", current.1);
        current.1 = current.1.add(dir);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

impl Edge {
    pub fn get_dir(&self) -> Point {
        match *self {
            Edge::Bottom | Edge::Top => Point::new(0, 1),
            Edge::Left | Edge::Right => Point::new(1, 0),
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

    pub fn inv(&self) -> Self {
        Point::new(-self.x, -self.y)
    }

    pub fn add(&self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}