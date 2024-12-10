use std::collections::HashSet;


const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 54;

pub fn run() {
    let grid = parse_input(INPUT);
    let sum = sum_trailheads(&grid);
    // println!("{:?}", grid);
    println!("{:?}", sum);
}


fn sum_trailheads(grid: &Vec<Vec<u8>>) -> u64 {
    let mut result = 0;

    for i in 0..SIZE {
        for j in 0..SIZE {
            if grid[i][j] == 0 {
                result += calc_score(i, j, grid);
            }
        }
    }

    result
}

fn calc_score(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> u64 {
    calc_score_helper(i as i32, j as i32, 0, grid).len() as u64
}

fn calc_score_helper(i: i32, j: i32, height: u8, grid: &Vec<Vec<u8>>) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();

    if height == 9 && grid[i as usize][j as usize] == 9 {
        set.insert((i, j));
        return set;
    }
    
    for x in -1..=1 {
        for y in -1..=1 {
            let off_i = i + x;
            let off_j = j + y;

            if x.abs() + y.abs() != 1 {continue}
            if off_i >= SIZE as i32 || off_i < 0 {continue}
            if off_j >= SIZE as i32 || off_j < 0 {continue}
            if grid[off_i as usize][off_j as usize] != height + 1 {continue}

            let branch = calc_score_helper(off_i, off_j, height + 1, grid);
            set.extend(branch);
        }
    }

    set
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars()
            .map(|ch| ch as u8 - 48)
            .collect()
        ).collect()
}