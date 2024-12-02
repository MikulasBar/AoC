

const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let mut first: [i32; 1000] = [0; 1000];
    let mut second: [i32; 1000] = [0; 1000];

    INPUT.lines()
        .map(|line| {
            line.split_once("   ").unwrap()
        })
        .map(|(x, y)| (
            x.parse::<i32>().unwrap(),
            y.parse::<i32>().unwrap(),
        ))
        .enumerate()
        .for_each(|(i, (x, y))| {
            first[i] = x;
            second[i] = y;
        });
    
    first.sort();
    second.sort();

    let result: i32 = first.iter().zip(second.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    println!("{}", result);
}
