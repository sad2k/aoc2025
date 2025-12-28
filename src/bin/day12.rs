use std::fs;
use std::str::Lines;

#[derive(Debug)]
struct Present {
    units: usize,
}

#[derive(Debug)]
struct Shape {
    width: usize,
    length: usize,
    presents: Vec<usize>,
}

fn parse(lines: Lines) -> (Vec<Present>, Vec<Shape>) {
    let mut presents = Vec::new();
    let mut shapes = Vec::new();
    let lines = lines.collect::<Vec<_>>();
    let spl = lines.split(|l| l.trim().is_empty()).collect::<Vec<_>>();

    // presents
    for plines in spl[0..spl.len() - 1].iter() {
        let filtered_plines = &plines[1..plines.len()];
        let units = filtered_plines
            .iter()
            .map(|s| s.chars().filter(|ch| *ch == '#').count())
            .sum::<usize>();
        presents.push(Present { units });
    }

    // shapes
    for line in spl[spl.len() - 1] {
        let spl = line.split(": ").collect::<Vec<_>>();
        let size = spl[0]
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let presents = spl[1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        shapes.push(Shape {
            width: size[0],
            length: size[1],
            presents,
        });
    }

    (presents, shapes)
}

fn part1(presents: &Vec<Present>, shapes: &Vec<Shape>) -> usize {
    let mut res = 0;
    for shape in shapes {
        let area = shape.width as u64 * shape.length as u64;

        // if definitely can't fit
        let min_area = shape
            .presents
            .iter()
            .zip(presents)
            .map(|(s, p)| (*s as u64) * p.units as u64)
            .sum::<u64>();
        let max_area = shape
            .presents
            .iter()
            .map(|s| (*s as u64) * 9u64)
            .sum::<u64>();
        if min_area > area {
            // definitely can't fit
        } else if max_area <= area {
            // definitely can fit
            res += 1;
        } else {
            panic!("don't know!");
        }
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day12.txt").unwrap();
    let lines = contents.lines();
    let (presents, shapes) = parse(lines);

    // part 1
    println!("{:?}", part1(&presents, &shapes));
}
