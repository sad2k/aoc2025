use std::fs;

fn parse_ranges(lines: &[&str]) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    for l in lines {
        let spl: Vec<u64> = l.split("-").map(|x| x.parse::<u64>().unwrap()).collect();
        res.push((spl[0], spl[1]));
    }
    res
}

fn parse_ids(lines: &[&str]) -> Vec<u64> {
    lines.iter().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn part1(ids: &[u64], ranges: &[(u64, u64)]) -> usize {
    let mut res = 0;
    for id in ids {
        if is_fresh(*id, ranges) {
            res += 1;
        }
    }
    res
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    for &(from, to) in ranges {
        if from > id {
            break;
        }
        if from <= id && to >= id {
            return true;
        }
    }
    false
}

fn main() {
    let contents = fs::read_to_string("inputs/day5.txt").unwrap();
    let lines: Vec<_> = contents.lines().collect();
    let lines_split = lines.split(|x| x.trim().is_empty()).collect::<Vec<_>>();
    let mut ranges = parse_ranges(lines_split[0]);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let ids = parse_ids(&lines_split[1]);

    // part 1
    println!("{:?}", part1(&ids, &ranges));
}
