use crate::point::*;
use crate::pfa::*;
const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 71;

pub fn run() {
    let bytes = parse_input(INPUT);
    let mut grid = vec![vec![Node::Idle; SIZE]; SIZE];
    let start = Point::new(0, 0);
    let end = Point::new(SIZE as i32 - 1, SIZE as i32 - 1);

    let mut byte_number = 1024;
    let mut is_crossable = true;

    while is_crossable {
        byte_number += 1;
        clear_grid(&mut grid);
        add_bytes(byte_number, &bytes, &mut grid);
        is_crossable = find_path(&mut grid, SIZE, start, end)
            .map_or(false, |_| true);
    }
    
    println!("RES: {:?}", bytes[byte_number-1]);
}

fn add_bytes(number: usize, bytes: &Vec<Point>, grid: &mut Vec<Vec<Node>>) {
    for i in 0..number {
        bytes[i].set(grid, Node::Wall);
    }
}

fn clear_grid(grid: &mut Vec<Vec<Node>>) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            grid[i][j] = Node::Idle;
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines()
        .map(|l| {
            let mut iter = l.split(",");
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            Point::new(x, y)
        })
        .collect()
}
