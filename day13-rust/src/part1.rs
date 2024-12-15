

// THIS IS VERY INEFFICIENT
// FOR EFFICIENT SOLUTION SEE PART2


const INPUT: &str = include_str!("./input.txt");
// const INPUT: &str = include_str!("./test_input.txt");

const A_COST: usize = 3;
const B_COST: usize = 1;

pub fn run() {
    let machines = parse_input(INPUT);
    let mut res = 0;

    for m in machines {
        res += m.calc_min_tokens();
    }

    println!("{:?}", res);
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.split("\r\n\r\n")
        .map(|s| Machine::parse(s))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Machine {
    pub a: Point,
    pub b: Point,
    pub prize: Point,
}

impl Machine {
    pub fn calc_min_tokens(&self) -> usize {
        let mut min = usize::MAX;
        for i in 0..=self.prize.x {
            let v = self.a.scale(i);
            if self.prize.x < v.x || self.prize.y < v.y {
                continue;
            }

            let w = self.prize.sub(v);

            if w.x % self.b.x == 0 && w.y % self.b.y == 0 {
                let j = w.div(self.b);
                if j.x != j.y {
                    continue;
                }

                let cost = A_COST * i + B_COST * j.x;
                min = cost.min(min);
            }
        }

        if min == usize::MAX {
            min = 0;
        }

        min
    }

    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let a = parse_button(lines.next().unwrap());
        let b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        Self { a, b, prize }
    }
}

fn parse_button(input: &str) -> Point {
    let mut iter = input.split(" ");
    let x_str = iter.nth(2).unwrap();
    let y_str = iter.next().unwrap();
    let x = x_str[2..=3].parse().unwrap();
    let y = y_str[2..=3].parse().unwrap();

    Point::new(x, y)
}

fn parse_prize(input: &str) -> Point {
    let mut iter = input.split(" ");
    let x_str = iter.nth(1).unwrap();
    let y_str = iter.next().unwrap();

    let x = x_str[2..x_str.len() - 1].parse().unwrap();
    let y = y_str[2..].parse().unwrap();

    Point::new(x, y)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point{
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }

    pub fn sub(&self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }

    pub fn div(&self, rhs: Point) -> Point {
        Point::new(self.x / rhs.x, self.y / rhs.y)
    }

    pub fn scale(&self, rhs: usize) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}