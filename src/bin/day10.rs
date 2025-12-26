use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::str::Lines;

#[derive(Debug)]
struct Machine {
    diagram: Vec<bool>,
    buttons: Vec<Vec<u32>>,
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
        res.push(Machine { diagram, buttons })
    }
    res
}

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
            if new_current.is_none() || step < new_current.unwrap() {
                new_current = Some(step);
            }
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

fn solve(machine: &Machine) -> u64 {
    let state = vec![false; machine.diagram.len()];
    let buttons = HashSet::from_iter(machine.buttons.clone());
    solve0(&state, &machine.diagram, &buttons, 1, None).unwrap() as u64
}

fn part1(machines: &Vec<Machine>) -> u64 {
    let mut res = 0;
    for (i, m) in machines.iter().enumerate() {
        println!("Solving {i} buttons: {}", m.buttons.len());
        res += solve(m);
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day10.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    println!("{:?}", part1(&parsed));
}
