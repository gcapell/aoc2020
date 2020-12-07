use daggy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn dot() {
    let re = Regex::new(r"(\d+) (.*) bags?\.?").unwrap();
    println!("digraph G {{");
    let mut dag = daggy::Dag::<u32, usize, u32>::new();
    let mut node_names = HashMap::new();

    for line in io::BufReader::new(fs::File::open("demo.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        if let [outer, inner] = line.split(" contain ").collect::<Vec<&str>>().as_slice() {
            let outer = outer.strip_suffix(" bags").unwrap();
            let outer_id = find_or_add(outer, &mut dag, &mut node_names);
            for x in inner.split(", ") {
                if x == "no other bags." {
                    continue;
                }

                let mat = re.captures(x).unwrap();
                let (count, name) = (&mat[1], &mat[2]);
                let count = count.parse::<usize>().unwrap();
                let inner_id = find_or_add(name, &mut dag, &mut node_names);
                dag.add_edge(
                    daggy::NodeIndex::new(outer_id),
                    daggy::NodeIndex::new(inner_id),
                    count,
                )
                .unwrap();

                println!(r#"   "{}"-> "{}" [ label = "{}"] "#, outer, name, count);
            }
        }
    }
    println!("}}");
    println!("names: {:?}", node_names);
    println!("names: {:?}", dag);
}

fn find_or_add(
    name: &str,
    dag: &mut daggy::Dag<u32, usize, u32>,
    names: &mut HashMap<String, usize>,
) -> usize {
    if let Some(o) = names.get(name) {
        *o
    } else {
        let i = dag.add_node(0).index();
        names.insert(name.to_string(), i);
        i
    }
}
