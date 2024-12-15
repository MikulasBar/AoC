const INPUT: &str = include_str!("./input.txt");
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn run() {
    let bounds = (WIDTH, HEIGHT);
    let mut robots = parse_input(INPUT);
    
    for i in 0..10_000 {
        for r in robots.iter_mut() {
            r.move_times(1);
            r.wrap_to_bounds(bounds);
        }

        println!("ITERATION: {}", i);
        let grid = generate_grid(&robots, bounds);
        println!("{}", grid);
    }
}

fn generate_grid(robots: &Vec<Robot>, bounds: Point) -> String {
    let mut grid = vec![vec![0; bounds.1 as usize]; bounds.0 as usize];

    for r in robots {
        grid[r.pos.0 as usize][r.pos.1 as usize] += 1;
    }

    let mut string = String::new();

    for i in 0..bounds.0 as usize {
        for j in 0..bounds.1 as usize {
            if grid[i][j] == 0 {
                string.push(' ');
            } else {
                string.push('X');
            }
        }
        string.push('\n');
    }

    string
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines()
        .map(|l| Robot::parse(l))
        .collect()
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Robot {
    pos: Point,
    vel: Point
}

impl Robot {
    pub fn new(pos: Point, vel: Point) -> Self {
        Self { pos, vel }
    }

    pub fn move_times(&mut self, times: i32) {
        let offset = (self.vel.0 * times, self.vel.1 * times);
        self.pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);
    }

    pub fn wrap_to_bounds(&mut self, bounds: Point) {
        self.pos.0 = ((self.pos.0 % bounds.0) + bounds.0) % bounds.0;
        self.pos.1 = ((self.pos.1 % bounds.1) + bounds.1) % bounds.1;
    }

    pub fn parse(input: &str) -> Self {
        let mut iter = input.split(" ")
            .map(|s| parse_point(&s[2..]));

        Self::new(iter.next().unwrap(), iter.next().unwrap())
    }
}

type Point = (i32, i32);

fn parse_point(input: &str) -> Point {
    let mut iter = input.split(",")
        .map(|s| s.parse::<i32>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}