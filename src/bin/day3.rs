use std::fs;
use std::str::Lines;

fn parse(lines: Lines<'_>) -> Vec<Vec<u8>> {
    let mut res = Vec::new();
    for line in lines {
        res.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    res
}

fn argmax(bank: &[u8], from: usize, to: usize) -> usize {
    let mut max_value: Option<u8> = None;
    let mut max_idx = 0;
    for i in from..to {
        let value = bank[i];
        if max_value.is_none() || max_value.unwrap() < value {
            max_value = Some(value);
            max_idx = i;
        }
    }
    max_idx
}

fn find_bank_max(bank: &[u8]) -> u64 {
    let fst_idx = argmax(bank, 0, bank.len() - 1);
    ((bank[fst_idx] * 10) + bank[argmax(bank, fst_idx + 1, bank.len())]) as u64
}

fn part1(input: &Vec<Vec<u8>>) -> u64 {
    let mut res = 0;
    for line in input {
        res += find_bank_max(&line);
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    println!("{}", part1(&parsed));
}
