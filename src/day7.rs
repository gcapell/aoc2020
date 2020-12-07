use daggy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn dot() {
    let re = Regex::new(r"(\d+) (.*) bags?\.?").unwrap();
    let mut dag = daggy::Dag::<u32, usize, u32>::new();
    let mut node_names = HashMap::new();
    let mut name_nodes = HashMap::new();

    for line in io::BufReader::new(fs::File::open("demo.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        if let [outer, inner] = line.split(" contain ").collect::<Vec<&str>>().as_slice() {
            let outer = outer.strip_suffix(" bags").unwrap();
            let outer_id = find_or_add(outer, &mut dag, &mut node_names, &mut name_nodes);
            if *inner == "no other bags." {
                continue
            }
            for x in inner.split(", ") {
                let mat = re.captures(x).unwrap();
                let (count, name) = (mat[1].parse::<usize>().unwrap(), &mat[2]);
                let inner_id = find_or_add(name, &mut dag, &mut node_names, &mut name_nodes);
                dag.add_edge(
                    daggy::NodeIndex::new(outer_id),
                    daggy::NodeIndex::new(inner_id),
                    count,
                )
                .unwrap();
                
            }
        }
    }
    graphviz(&dag, &name_nodes);
}

fn graphviz(dag: &daggy::Dag<u32, usize, u32>, nodes: &HashMap<usize,String>){
    println!("digraph G {{");
    for e in 0..dag.edge_count(){
        let e = daggy::EdgeIndex::new(e);
        let w = dag.edge_weight(e).unwrap();
        let (a,b) = dag.edge_endpoints(e).unwrap();
        println!(r#"   "{}"-> "{}" [ label = "{}"] "#, nodes.get(&a.index()).unwrap(), nodes.get(&b.index()).unwrap(), w);
    }
    println!("}}");
}

fn find_or_add(
    name: &str,
    dag: &mut daggy::Dag<u32, usize, u32>,
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
