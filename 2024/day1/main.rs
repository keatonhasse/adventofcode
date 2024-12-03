use std::collections::HashMap;
use std::fs;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("   ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .unzip()
}

fn part1(input: (Vec<i32>, Vec<i32>)) {
    let mut left = input.0;
    let mut right = input.1;
    left.sort();
    right.sort();
    let mut sum = 0;
    for (left_it, right_it) in left.iter().zip(right.iter()) {
        let diff = (left_it - right_it).abs();
        sum += diff;
    }
    println!("{sum}");
}

fn part2(input: (Vec<i32>, Vec<i32>)) {
    let mut hash_map = HashMap::new();
    let left = input.0;
    let right = input.1;
    for left_it in left.iter() {
        let count = right.iter().filter(|&n| *n == *left_it).count();
        hash_map.insert(left_it, count);
    }
    let mut sum = 0;
    for (left, count) in &hash_map {
        let score = *left * *count as i32;
        sum += score;
    }
    println!("{sum}");
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = parse(&contents);
    part1(input.clone());
    part2(input.clone());
}
