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

#[derive(Debug, Clone, Copy)]
struct CachedPaths {
    total_paths: u64,
    paths_via_fft: u64,
    paths_via_dac: u64,
    paths_via_fft_and_dac: u64,
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
    cache: &'c mut HashMap<&'a str, CachedPaths>,
) -> CachedPaths {
    if !cache.contains_key(from) {
        if from == "out" {
            cache.insert(
                "out",
                CachedPaths {
                    total_paths: 1,
                    paths_via_dac: 0,
                    paths_via_fft: 0,
                    paths_via_fft_and_dac: 0,
                },
            );
        } else {
            let node = &graph.nodes[from];
            let mut total_paths = 0;
            let mut paths_via_dac = 0;
            let mut paths_via_fft = 0;
            let mut paths_via_fft_and_dac = 0;
            for oe in &node.output_edges {
                let cached_paths = traverse_cached(graph, oe, cache);
                total_paths += cached_paths.total_paths;
                paths_via_dac += cached_paths.paths_via_dac;
                paths_via_fft += cached_paths.paths_via_fft;
                paths_via_fft_and_dac += cached_paths.paths_via_fft_and_dac;
            }
            match from {
                "fft" => {
                    paths_via_fft = total_paths;
                    paths_via_fft_and_dac = paths_via_dac;
                }
                "dac" => {
                    paths_via_dac = total_paths;
                    paths_via_fft_and_dac = paths_via_fft;
                }
                _ => {}
            }
            cache.insert(
                from,
                CachedPaths {
                    total_paths,
                    paths_via_dac,
                    paths_via_fft,
                    paths_via_fft_and_dac,
                },
            );
        }
    }
    cache.get(from).unwrap().clone()
}

fn part2(graph: &Graph) -> u64 {
    let mut cache = HashMap::new();
    let paths = traverse_cached(&graph, &"svr", &mut cache);
    paths.paths_via_fft_and_dac
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
