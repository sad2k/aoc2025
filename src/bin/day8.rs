use std::collections::HashMap;
use std::fs;
use std::str::Lines;

type Coord = (u64, u64, u64);

fn parse(lines: Lines) -> Vec<Coord> {
    lines
        .map(|s| {
            let v = s
                .split(",")
                .map(|s2| s2.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        })
        .collect()
}

fn calculate_distances(input: &Vec<Coord>) -> Vec<(Coord, Coord, f64)> {
    let mut res = Vec::new();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let fst = input[i];
            let snd = input[j];
            res.push((fst, snd, calculate_distance(fst, snd)));
        }
    }
    res.sort_by(|c1, c2| c1.2.partial_cmp(&c2.2).unwrap());
    res
}

fn calculate_distance(fst: Coord, snd: Coord) -> f64 {
    let d0 = (fst.0 as i64 - snd.0 as i64);
    let d1 = (fst.1 as i64 - snd.1 as i64);
    let d2 = (fst.2 as i64 - snd.2 as i64);
    ((d0 * d0 + d1 * d1 + d2 * d2) as f64).sqrt()
}

fn part1(input: &Vec<Coord>) -> u64 {
    let distances = calculate_distances(&input);
    // println!("{:?}", distances);
    let num_connections: usize = if input.len() < 1000 { 10 } else { 1000 };
    let mut coord_to_circuits = HashMap::new();
    for c in input {
        coord_to_circuits.insert(c.clone(), 0);
    }
    let mut circuits_to_coords = HashMap::new();
    let mut id_gen = 1;
    for i in 0..num_connections {
        let (from, to, _) = &distances[i];
        let from_circuit = coord_to_circuits[from];
        let to_circuit = coord_to_circuits[to];
        match (from_circuit, to_circuit) {
            (0, 0) => {
                // create new circuit
                let id = id_gen;
                id_gen += 1;
                *coord_to_circuits.get_mut(from).unwrap() = id;
                *coord_to_circuits.get_mut(to).unwrap() = id;
                circuits_to_coords.insert(id, vec![*from, *to]);
            }
            (0, circuit) => {
                // add to existing circuit
                *coord_to_circuits.get_mut(from).unwrap() = circuit;
                circuits_to_coords.get_mut(&circuit).unwrap().push(*from);
            }
            (circuit, 0) => {
                // add to existing circuit
                *coord_to_circuits.get_mut(to).unwrap() = circuit;
                circuits_to_coords.get_mut(&circuit).unwrap().push(*to);
            }
            (circuit1, circuit2) if circuit1 != circuit2 => {
                // merge 2 circuits
                // arbitrarily move from 2nd to 1st
                let moved_coords = circuits_to_coords.remove(&circuit2).unwrap();
                for c in moved_coords {
                    circuits_to_coords
                        .get_mut(&circuit1)
                        .unwrap()
                        .push(c.clone());
                    *coord_to_circuits.get_mut(&c).unwrap() = circuit1;
                }
            }
            (_, _) => {}
        }
    }
    // println!("{:?}", circuits_to_coords);
    let mut top = circuits_to_coords
        .values()
        .map(|v| v.len() as u64)
        .collect::<Vec<_>>();
    top.sort();
    top.reverse();
    top.iter().take(3).product::<u64>()
}

fn main() {
    let contents = fs::read_to_string("inputs/day8.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    println!("{:?}", part1(&parsed));
}
