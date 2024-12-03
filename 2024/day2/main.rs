use std::fs;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let order = report[0] > report[1];
    for window in report.windows(2) {
        let order_check = (window[0] > window[1]) == order;
        let diff = (window[0] - window[1]).abs() >= 1 && (window[0] - window[1]).abs() <= 3;
        if !order_check || !diff {
            return false;
        }
    }
    true
}

fn part1(input: Vec<Vec<i32>>) {
    let mut count = 0;
    for report in input {
        if is_safe(&report) {
            count += 1;
        }
    }
    println!("{count}");
}

fn part2(input: Vec<Vec<i32>>) {
    let mut count = 0;
    for report in input {
        if is_safe(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut temp_report = report.clone();
                temp_report.remove(i);
                if is_safe(&temp_report) {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("{count}");
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = parse(&contents);
    part1(input.clone());
    part2(input.clone());
}
