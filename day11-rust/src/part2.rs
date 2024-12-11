use std::collections::HashMap;


const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let stones = parse_input(INPUT);

    let mut res = 0;
    let mut memo = HashMap::new();
    for s in stones {
        res += blink(s, 0, 75, &mut memo);
    }

    println!("{:?}", res);
}

fn blink(stone: u128, depth: usize, max: usize, memo: &mut HashMap<(u128, usize), u128>) -> u128 {
    if depth == max {
        return 1;
    }

    if let Some(n) = memo.get(&(stone, depth)) {
        return *n;
    }

    if stone == 0 {
        let res = blink(1, depth + 1, max, memo);
        memo.insert((stone, depth), res);
        return res;
    }

    let str = stone.to_string();
    if str.len() % 2 == 0 {
        let (left, right) = split_num(str);
        let res = blink(left, depth + 1, max, memo) + blink(right, depth + 1, max, memo);
        memo.insert((stone, depth), res);
        return res;
    }

    let res = blink(stone * 2024, depth + 1, max, memo);
    memo.insert((stone, depth), res);
    return res;
}

fn split_num(str: String) -> (u128, u128) {
    let mid = str.len() / 2;
    let left = str[..mid].parse().unwrap();
    let right = str[mid..].parse().unwrap();

    (left, right)
}

fn parse_input(input: &str) -> Vec<u128> {
    input.split(" ")
        .map(|s| s.parse().unwrap())
        .collect()
}