use std::fs;
use std::str::Lines;

const MAX_POSITION: i32 = 99;

fn part1(lines: Lines<'_>) -> usize {
    let mut result = 0;
    let mut position = 50;
    for line in lines {
        let (direction, distance) = line.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        position += match direction {
            "L" => -distance,
            "R" => distance,
            _ => panic!("Unknown direction: {}", direction),
        };
        position = position % (MAX_POSITION + 1);
        if position < 0 {
            position += MAX_POSITION + 1;
        }
        if position == 0 {
            result += 1;
        }
    }
    result
}

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines = contents.lines();

    // part 1
    println!("{}", part1(lines));
}
