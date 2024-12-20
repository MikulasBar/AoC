use std::collections::HashMap;


const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let (towels, designs) = parse_input(INPUT);

    let mut res = 0;
    let mut memo = HashMap::new();
    for design in designs {
        if is_possible(design, &towels, &mut memo) {
            res += 1;
        }
    }

    println!("RES: {}", res);
}

fn is_possible(design: &str, towels: &Vec<&str>, memo: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&res) = memo.get(design) {
        return res;
    }
    
    for towel in towels {
        if design.starts_with(towel) {
            let start = towel.len();
            if is_possible(&design[start..], towels, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(design.to_string(), false);
    false
}

fn parse_input<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut iter = input.split("\r\n\r\n");

    let towels = parse_towels(iter.next().unwrap());
    let designs = parse_designs(iter.next().unwrap());
    (towels, designs)
}

fn parse_towels<'a>(input: &'a str) -> Vec<&'a str> {
    input.split(", ").collect()
}

fn parse_designs<'a>(input: &'a str) -> Vec<&'a str> {
    input.split("\r\n").collect()
}