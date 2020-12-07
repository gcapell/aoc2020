use daggy;
use daggy::{Dag, EdgeIndex, NodeIndex, Walker};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::BufRead;

pub fn dot() {
    let re = Regex::new(r"(\d+) (.*) bags?\.?").unwrap();
    let mut dag = Dag::<u32, usize, u32>::new();
    let mut node_names: HashMap<String, usize> = HashMap::new();
    let mut name_nodes: HashMap<usize, String> = HashMap::new();

    for line in io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        if let [outer, inner] = line.split(" contain ").collect::<Vec<&str>>().as_slice() {
            let outer = outer.strip_suffix(" bags").unwrap();
            let outer_id = find_or_add(outer, &mut dag, &mut node_names, &mut name_nodes);
            if *inner == "no other bags." {
                continue;
            }
            for x in inner.split(", ") {
                let mat = re.captures(x).unwrap();
                let (count, name) = (mat[1].parse::<usize>().unwrap(), &mat[2]);
                let inner_id = find_or_add(name, &mut dag, &mut node_names, &mut name_nodes);
                dag.add_edge(NodeIndex::new(outer_id), NodeIndex::new(inner_id), count)
                    .unwrap();
            }
        }
    }
    if false {
        graphviz(&dag, &name_nodes);
    }
    let start = NodeIndex::new(*(node_names.get("shiny gold").unwrap()));
    let mut visited = HashSet::new();
    println!(
        "ancestors:{}",
        count_distinct_ancestors(&dag, start, &mut visited)
    );
    let mut cache = HashMap::new();
    println!("contents:{}", contents(&dag, start, &mut cache));
}

fn contents(
    dag: &Dag<u32, usize, u32>,
    start: NodeIndex,
    cache: &mut HashMap<NodeIndex, u32>,
) -> u32 {
    if let Some(n) = cache.get(&start) {
        return *n;
    }
    let mut count: u32 = 0;
    for (e, c) in dag.children(start).iter(dag) {
        let w = *dag.edge_weight(e).unwrap() as u32;
        count += w * (1 + contents(dag, c, cache));
    }
    cache.insert(start, count);
    count
}

fn count_distinct_ancestors(
    dag: &Dag<u32, usize, u32>,
    start: NodeIndex,
    visited: &mut HashSet<NodeIndex>,
) -> u32 {
    let mut count = 0;
    for (_, p) in dag.parents(start).iter(dag) {
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        count += 1 + count_distinct_ancestors(dag, p, visited);
    }
    count
}

fn graphviz(dag: &Dag<u32, usize, u32>, nodes: &HashMap<usize, String>) {
    println!("digraph G {{");
    for e in 0..dag.edge_count() {
        let e = EdgeIndex::new(e);
        let w = dag.edge_weight(e).unwrap();
        let (a, b) = dag.edge_endpoints(e).unwrap();
        println!(
            r#"   "{}"-> "{}" [ label = "{}"] "#,
            nodes.get(&a.index()).unwrap(),
            nodes.get(&b.index()).unwrap(),
            w
        );
    }
    println!("}}");
}

fn find_or_add(
    name: &str,
    dag: &mut Dag<u32, usize, u32>,
    names: &mut HashMap<String, usize>,
    nodes: &mut HashMap<usize, String>,
) -> usize {
    if let Some(o) = names.get(name) {
        *o
    } else {
        let i = dag.add_node(0).index();
        names.insert(name.to_string(), i);
        nodes.insert(i, name.to_string());
        i
    }
}
