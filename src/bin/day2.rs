use std::fs;

const TEN: u64 = 10;

fn is_invalid_id_part1(id: u64) -> bool {
    let mut log = id.ilog(10);
    if log % 2 != 0 {
        log += 1;
    }
    let power_of_ten = TEN.pow(log / 2);
    (id % power_of_ten) == (id / power_of_ten)
}

fn find_sum_of_invalid_ids<F>(from: u64, to: u64, invalid_id_checker: F) -> u64
where
    F: Fn(u64) -> bool,
{
    let mut result = 0;
    for i in from..=to {
        if invalid_id_checker(i) {
            result += i;
        }
    }
    result
}

fn solve<F>(lines: &str, invalid_id_checker: F) -> u64
where
    F: Fn(u64) -> bool,
{
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
        result += find_sum_of_invalid_ids(interval[0], interval[1], &invalid_id_checker);
    }
    result
}

fn main() {
    let contents = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines = contents.lines();

    // part 1
    println!("{}", solve(lines.clone().next().unwrap(), is_invalid_id_part1));

    // part 2
    // println!("{}", part2(lines.clone().next().unwrap()));
}
