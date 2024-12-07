
const INPUT: &str = include_str!("./input.txt");


fn main() {
    let eqs = parse_equations(INPUT);

    // for e in eqs {
    //     println!("{}: {:?}", e.result, e.nums);
    // }
    let mut res = 0;

    for e in eqs {
        if e.is_possible() {
            res += e.result;
        }
    }

    println!("{}", res);
}


fn parse_equations(input: &str) -> Vec<Equation> {
    return input.lines()
        .map(|line| {
            line.split(" ")
        })
        .map(|mut parts| {
            let first = parts.next().unwrap();
            let result = first[..first.len() - 1].parse::<u128>().unwrap();
            let nums = parts.map(|p| p.parse::<u32>().unwrap()).collect();
            Equation::new(result, nums)
        })
        .collect()
}

struct Equation {
    pub result: u128,
    pub nums: Vec<u32>,
}

impl Equation {
    pub fn new(result: u128, nums: Vec<u32>) -> Self {
        Self {
            result,
            nums,
        }
    }

    pub fn is_possible(&self) -> bool {
        let mut results = vec![self.nums[0] as u128];

        for i in 1..self.nums.len() {
            let length = results.len();
            let next = self.nums[i];

            for j in 0..length {
                results.push(next as u128 + results[j]);
                results.push(concat_nums(results[j], next as u128));
                results[j] *= next as u128;
            }
        }

        if results.contains(&self.result) {
            true
        } else {
            false
        }
    }
}


fn concat_nums(a: u128, b: u128) -> u128 {
    let b_len = b.to_string().len();
    a * 10_u128.pow(b_len as u32) + b
}