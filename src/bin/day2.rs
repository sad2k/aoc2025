use std::fs;
use std::str::Lines;

const TEN: u64 = 10;
fn is_invalid_id(id: u64) -> bool {
    let mut log = id.ilog(10);
    if log % 2 != 0 {
        log += 1;
    }
    let power_of_ten = TEN.pow(log / 2);
    (id % power_of_ten) == (id / power_of_ten)
}

fn find_sum_of_invalid_ids(from: u64, to: u64) -> u64 {
    let mut result = 0;
    for i in from..=to {
        if is_invalid_id(i) {
            result += i;
        }
    }
    result
}

fn part1(lines: &str) -> u64 {
    let mut result = 0;
    let intervals: Vec<_> = lines
        .split(",")
        .map(|x| {
            x.split("-")
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    for interval in &intervals {
        result += find_sum_of_invalid_ids(interval[0], interval[1]);
    }
    result
}

fn main() {
    let contents = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines = contents.lines();

    // part 1
    println!("{}", part1(lines.clone().next().unwrap()));
}
