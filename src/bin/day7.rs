use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Lines;

#[derive(Debug)]
struct Diagram {
    splitters: Vec<(u64, u64)>,
    start: (u64, u64),
    rows: u64,
    cols: u64,
}

fn parse(lines: Lines<'_>) -> Diagram {
    let mut i = 0;
    let mut cols: u64 = 0;
    let mut splitters = Vec::new();
    let mut start: Option<(u64, u64)> = None;
    for line in lines {
        let chars: Vec<_> = line.chars().collect();
        cols = chars.len() as u64;
        for j in 0..chars.len() {
            match chars[j] {
                '^' => {
                    splitters.push((i, j as u64));
                }
                'S' => {
                    start = Some((i, j as u64));
                }
                _ => {}
            }
        }
        i += 1;
    }
    Diagram {
        splitters,
        start: start.unwrap(),
        rows: i,
        cols,
    }
}

fn part1(diagram: &Diagram) -> u64 {
    let mut res = 0;
    let mut beams = HashSet::new();
    beams.insert(diagram.start);
    for r in 0..diagram.rows {
        let mut new_beams = HashSet::new();
        for beam in &beams {
            assert_eq!(beam.0, r);
            if diagram.splitters.contains(&beam) {
                res += 1;
                if beam.1 > 0 {
                    new_beams.insert((beam.0 + 1, beam.1 - 1));
                }
                if beam.1 < diagram.cols - 1 {
                    new_beams.insert((beam.0 + 1, beam.1 + 1));
                }
            } else {
                new_beams.insert((beam.0 + 1, beam.1));
            }
        }
        beams = new_beams;
    }
    res
}

fn part2(diagram: &Diagram) -> u64 {
    let mut res = 1;
    let mut beams = HashMap::new();
    beams.insert(diagram.start, 1);
    for r in 0..diagram.rows {
        let mut new_beams = HashMap::new();
        for (beam, mult) in &beams {
            assert_eq!(beam.0, r);
            if diagram.splitters.contains(&beam) {
                res += mult;
                if beam.1 > 0 {
                    *new_beams.entry((beam.0 + 1, beam.1 - 1)).or_insert(0) += mult;
                }
                if beam.1 < diagram.cols - 1 {
                    *new_beams.entry((beam.0 + 1, beam.1 + 1)).or_insert(0) += mult;
                }
            } else {
                *new_beams.entry((beam.0 + 1, beam.1)).or_insert(0) += mult;;
            }
        }
        beams = new_beams;
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day7.txt").unwrap();
    let lines = contents.lines();
    let diagram = parse(lines);

    // part 1
    // println!("{}", part1(&diagram));

    // part 2
    println!("{}", part2(&diagram));
}
