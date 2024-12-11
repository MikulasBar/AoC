
const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let mut stones = parse_input(INPUT);

    for _ in 0..25 {
        blink(&mut stones);
    }

    println!("{:?}", stones.len());
}

// I use Vec<> because the other alternatives are not much better.
// The LinkedList is scuffed and doesn't provide the insert method.

fn blink(stones: &mut Vec<u128>) {
    let mut i = 0;
    let mut max = stones.len();
    
    while i < max {
        let n = stones[i];
        if n == 0 {
            stones[i] = 1;
            i += 1;
            continue;
        }

        let str = n.to_string();
        if str.len() % 2 == 0 {
            let (left, right) = split_num(str);
            stones[i] = left;
            stones.insert(i+1, right);
            i += 2;
            max += 1;
            continue;
        }

        stones[i] *= 2024;
        i += 1;
    }
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


