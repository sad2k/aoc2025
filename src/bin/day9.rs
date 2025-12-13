use std::fs;
use std::str::Lines;

fn parse(lines: Lines) -> Vec<(i64, i64)> {
    lines
        .map(|l| {
            let spl = l
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (spl[0], spl[1])
        })
        .collect::<Vec<_>>()
}

fn area(c1: (i64, i64), c2: (i64, i64)) -> i64 {
    (c1.0 - c2.0 + 1).abs() * (c1.1 - c2.1 + 1).abs()
}

fn part1(coords: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let a = area(coords[i], coords[j]);
            if a > max_area {
                max_area = a;
            }
        }
    }
    max_area
}

fn main() {
    let contents = fs::read_to_string("inputs/day9.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    println!("{}", part1(&parsed));
}
