use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    // possible_foods maps from allergen to set of possible foods.
    let mut possible_foods: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut food_count: HashMap<&str, i32> = HashMap::new();
    let mut all_foods: HashSet<&str> = HashSet::new();
    let file = fs::read_to_string("input.txt").unwrap();
    for line in file.lines() {
        let mut chunks = line.split("(contains ");
        let foods = chunks.next().unwrap();
        let allergens = chunks.next().unwrap().strip_suffix(")").unwrap();
        let foods: HashSet<&str> = foods.split_ascii_whitespace().collect();
        //println!("food:{:?}, allergen:{:?}", foods, allergens);
        for f in foods.iter() {
            all_foods.insert(f);
            *food_count.entry(f).or_insert(0) += 1;
        }

        for allergen in allergens.split(", ") {
            if possible_foods.contains_key(allergen) {
                let mut i: HashSet<&str> = HashSet::new();
                for v in possible_foods[allergen].intersection(&foods) {
                    i.insert(v);
                }
                possible_foods.insert(allergen, i);
            } else {
                possible_foods.insert(allergen, foods.clone());
            }
        }
    }
    println!("all_food {:?}", all_foods);
    println!("possible_foods {:?}", possible_foods);

    let mut total = 0;
    for f in all_foods
        .iter()
        .filter(|x| !possible_foods.values().any(|s| s.contains(*x)))
    {
        let n = food_count[f];
        total += n;
        //println!("impossible {}, {}", f, n);
    }
    println!("total {}", total);
}
