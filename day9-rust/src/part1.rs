

const INPUT: &str = include_str!("./input.txt");

// const INPUT: &str = "2333133121414131402";

pub fn run() {
    let nums = parse_input(INPUT);
    let check_sum = check_sum(&nums);
    println!("{:?}", check_sum);  
}

fn check_sum(nums: &Vec<Option<usize>>) -> usize {
    let mut res = 0;
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        if let Some(n) = nums[left] {
            res += left * n;
        } else {
            res += left * get_from_right(&mut right, left, &nums);
        }
        left += 1;
    }

    res
}

fn get_from_right(r: &mut usize, left: usize, nums: &[Option<usize>]) -> usize {
    *r -= 1;
    while nums[*r].is_none() {
        *r -= 1;
    }

    if left >= *r {
        return 0;
    } else {
        nums[*r].unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Option<usize>> {
    let mut alt = true;
    let mut vec = vec![];
    let mut id = 0;

    for ch in input.chars() {
        let n = ch.to_digit(10).unwrap();
        if alt {
            for _ in 0..n {
                vec.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..n {
                vec.push(None);
            }
        }

        alt = !alt;
    }

    return vec;
}