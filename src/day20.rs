use std::fs;
use std::fmt;
use std::cmp::min;
use std::collections::HashMap;

pub fn run() {
    let tile_str = fs::read_to_string("input.txt").unwrap();

    let tiles:Vec<Tile> = tile_str.trim().split("\n\n").map(Tile::new).collect();
    let mut matchings: HashMap<i32,Matching> = HashMap::new();
    for t in &tiles{
        println!("{}",t);
        for s in t.sides.iter(){
            let m = min(s.cw, s.acw);
            matchings.entry(m).or_insert_with(Matching::new).tiles.push(t.id)
        }
    }
    let mut edges: HashMap<usize, Vec<u32>> = HashMap::new();
    for t in &tiles {
        let n = t.sides.iter().filter(|s| matchings[&min(s.cw, s.acw)].tiles.len()>1 ).count();
        edges.entry(n).or_insert_with(Vec::new).push(t.id);
        println!("{}->{}", t.id, n);
    }
    println!("{:?}", edges);
    let product = edges[&2].iter().fold(1i64, |acc,n| acc* *n as i64);  
    println!("product {}", product);
}

#[derive(Debug)]
struct Matching {
    tiles: Vec<u32>,
}
impl Matching{
    fn new()->Matching{
        Matching{tiles:Vec::new()}
    }
}


#[derive(Debug)]
struct Side {
    cw:i32,
    acw:i32,
}

impl Side {
    fn swap(self) -> Side {
        Side{cw:self.acw, acw:self.cw}
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:010b}", self.cw)
    }
}

fn parse_side(s: &str) -> Side {
     let bits = s.chars().map(|c| if c=='#'{1}else{0}).collect::<Vec<i32>>();
    let a = bits.iter().fold(0,|acc,n|acc*2+n);
    let b = bits.iter().rev().fold(0,|acc,n|acc*2+n);
    // println!("parse_side({})-> {:010b},{:010b}", s, a, b);
   
    Side{cw:a, acw:b}
}

struct Tile {
    id: u32,
    sides: [Side; 4],
}

impl fmt::Display for Tile{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile: {}, ", self.id)?;
        for s in  self.sides.iter(){
            write!(f, "{},",s)?;
        }
        Ok(())
    }
}

impl Tile {
    fn new(s: &str) -> Tile {
        let lines = s.lines().collect::<Vec<&str>>();

        let west = lines[1..]
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect::<String>();
        let east = lines[1..]
            .iter()
            .map(|x| x.chars().last().unwrap())
            .collect::<String>();

        return Tile {
            id: lines[0][5..9].parse::<u32>().unwrap(),
            sides: [
                parse_side(&lines[1]),
                parse_side(&east),
                parse_side(&lines[lines.len() - 1]).swap(),
                parse_side(&west).swap(),
            ],
        };
    }
}
