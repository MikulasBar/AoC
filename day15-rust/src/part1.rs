
const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 50;

pub fn run() {
    let (mut grid, moves, mut robot) = parse_input(INPUT);

    // println!("GRID:\n{:?}", grid);
    // println!("MOVES:\n{:?}", moves);
    // println!("ROBOT: {:?}", robot);

    for dir in moves.into_iter().map(char_to_dir) {
        move_robot(&mut robot, &mut grid, dir);
    }

    let mut res = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if grid[i][j] == Node::Box {
                res += 100 * i + j;
            }
        }
    }

    println!("RES: {}", res);
}




#[derive(Clone, Copy, PartialEq, Debug)]
enum Node {
    Robot,
    Empty,
    Wall,
    Box,
}

impl Node {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '.' => Node::Empty,
            '#' => Node::Wall,
            'O' => Node::Box,
            '@' => Node::Robot,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
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
}

fn move_robot(robot: &mut Point, grid: &mut Vec<Vec<Node>>, dir: (i8, i8)) {
    let new_pos = robot.add_dir(dir);
    let new_node = new_pos.get(grid);

    match new_node {
        Node::Wall => return,
        Node::Box => move_box_row(robot, dir, grid),
        Node::Empty => {
            robot.set(grid, Node::Empty);
            *robot = new_pos;
            robot.set(grid, Node::Robot);
        },

        _ => unreachable!(),
    }
}

fn move_box_row(robot: &mut Point, dir: (i8, i8), grid: &mut Vec<Vec<Node>>) {
    let mut curr = robot.add_dir(dir);

    while curr.get(grid) == Node::Box {
        curr = curr.add_dir(dir);
    }

    match curr.get(grid) {
        Node::Empty => {
            curr.set(grid, Node::Box);
            robot.set(grid, Node::Empty);
            *robot = robot.add_dir(dir);
            robot.set(grid, Node::Robot);
        },
        Node::Wall => (),
        _ => unreachable!(),
    }
}

fn char_to_dir(ch: char) -> (i8, i8) {
    match ch {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Node>>, Vec<char>, Point) {
    let mut iter = input.split("\r\n\r\n");
    let (grid, robot) = parse_grid(iter.next().unwrap());
    let moves = parse_moves(iter.next().unwrap());

    (grid, moves, robot)
}

fn parse_grid(input: &str) -> (Vec<Vec<Node>>, Point) {
    let mut robot = Point::new(0, 0);

    let grid: Vec<Vec<Node>> = input.lines()
        .map(|l|
            l.chars()
                .map(|ch| Node::from_char(ch))
                .collect()
        )
        .collect();

    for i in 0..SIZE {
        for j in 0..SIZE {
            if grid[i][j] == Node::Robot {
                robot = Point::new(i as i32, j as i32);
            }
        }
    }

    (grid, robot)
}

fn parse_moves(input: &str) -> Vec<char> {
    input.chars()
        .filter(|ch| !ch.is_whitespace())
        .collect()
}


