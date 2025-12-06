use std::fs;
use std::str::Lines;

type Problem = (Vec<u64>, char);

fn parse(lines: Lines<'_>) -> Vec<Problem> {
    let mut res = Vec::new();
    let collected_lines: Vec<_> = lines.collect();
    for (i, line) in collected_lines.iter().enumerate() {
        let elements: Vec<_> = line.split_whitespace().collect();
        for (j, el) in elements.iter().enumerate() {
            if i == 0 {
                res.push((vec![el.parse::<u64>().unwrap()], ' '));
            } else if i == collected_lines.len() - 1 {
                res[j].1 = el.chars().nth(0).unwrap();
            } else {
                res[j].0.push(
                    el.parse::<u64>()
                        .expect(format!("not a number: {}", el).as_str()),
                );
            }
        }
    }
    res
}

fn solve(problem: &Problem) -> u64 {
    match problem.1 {
        '+' => problem.0.iter().sum(),
        '*' => problem.0.iter().product(),
        _ => panic!("invalid op: {}", problem.1),
    }
}

fn part1(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(solve).sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").unwrap();
    let lines = contents.lines();
    let problems = parse(lines);

    // part 1
    println!("{:?}", part1(&problems));
}
