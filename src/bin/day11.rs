use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::Lines;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    output_edges: HashSet<&'a str>,
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

fn parse(lines: Lines) -> Graph {
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    for line in lines {
        let spl = line.split(": ").collect::<Vec<_>>();
        let name = spl[0];
        let output_edges = spl[1].split_whitespace().collect::<HashSet<_>>();
        nodes.insert(name, Node { name, output_edges });
    }
    Graph { nodes }
}

fn part1(graph: &Graph) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back("you");
    let mut res = 0;
    while !queue.is_empty() {
        let el = queue.pop_front().unwrap();
        if el == "out" {
            res += 1;
        } else {
            let node = &graph.nodes[el];
            for oe in &node.output_edges {
                queue.push_back(oe);
            }
        }
    }
    res
}

fn traverse_cached<'a, 'c>(
    graph: &'a Graph,
    from: &'a str,
    cache: &'c mut HashMap<&'a str, Vec<Vec<&'a str>>>,
) -> &'c Vec<Vec<&'a str>> {
    if !cache.contains_key(from) {
        if from == "out" {
            cache.insert("out", vec![vec!["out"]]);
        } else {
            let mut cached_res = Vec::new();
            let node = &graph.nodes[from];
            for oe in &node.output_edges {
                let paths = traverse_cached(graph, oe, cache);
                for p in paths {
                    let mut cached_path = Vec::new();
                    cached_path.push(from);
                    cached_path.extend(p.iter().copied());
                    cached_res.push(cached_path);
                }
            }
            cache.insert(from, cached_res);
        }
    }
    &cache[from]
}

fn part2(graph: &Graph) -> usize {
    let mut cache = HashMap::new();
    let paths = traverse_cached(&graph, &"svr", &mut cache);
    println!("{:?}", paths);
    0
}

fn main() {
    let contents = fs::read_to_string("inputs/day11.txt").unwrap();
    let lines = contents.lines();
    let parsed = parse(lines);

    // part 1
    // println!("{:?}", part1(&parsed));

    // part 2
    println!("{:?}", part2(&parsed));
}
