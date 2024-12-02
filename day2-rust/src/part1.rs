
const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let amount: i32 = INPUT.lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .map(|r| is_valid(r))
        .sum();

    println!("{}", amount);
}

fn is_valid(report: Vec<i32>) -> i32 {
    let sign = (report[0] - report[1]).signum();

    if sign == 0 {
        return 0;
    }

    for i in 0..report.len()-1 {
        let diff = report[i] - report[i+1];

        if diff.signum() != sign || diff.abs() > 3 {
            return 0;
        }
    }

    return 1
}
