
const INPUT: &str = include_str!("./input.txt");
// const INPUT: &str = include_str!("./test_input.txt");
const SIZE: usize = 50;

pub fn run() {
    let (mut grid, moves, mut robot) = parse_input(INPUT);
    // draw_grid(&grid);

    for dir in moves.into_iter().map(char_to_dir) {
        move_robot(&mut robot, &mut grid, dir);
        // draw_grid(&grid);
    }

    let mut res = 0;
    for i in 0..SIZE {
        for j in 0..SIZE * 2 {
            if grid[i][j] == Node::BoxL {
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
    BoxL,
    BoxR,
}

impl Node {
    pub fn pair_from_char(ch: char) -> [Self; 2] {
        match ch {
            '.' => [Node::Empty; 2],
            '#' => [Node::Wall; 2],
            'O' => [Node::BoxL, Node::BoxR],
            '@' => [Node::Robot, Node::Empty],
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

fn draw_grid(grid: &Vec<Vec<Node>>) {
    for i in 0..SIZE {
        for j in 0..SIZE * 2 {
            let ch = node_to_char(grid[i][j]);
            print!("{}", ch);
        }

        println!();
    }
}

fn node_to_char(node: Node) -> char {
    match node {
        Node::BoxL => '[',
        Node::BoxR => ']',
        Node::Empty => '.',
        Node::Robot => '@',
        Node::Wall => '#',
    }
}

fn move_robot(robot: &mut Point, grid: &mut Vec<Vec<Node>>, dir: (i8, i8)) {
    let new_pos = robot.add_dir(dir);
    let new_node = new_pos.get(grid);

    match new_node {
        Node::Wall => return,
        Node::BoxL | Node::BoxR => move_box_row(robot, dir, grid),
        Node::Empty => {
            robot.set(grid, Node::Empty);
            *robot = new_pos;
            robot.set(grid, Node::Robot);
        },

        _ => unreachable!(),
    }
}

fn move_box_row(robot: &mut Point, dir: (i8, i8), grid: &mut Vec<Vec<Node>>) {
    let new_pos = robot.add_dir(dir);

    if dir.0 == 0 {
        move_horizontal_box(robot, dir, grid);
        return;
    }

    if !can_move_vertical(*robot, dir, grid) {
        return;
    }

    move_vertical_box(*robot, Node::Robot, dir, grid);
    *robot = new_pos;
}

fn can_move_vertical(pos: Point, dir: (i8, i8), grid: &Vec<Vec<Node>>) -> bool {
    let new_pos = pos.add_dir(dir);
    let new_node = new_pos.get(grid);
    match new_node {
        Node::Empty => true,
        Node::Wall => false,
        Node::BoxL | Node::BoxR => {
            let new_other = other_box_pos(new_pos, new_node);
            can_move_vertical(new_pos, dir, grid) &&
            can_move_vertical(new_other, dir, grid)
        },
        _ => unreachable!(),
    }
}

fn move_vertical_box(pos: Point, node: Node, dir: (i8, i8), grid: &mut Vec<Vec<Node>>) {
    let new_pos = pos.add_dir(dir);
    let new_node = new_pos.get(grid);
    match new_node {
        Node::Empty => (),
        Node::BoxL | Node::BoxR => {
            let new_other = other_box_pos(new_pos, new_node);
            move_vertical_box(new_pos, new_node, dir, grid);
            move_vertical_box(new_other, new_other.get(grid), dir, grid);
        },
        _ => unreachable!(),
    }
    
    new_pos.set(grid, node);
    pos.set(grid, Node::Empty);
}

fn move_horizontal_box(robot: &mut Point, dir: (i8, i8), grid: &mut Vec<Vec<Node>>) {
    let mut curr = robot.add_dir(dir);
    let mut carry = Node::Robot;

    while let Node::BoxL | Node::BoxR = curr.get(grid) {
        carry = curr.get(grid);
        curr = curr.add_dir(dir);
    }

    match curr.get(grid) {
        Node::Wall => return,
        Node::Empty => (),
        _ => unreachable!(),
    }

    let inv_dir = (-dir.0, -dir.1);
    while curr != *robot {
        curr.set(grid, carry);
        carry = other_box_node(carry);
        curr = curr.add_dir(inv_dir);
    }

    robot.set(grid, Node::Empty);
    *robot = robot.add_dir(dir);
    robot.set(grid, Node::Robot);
}

fn other_box_node(node: Node) -> Node {
    match node {
        Node::BoxL => Node::BoxR,
        Node::BoxR => Node::BoxL,
        _ => panic!("Expected box node"),
    }
}

fn other_box_pos(pos: Point, node: Node) -> Point {
    match node {
        Node::BoxL => pos.add_dir((0, 1)),
        Node::BoxR => pos.add_dir((0, -1)),
        _ => panic!("Expected box node"),
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
                .flat_map(|ch| Node::pair_from_char(ch))
                .collect()
        )
        .collect();

    for i in 0..SIZE {
        for j in 0..SIZE * 2 {
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
