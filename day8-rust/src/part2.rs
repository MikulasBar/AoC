use std::{collections::{HashMap, HashSet}, ops::{Add, AddAssign, Mul, Sub}};

const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let grid = INPUT.lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let height = grid.len();
    let width = grid[0].len();
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    for i in 0..height {
        for j in 0..width {
            let ch = grid[i][j];
            if ch == '.' {
                continue;
            }

            if let Some(v) = antennas.get_mut(&ch) {
                v.push(Point(i as i32, j as i32));
            } else {
                antennas.insert(ch, vec![Point(i as i32, j as i32)]);
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_, v) in antennas.iter() {
        for i in 0..v.len() {
            for j in 0..i {
                let antenna = v[i];
                let dir = (v[i] - v[j]).normal();
                let mut antinode = antenna + dir;
                
                antinodes.insert(antenna);
                
                while is_in_range(antinode, height, width) {
                    antinodes.insert(antinode);
                    antinode += dir;
                }
                
                antinode = antenna - dir;
                while is_in_range(antinode, height, width) {
                    antinodes.insert(antinode);
                    antinode += dir * -1;
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn is_in_range(p: Point, height: usize, width: usize) -> bool {
    if p.0 < 0 || p.1 < 0 {
        return false;
    }

    if p.0 >= height as i32 || p.1 >= width as i32 {
        return false;
    }

    return true;
}

// x = b - a
// res1 = b + x
// res2 = a - x


// res1 = 2b - a
// res2 = 2a - b


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);


impl Point {
    pub fn normal(&self) -> Point {
        let gcd = gcd(self.0.abs(), self.1.abs());
        Point(self.0 / gcd, self.1 / gcd)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}


impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}


fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}