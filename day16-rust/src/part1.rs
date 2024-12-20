use std::collections::HashSet;

// FIRST TRY, EZ

const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 141;

pub fn run() {
    let (mut grid, start, end) = parse_input(INPUT);
    let res = find_path(&mut grid, start, end);
    println!("{}", res);
}

fn find_path(grid: &mut Vec<Vec<Node>>, start: Point, end: Point) -> usize {
    let mut closed = HashSet::new();
    let mut open = HashSet::new();
    let mut costs = vec![vec![usize::MAX; SIZE]; SIZE];
    let mut parents = vec![vec![Point::new(0, 0); SIZE]; SIZE];

    open.insert(start);
    start.set(&mut costs, 0);
    start.set(&mut parents, Point::new(0, 0));
    
    loop {
        let point = find_lowest_cost(&open, &costs);
        open.remove(&point);
        closed.insert(point);
        point.set(grid, Node::Closed);

        if point == end {
            break;            
        }

        for i in -1..=1 {
            for j in -1..=1 {
                let new = point.add_dir((i, j));

                if i.abs() + j.abs() != 1 {continue}
                if !can_be_open(new, grid) {continue}
                
                let can_be_optimized = new.get(&parents).get(&costs) > point.get(&costs);
                if new.get(grid) != Node::Open || can_be_optimized {
                    new.set(&mut parents, point);
                    let cost = calc_cost(new, point, point.get(&costs), &parents);
                    new.set(&mut costs, cost);
                    new.set(grid, Node::Open);
                    open.insert(new);
                }
            }
        }
    }

    end.get(&costs)
}

fn calc_cost(point: Point, parent: Point, parent_cost: usize, parents: &Vec<Vec<Point>>) -> usize {
    let grandparent = parent.get(parents);
    let diff = point.diff(parent);
    let parent_diff = parent.diff(grandparent);

    if diff != parent_diff || parent_diff == Point::new(0, 0) {
        parent_cost + 1001
    } else {
        parent_cost + 1
    }
}

fn can_be_open(point: Point, grid: &Vec<Vec<Node>>) -> bool {
    if point.x >= SIZE as i32 || point.x < 0 {return false}
    if point.y >= SIZE as i32 || point.y < 0 {return false}

    match point.get(grid) {
        Node::Wall => false,
        Node::Closed => false,
        _ => true,
    }
}

fn find_lowest_cost(open: &HashSet<Point>, costs: &Vec<Vec<usize>>) -> Point {
    let mut min = usize::MAX;
    let mut point = Point::new(0, 0);

    for p in open {
        let c = p.get(costs);

        if c < min {
            min = c;
            point = *p;
        }
    }

    point
}

fn parse_input(input: &str) -> (Vec<Vec<Node>>, Point, Point) {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let grid = input.lines()
        .enumerate()
        .map(|(i, l)| l.chars()
            .enumerate()
            .map(|(j, ch)| {
                match ch {
                    'S' => {
                        start = Point::new(i as i32, j as i32);
                        Node::Idle
                    },
                    'E' => {
                        end = Point::new(i as i32, j as i32);
                        Node::Idle
                    },
                    _ => Node::from_char(ch),
                }
            })
            .collect()
        )
        .collect();

    (grid, start, end)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Node {
    Idle,
    Open,
    Closed,
    Wall,
}

impl Node {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '.' => Node::Idle,
            '#' => Node::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
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

    pub fn diff(&self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}