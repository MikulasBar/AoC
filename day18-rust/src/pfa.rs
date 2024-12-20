
use std::collections::HashSet;
use crate::point::*;


pub fn find_path(grid: &mut Vec<Vec<Node>>, size: usize, start: Point, end: Point) -> Option<Vec<Point>> {
    let mut closed = HashSet::new();
    let mut open = HashSet::new();
    let mut costs = vec![vec![usize::MAX; size]; size];
    let mut parents = vec![vec![Point::new(0, 0); size]; size];

    open.insert(start);
    start.set(&mut costs, 0);
    start.set(&mut parents, Point::new(0, 0));
    
    while !open.is_empty() {
        // print_grid(grid, size);

        let point = *open.iter().min_by_key(|p| p.get(&costs)).unwrap(); 
        open.remove(&point);
        closed.insert(point);
        point.set(grid, Node::Closed);

        if point == end {
            return Some(reconstruct_path(start, end, &parents));           
        }

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let new = point.add_dir(*dir);

            if !can_be_open(new, grid, size) {continue}
            
            let can_be_optimized = new.get(&parents).get(&costs) > point.get(&costs);
            if new.get(grid) != Node::Open || can_be_optimized {
                new.set(&mut parents, point);
                let cost = calc_cost(new, point.get(&costs), end);
                new.set(&mut costs, cost);
                new.set(grid, Node::Open);
                open.insert(new);
            }
        }
    }

    if end.get(grid) == Node::Open {
        return Some(reconstruct_path(start, end, &parents));           
    } else {
        None
    }
}

fn reconstruct_path(start: Point, end: Point, parents: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut curr = end;
    let mut path = vec![];

    while curr != start {
        path.push(curr);
        curr = curr.get(parents);
    }

    // path.push(start);

    path
}

// possibly a* cost
fn calc_cost(point: Point, parent_cost: usize, end: Point) -> usize {
    let g = parent_cost + 1;
    let h = point.taxicab_dist(end);
    g + h
}

fn can_be_open(point: Point, grid: &Vec<Vec<Node>>, size: usize) -> bool {
    if point.x >= size as i32 || point.x < 0 {return false}
    if point.y >= size as i32 || point.y < 0 {return false}

    match point.get(grid) {
        Node::Wall => false,
        Node::Closed => false,
        _ => true,
    }
}

fn print_grid(grid: &Vec<Vec<Node>>, size: usize) {
    for i in 0..size {
        for j in 0..size {
            print!("{}", grid[i][j].to_char());
        }
        println!()
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Node {
    Idle,
    Open,
    Closed,
    Wall,
}

impl Node {
    pub fn to_char(&self) -> char {
        match self {
            Node::Idle => '.',
            Node::Open => 'o',
            Node::Closed => 'g',
            Node::Wall => '#',
        }
    }
}