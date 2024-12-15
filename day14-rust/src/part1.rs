const SECONDS: i32 = 100;

const INPUT: &str = include_str!("./input.txt");
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

// const INPUT: &str = include_str!("./test_input.txt");
// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;

pub fn run() {
    let bounds = (WIDTH, HEIGHT);
    let robots = parse_input(INPUT);
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for mut r in robots {
        r.move_times(SECONDS);
        r.wrap_to_bounds(bounds);
        let q = r.quadrant(bounds);
        match q {
            (1, 1) => q1 += 1,
            (1, 2) => q2 += 1,
            (2, 1) => q3 += 1,
            (2, 2) => q4 += 1,
            _ => (),
        }
    }

    let res = q1 * q2 * q3 * q4;
    // println!("{}", q1);
    // println!("{}", q2);
    // println!("{}", q3);
    // println!("{}", q4);

    println!("RES: {}", res);
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

    pub fn quadrant(&self, bounds: Point) -> (u8, u8) {
        let mid_w = bounds.0 / 2;
        let mid_h = bounds.1 / 2;

        let mut h = 0;
        let mut v = 0;
        if self.pos.0 < mid_w {
            h = 1;
        } else if self.pos.0 > mid_w {
            h = 2;
        } else {
            return (0, 0);
        }

        if self.pos.1 < mid_h {
            v = 1;
        } else if self.pos.1 > mid_h {
            v = 2;
        } else {
            return (0, 0);
        }

        (h, v)
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