

const INPUT: &str = include_str!("./input.txt");
// const INPUT: &str = include_str!("./test_input.txt");

const A_COST: usize = 3;
const B_COST: usize = 1;
const OFF: i64 = 10000000000000;
// const OFF: i64 = 0;

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
    a1: i64,
    a2: i64,
    b1: i64,
    b2: i64,
    p1: i64,
    p2: i64,
}

impl Machine {
    fn calc_min_tokens(&self) -> usize {
        let (x, y) = self.solve_system();
        if x <= 0 || y <= 0 {
            return 0;
        }

        A_COST * x as usize + B_COST * y as usize
    }

    fn solve_system(&self) -> (i64, i64) {
        let y = solve_y(self.a1, self.a2, self.b1, self.b2, self.p1, self.p2);
        let x = solve_x(self.a1, self.b1, self.p1, y);

        (x, y)
    }

    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let (a1, a2) = parse_button(lines.next().unwrap());
        let (b1, b2) = parse_button(lines.next().unwrap());
        let (p1, p2) = parse_prize(lines.next().unwrap());

        Self { a1, a2, b1, b2, p1, p2 }
    }
}

fn solve_y(a1: i64, a2: i64, b1: i64, b2: i64, p1: i64, p2: i64) -> i64 {
    let n = a1 * p2 - a2 * p1;
    let m = a1 * b2 - a2 * b1;
    if n % m == 0 {
        n / m
    } else {
        0
    }
}

fn solve_x(a1: i64, b1: i64, p1: i64, y: i64) -> i64 {
    let n = p1 - b1 * y;
    let m = a1;
    if n % m == 0 {
        n / m
    } else {
        0
    }
}

fn parse_button(input: &str) -> (i64, i64) {
    let mut iter = input.split(" ");
    let x_str = iter.nth(2).unwrap();
    let y_str = iter.next().unwrap();
    let x = x_str[2..=3].parse().unwrap();
    let y = y_str[2..=3].parse().unwrap();

    (x, y)
}

fn parse_prize(input: &str) -> (i64, i64) {
    let mut iter = input.split(" ");
    let x_str = iter.nth(1).unwrap();
    let y_str = iter.next().unwrap();

    let x: i64 = x_str[2..x_str.len() - 1].parse().unwrap();
    let y: i64 = y_str[2..].parse().unwrap();

    (x + OFF, y + OFF)
}
