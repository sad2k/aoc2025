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

#[derive(Debug)]
enum CornerType {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[derive(Debug)]
enum CornerDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
enum LineDirection {
    Left,
    Right,
    Down,
    Up,
}

fn line_direction(p0: (i64, i64), p1: (i64, i64)) -> LineDirection {
    if p1.0 - p0.0 > 0 {
        LineDirection::Right
    } else if p1.0 - p0.0 < 0 {
        LineDirection::Left
    } else if p1.1 - p0.1 > 0 {
        LineDirection::Down
    } else {
        LineDirection::Up
    }
}

fn classify_corner(corner: &[(i64, i64)]) -> (CornerType, CornerDirection) {
    let dir1 = line_direction(corner[0], corner[1]);
    let dir2 = line_direction(corner[1], corner[2]);
    match (&dir1, &dir2) {
        (LineDirection::Right, LineDirection::Down) => {
            (CornerType::TopRight, CornerDirection::Clockwise)
        }
        (LineDirection::Right, LineDirection::Up) => {
            (CornerType::BottomRight, CornerDirection::CounterClockwise)
        }
        (LineDirection::Right, LineDirection::Right) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Right, LineDirection::Left) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Left, LineDirection::Down) => {
            (CornerType::TopLeft, CornerDirection::CounterClockwise)
        }
        (LineDirection::Left, LineDirection::Up) => {
            (CornerType::BottomLeft, CornerDirection::Clockwise)
        }
        (LineDirection::Left, LineDirection::Left) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Left, LineDirection::Right) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Up, LineDirection::Left) => {
            (CornerType::TopRight, CornerDirection::CounterClockwise)
        }
        (LineDirection::Up, LineDirection::Right) => {
            (CornerType::TopLeft, CornerDirection::Clockwise)
        }
        (LineDirection::Up, LineDirection::Up) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Up, LineDirection::Down) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Down, LineDirection::Left) => {
            (CornerType::BottomRight, CornerDirection::Clockwise)
        }
        (LineDirection::Down, LineDirection::Right) => {
            (CornerType::BottomLeft, CornerDirection::CounterClockwise)
        }
        (LineDirection::Down, LineDirection::Down) => panic!("bad corner: {dir1:?} {dir2:?}"),
        (LineDirection::Down, LineDirection::Up) => panic!("bad corner: {dir1:?} {dir2:?}"),
    }
}

fn build_outer_shell(coords: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    let mut corner_coords = Vec::new();
    for i in 0..coords.len() {
        let c = coords[i];
        corner_coords.clear();
        if i == 0 {
            corner_coords.push(coords[coords.len() - 1]);
            corner_coords.push(coords[0]);
            corner_coords.push(coords[1]);
        } else if i == coords.len() - 1 {
            corner_coords.push(coords[i - 1]);
            corner_coords.push(coords[i]);
            corner_coords.push(coords[0]);
        } else {
            corner_coords.push(coords[i - 1]);
            corner_coords.push(coords[i]);
            corner_coords.push(coords[i + 1]);
        }
        let (corner_type, corner_direction) = classify_corner(&corner_coords);
        // println!("{corner_type:?} {corner_direction:?}");
        res.push(match (corner_type, corner_direction) {
            // clockwise
            (CornerType::TopRight, CornerDirection::Clockwise) => (c.0 + 1, c.1 - 1),
            (CornerType::BottomRight, CornerDirection::Clockwise) => (c.0 + 1, c.1 + 1),
            (CornerType::BottomLeft, CornerDirection::Clockwise) => (c.0 - 1, c.1 + 1),
            (CornerType::TopLeft, CornerDirection::Clockwise) => (c.0 - 1, c.1 - 1),
            // counterclockwise
            (CornerType::TopRight, CornerDirection::CounterClockwise) => (c.0 - 1, c.1 + 1),
            (CornerType::BottomRight, CornerDirection::CounterClockwise) => (c.0 - 1, c.1 - 1),
            (CornerType::BottomLeft, CornerDirection::CounterClockwise) => (c.0 + 1, c.1 - 1),
            (CornerType::TopLeft, CornerDirection::CounterClockwise) => (c.0 + 1, c.1 + 1),
        });
    }
    res
}

fn find_orientation(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> i64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn on_segment(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    // assumes a, b, c are collinear; checks if c lies within the bounding box of segment ab
    c.0 >= a.0.min(b.0) && c.0 <= a.0.max(b.0) && c.1 >= a.1.min(b.1) && c.1 <= a.1.max(b.1)
}

fn is_intersect(p0: (i64, i64), p1: (i64, i64), q0: (i64, i64), q1: (i64, i64)) -> bool {
    let o1 = find_orientation(p0, p1, q0);
    let o2 = find_orientation(p0, p1, q1);
    let o3 = find_orientation(q0, q1, p0);
    let o4 = find_orientation(q0, q1, p1);

    // if o1 == 0 && o2 == 0 && o3 == 0 && o4 == 0 {
    //     return false;
    // }

    if (o1 > 0) != (o2 > 0) && (o3 > 0) != (o4 > 0) {
        return true;
    }

    if o1 == 0 && on_segment(p0, p1, q0) {
        return true;
    }
    if o2 == 0 && on_segment(p0, p1, q1) {
        return true;
    }
    if o3 == 0 && on_segment(q0, q1, p0) {
        return true;
    }
    if o4 == 0 && on_segment(q0, q1, p1) {
        return true;
    }
    false
}

fn have_intersections(p0: (i64, i64), p1: (i64, i64), outer_shell: &[(i64, i64)]) -> bool {
    let square_perimeter = vec![
        ((p0.0, p0.1), (p1.0, p0.1)),
        ((p1.0, p0.1), (p1.0, p1.1)),
        ((p1.0, p1.1), (p0.0, p1.1)),
        ((p0.0, p1.1), (p0.0, p0.1)),
    ];
    for i in 0..outer_shell.len() {
        let q0: (i64, i64);
        let q1: (i64, i64);
        q0 = outer_shell[i];
        q1 = if i == outer_shell.len() - 1 {
            outer_shell[0]
        } else {
            outer_shell[i + 1]
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
    let outer_shell = build_outer_shell(coords);
    println!("{:?}", outer_shell);
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            if !have_intersections(coords[i], coords[j], &outer_shell) {
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
