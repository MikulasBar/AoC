use crate::point::*;
use crate::pfa::*;
const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 71;

pub fn run() {
    let bytes = parse_input(INPUT);
    let mut grid = vec![vec![Node::Idle; SIZE]; SIZE];
    
    for i in 0..1024 {
        bytes[i].set(&mut grid, Node::Wall);
    }

    let start = Point::new(0, 0);
    let end = Point::new(SIZE as i32 - 1, SIZE as i32 - 1);
    let path = find_path(&mut grid, SIZE, start, end).unwrap();

    println!("RES: {}", path.len());
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
