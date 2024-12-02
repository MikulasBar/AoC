

const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let amount: i32 = INPUT.lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|r| is_valid(r))
        .sum();

    println!("{}", amount);
}

fn is_valid(report: Vec<i32>) -> i32 {
    let sign = (report[0] - report[report.len()-1]).signum();

    if sign == 0 {return 0}

    for i in 0..report.len()-1 {
        let diff = report[i] - report[i+1];

        if !is_valid_diff(diff, sign) {
            let mut vec1 = report.clone();
            vec1.remove(i);
            let res1 = check_report(vec1);
            
            if res1 == 1 {
                return 1;
            }

            let mut vec2 = report.clone();
            vec2.remove(i+1);
            let res2 = check_report(vec2);

            if res2 == 1 {
                return 1;
            }

            return 0;
        }
    }

    return 1;
}

fn check_report(report: Vec<i32>) -> i32 {
    let sign = (report[0] - report[1]).signum();

    if sign == 0 {
        return 0;
    }

    for i in 0..report.len()-1 {
        let diff = report[i] - report[i+1];

        if !is_valid_diff(diff, sign) {
            return 0;
        }
    }

    return 1;
}

fn is_valid_diff(diff: i32, sign: i32) -> bool {
    if diff.signum() != sign || diff.abs() > 3 {
        false
    } else {
        true
    }
}