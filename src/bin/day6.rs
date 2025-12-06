use std::fs;
use std::str::Lines;

type Problem = (Vec<u64>, char);

fn parse_part1(lines: Lines<'_>) -> Vec<Problem> {
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

fn transpose_numbers(char_vecs: &Vec<&[char]>) -> Vec<u64> {
    let mut res = Vec::new();
    for char_idx in 0..char_vecs[0].len() {
        let mut num = 0;
        for char_vec in char_vecs.iter() {
            let ch = char_vec[char_idx];
            if ch != ' ' {
                num = num * 10 + ch.to_digit(10).unwrap() as u64;
            }
        }
        res.push(num);
    }
    res
}

fn parse_part2(lines: Lines<'_>) -> Vec<Problem> {
    let char_vecs: Vec<_> = lines.map(|s| s.chars().collect::<Vec<_>>()).collect();
    let op_indices = char_vecs[char_vecs.len() - 1]
        .iter()
        .enumerate()
        .filter(|(i, ch)| **ch != ' ')
        .collect::<Vec<_>>();
    let mut res = Vec::new();
    for i in 0..op_indices.len() {
        let mut problem_char_vecs = Vec::new();
        let from = op_indices[i].0;
        for j in 0..char_vecs.len() - 1 {
            let char_vec = &char_vecs[j];
            let to = if i == op_indices.len() - 1 {
                char_vec.len()
            } else {
                (op_indices[i + 1].0) - 1
            };
            problem_char_vecs.push(&char_vec[from..to]);
        }
        res.push((transpose_numbers(&problem_char_vecs), *op_indices[i].1));
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

fn solve_all(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(solve).sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").unwrap();
    let lines = contents.lines();

    // part 1
    // let problems = parse_part1(lines);
    // println!("{:?}", solve_all(&problems));

    // part 1
    let problems = parse_part2(lines);
    println!("{:?}", solve_all(&problems));
}
