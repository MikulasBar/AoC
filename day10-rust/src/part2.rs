
const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 54;

pub fn run() {
    let mut grid = parse_input(INPUT);
    set_trails(&mut grid);
    let sum = sum_trailheads(&grid);
    // println!("{:?}", grid);
    println!("{:?}", sum);
}


fn sum_trailheads(grid: &Vec<Vec<(u8, u32)>>) -> u64 {
    let mut result = 0;

    for i in 0..SIZE {
        for j in 0..SIZE {
            if grid[i][j].0 == 0 {
                result += grid[i][j].1 as u64;
            }
        }
    }

    result
}

fn set_trails(grid: &mut Vec<Vec<(u8, u32)>>) {
    for n in (0..=8).rev() {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if grid[i][j].0 == n {
                    grid[i][j].1 = get_amount_of(i as i32, j as i32, n+1, grid);
                }
            }
        }
    }
}


fn get_amount_of(i: i32, j: i32, wanted: u8, grid: &Vec<Vec<(u8, u32)>>) -> u32 {
    let mut amount = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            let off_i = i + x;
            let off_j = j + y;

            if x.abs() + y.abs() != 1 {continue}
            if off_i >= SIZE as i32 || off_i < 0 {continue}
            if off_j >= SIZE as i32 || off_j < 0 {continue}

            let n = grid[off_i as usize][off_j as usize];
            if n.0 == wanted {
                amount += n.1;
            }
        }
    }

    amount
}

fn parse_input(input: &str) -> Vec<Vec<(u8, u32)>> {
    input.lines()
        .map(|l| l.chars()
            .map(|ch| {
                let n = ch as u8 - 48;
                if n == 9 {
                    (n, 1)
                } else {
                    (n, 0)
                }
            })
            .collect()
        ).collect()
}