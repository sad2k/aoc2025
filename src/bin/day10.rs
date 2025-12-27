use std::collections::HashSet;
use std::fs;
use std::str::Lines;
use z3::Solver;
use z3::ast::Int;

#[derive(Debug)]
struct Machine {
    diagram: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
}

fn parse_button(button: &str) -> Vec<u32> {
    button
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn parse(lines: Lines) -> Vec<Machine> {
    let mut res = Vec::new();
    for line in lines {
        let spl = line.split("]").collect::<Vec<_>>();
        let diagram = spl[0]
            .chars()
            .into_iter()
            .skip(1)
            .map(|ch| ch == '#')
            .collect::<Vec<_>>();
        let spl = spl[1].split("{").collect::<Vec<_>>();
        let buttons = spl[0].trim();
        let buttons = buttons
            .split_whitespace()
            .map(|s| parse_button(&s[1..s.len() - 1]))
            .collect::<Vec<_>>();
        let joltage = spl[1];
        let joltage = &joltage[0..joltage.len() - 1];
        let joltage = joltage
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        res.push(Machine {
            diagram,
            buttons,
            joltage,
        })
    }
    res
}

// Super slow, can't be bothered to optimize further :)
fn solve0(
    state: &Vec<bool>,
    target: &Vec<bool>,
    remaining_buttons: &HashSet<Vec<u32>>,
    step: usize,
    current: Option<usize>,
) -> Option<usize> {
    if current.is_some() && step >= current.unwrap() {
        return current;
    }
    let mut new_current = current.clone();
    for button in remaining_buttons.iter() {
        let mut new_state = state.clone();
        for i in button {
            let ii = *i as usize;
            if new_state[ii] {
                new_state[ii] = false;
            } else {
                new_state[ii] = true;
            }
        }
        if new_state == *target {
            return Some(step);
        } else if remaining_buttons.len() > 1
            && (new_current.is_none() || (step + 1) < new_current.unwrap())
        {
            let mut new_remaining_buttons = remaining_buttons.clone();
            new_remaining_buttons.remove(button);
            let recursive_result = solve0(
                &new_state,
                target,
                &new_remaining_buttons,
                step + 1,
                new_current,
            );
            if recursive_result.is_some()
                && (new_current.is_none() || recursive_result.unwrap() < new_current.unwrap())
            {
                new_current = recursive_result;
            }
        }
    }
    new_current
}

fn solve1(machine: &Machine) -> u64 {
    let state = vec![false; machine.diagram.len()];
    let buttons = HashSet::from_iter(machine.buttons.clone());
    solve0(&state, &machine.diagram, &buttons, 1, None).unwrap() as u64
}

fn part1(machines: &Vec<Machine>) -> u64 {
    let mut res = 0;
    for (i, m) in machines.iter().enumerate() {
        println!("Solving {i} buttons: {}", m.buttons.len());
        res += solve1(m);
    }
    res
}

fn solve2(machine: &Machine) -> u64 {
    let mut presses = Vec::new();
    for b in 0..machine.buttons.len() {
        presses.push(Int::fresh_const(&b.to_string()));
    }
    let solver = Solver::new();
    for p in &presses {
        solver.assert(p.ge(0));
    }
    for (joltage_id, joltage) in machine.joltage.iter().enumerate() {
        let mut included_buttons = Vec::new();
        for (button_id, b) in machine.buttons.iter().enumerate() {
            if b.contains(&(joltage_id as u32)) {
                included_buttons.push(&presses[button_id]);
            }
        }
        if !included_buttons.is_empty() {
            let sum = Int::add(&included_buttons);
            solver.assert(sum.eq(*joltage));
        }
    }
    let solutions = solver.solutions(presses, true);
    let mut res: Option<u64> = None;
    for solution in solutions {
        // println!("Solution {:?}", solution);
        let sum = solution
            .iter()
            .map(Int::as_u64)
            .map(Option::unwrap)
            .sum::<u64>();
        if res.is_none() || sum < res.unwrap() {
            res = Some(sum);
        }
    }
    res.unwrap()
}

fn part2(machines: &Vec<Machine>) -> u64 {
    let mut res = 0;
    for (i, m) in machines.iter().enumerate() {
        println!("Solving {i} buttons: {}", m.buttons.len());
        res += solve2(m);
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day10.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    // println!("{:?}", part1(&parsed));

    // part 1
    println!("{:?}", part2(&parsed));
}
