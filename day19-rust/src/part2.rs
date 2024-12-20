use std::collections::HashMap;


const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let (towels, designs) = parse_input(INPUT);

    let mut res = 0;
    let mut memo = HashMap::new();
    for design in designs {
        res += possible_ways(design, &towels, &mut memo);
    }

    println!("RES: {}", res);
}

fn possible_ways(design: &str, towels: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&stored) = memo.get(design) {
        return stored;
    }

    let mut res = 0;
    for towel in towels {
        if design.starts_with(towel) {
            let start = towel.len();
            res += possible_ways(&design[start..], towels, memo);
            memo.insert(design.to_string(), res);
        }
    }

    memo.insert(design.to_string(), res);
    return res;
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