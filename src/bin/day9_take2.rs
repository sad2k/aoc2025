use std::fs;
use std::str::Lines;

fn parse(lines: Lines) -> Vec<(i64, i64)> {
    lines
        .map(|l| {
            let spl = l
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (spl[0], spl[1])
        })
        .collect::<Vec<_>>()
}

fn area(c1: (i64, i64), c2: (i64, i64)) -> i64 {
    (c1.0 - c2.0 + 1).abs() * (c1.1 - c2.1 + 1).abs()
}

fn part1(coords: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let a = area(coords[i], coords[j]);
            if a > max_area {
                max_area = a;
            }
        }
    }
    max_area
}

fn is_vertical_and_horizontal_intersect(
    vert0: (f64, f64),
    vert1: (f64, f64),
    horiz0: (f64, f64),
    horiz1: (f64, f64),
) -> bool {
    vert0.0 >= horiz0.0.min(horiz1.0)
        && vert0.0 <= horiz0.0.max(horiz1.0)
        && horiz0.1 >= vert0.1.min(vert1.1)
        && horiz0.1 <= vert0.1.max(vert1.1)
}

fn is_intersect(p0: (f64, f64), p1: (f64, f64), q0: (f64, f64), q1: (f64, f64)) -> bool {
    if p0.0 == p1.0 {
        // p is vertical
        if q0.1 == q1.1 {
            // q is horizontal
            return is_vertical_and_horizontal_intersect(p0, p1, q0, q1);
        }
    } else {
        // p is horizontal
        if q0.0 == q1.0 {
            // p is vertical
            return is_vertical_and_horizontal_intersect(q0, q1, p0, p1);
        }
    }
    false
}

fn have_intersections(p0: (i64, i64), p1: (i64, i64), coords: &[(i64, i64)]) -> bool {
    let p0small = (p0.0.min(p1.0) as f64 + 0.5, p0.1.min(p1.1) as f64 + 0.5);
    let p1small = (p0.0.max(p1.0) as f64 - 0.5, p0.1.max(p1.1) as f64 - 0.5);
    let square_perimeter = vec![
        ((p0small.0, p0small.1), (p1small.0, p0small.1)),
        ((p1small.0, p0small.1), (p1small.0, p1small.1)),
        ((p1small.0, p1small.1), (p0small.0, p1small.1)),
        ((p0small.0, p1small.1), (p0small.0, p0small.1)),
    ];
    for i in 0..coords.len() {
        let q0: (f64, f64);
        let q1: (f64, f64);
        q0 = (coords[i].0 as f64, coords[i].1 as f64);
        q1 = if i == coords.len() - 1 {
            (coords[0].0 as f64, coords[0].1 as f64)
        } else {
            (coords[i + 1].0 as f64, coords[i + 1].1 as f64)
        };
        for (p0prime, p1prime) in &square_perimeter {
            if is_intersect(*p0prime, *p1prime, q0, q1) {
                // println!("intersection: {:?} {:?} {:?} {:?}", p0, p1, q0, q1);
                return true;
            }
        }
    }
    false
}

fn part2(coords: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            if !have_intersections(coords[i], coords[j], coords) {
                let a = area(coords[i], coords[j]);
                if a > max_area {
                    max_area = a;
                }
            }
        }
    }
    max_area
}

fn main() {
    let contents = fs::read_to_string("inputs/day9.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);
    println!("{:?}", parsed);

    // part 1
    // println!("{}", part1(&parsed));

    // part 2
    println!("{}", part2(&parsed));
}
