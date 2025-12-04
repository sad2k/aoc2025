use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::ops::Deref;
use std::str::Lines;

type Grid = HashSet<(i32, i32)>;

fn parse(lines: Lines<'_>) -> Grid {
    let mut r: usize = 0;
    let mut c: usize = 0;
    let mut rolls: HashSet<(i32, i32)> = HashSet::new();
    for line in lines {
        for (c, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((r as i32, c as i32));
            }
        }
        r += 1;
        c = line.len();
    }
    rolls
}

fn part1(grid: &Grid) -> usize {
    let mut res = 0;
    for (r, c) in grid.iter() {
        if is_accessible(*r, *c, grid) {
            res += 1;
        }
    }
    res
}

fn part2(grid: &Grid) -> usize {
    let mut rolls = grid.clone();
    let mut res = 0;
    loop {
        let deleted = delete_rolls(&mut rolls);
        if deleted == 0 {
            break;
        }
        res += deleted;
    }
    res
}

fn delete_rolls(grid: &mut Grid) -> usize {
    let mut to_delete = HashSet::new();
    for (r, c) in grid.iter() {
        if is_accessible(*r, *c, &grid) {
            to_delete.insert((*r, *c));
        }
    }
    for d in &to_delete {
        grid.remove(&d);
    }
    to_delete.len()
}

fn is_accessible(r: i32, c: i32, grid: &Grid) -> bool {
    let candidates: [(i32, i32); 8] = [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ];
    candidates
        .iter()
        .filter(|(r, c)| grid.contains(&(*r, *c)))
        .count()
        < 4
}

fn main() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    // println!("{}", part1(&parsed));

    // part 2
    println!("{}", part2(&parsed));
}
