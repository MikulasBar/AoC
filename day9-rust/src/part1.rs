use std::mem;

const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let mut nums = parse_input(INPUT);
    defragment(&mut nums);
    let check_sum = check_sum(nums);
    println!("{:?}", check_sum);
    
}

fn check_sum(nums: Vec<Option<u32>>) -> u64 {
    let mut res = 0;

    for (i, opt) in nums.into_iter().enumerate() {
        match opt {
            None | Some(0) => continue,
            Some(x) => res += i as u64 * x as u64,
        }
    }

    res
}

fn defragment(nums: &mut Vec<Option<u32>>) {
    let mut index = nums.len();

    for i in 0..nums.len() {
        if nums[i] == Some(0) || index <= i {break}
        if nums[i].is_some() {continue}

        let r = get_from_right(&mut index, nums);
        nums[i] = r;
        nums[index] = Some(0)
    }
}

fn get_from_right(index: &mut usize, nums: &[Option<u32>]) -> Option<u32> {
    *index -= 1;
    while nums[*index].is_none() {
        *index -= 1;
    }

    nums[*index]
}

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut alt = false;
    let mut vec = vec![];
    let mut id: u32 = 1;
    let mut chars = input.chars();

    chars.next(); // skip the first because it has 0 id and it would break the defragmentation

    for ch in chars {
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