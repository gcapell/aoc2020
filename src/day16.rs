use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

#[derive(Debug)]
struct Pos {
    p: usize,
    rules: HashSet<usize>,
}

pub fn run() {
    let ranges = [
        ((48, 793), (800, 971)),
        ((36, 235), (247, 974)),
        ((25, 850), (862, 964)),
        ((40, 173), (193, 959)),
        ((42, 895), (902, 955)),
        ((43, 692), (715, 951)),
        ((38, 528), (549, 967)),
        ((43, 133), (141, 963)),
        ((40, 651), (675, 951)),
        ((48, 801), (811, 973)),
        ((50, 562), (587, 955)),
        ((43, 520), (527, 968)),
        ((44, 745), (752, 950)),
        ((41, 929), (941, 963)),
        ((37, 828), (838, 958)),
        ((47, 475), (491, 972)),
        ((38, 198), (210, 965)),
        ((33, 66), (74, 949)),
        ((35, 492), (507, 962)),
        ((35, 358), (381, 965)),
    ];

    let mut possible = [[true; 20]; 20];

    for line in io::BufReader::new(fs::File::open("tickets.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        let nums: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        if invalid(&nums, &ranges) {
            continue;
        }
        for (ticket_pos, num) in nums.iter().enumerate() {
            for (rule_pos, ((lo1, hi1), (lo2, hi2))) in ranges.iter().enumerate() {
                if possible[ticket_pos][rule_pos] {
                    if !((num >= lo1 && num <= hi1) || (num >= lo2 && num <= hi2)) {
                        possible[ticket_pos][rule_pos] = false;
                    }
                }
            }
        }
    }
    
    let mut tickets = Vec::new();
    for (ticket, r) in possible.iter().enumerate() {
        let mut rules = HashSet::new();
        for (rule,valid) in r.iter().enumerate() {
            if *valid {
                rules.insert(rule);
            }
        }
        let pos = Pos{p:ticket, rules:rules};
        //println!("{:?}", pos);
        tickets.push(pos);
     }

     tickets.sort_by_key(|x|x.rules.len());
     tickets.reverse();

     while let Some(t) = tickets.pop(){
         assert!(t.rules.len() == 1);
         let r = t.rules.iter().next().unwrap();
         println!("t:{}, rule:{}", t.p, r);
         for  t2 in &mut tickets {
             t2.rules.remove(r);
         }
     }

}

fn invalid(nums: &[i32], ranges: &[((i32, i32), (i32, i32))]) -> bool {
    'num_loop: for n in nums {
        for ((lo1, hi1), (lo2, hi2)) in ranges {
            if n >= lo1 && n <= hi1 || n >= lo2 && n <= hi2 {
                continue 'num_loop;
            }
        }
        return true;
    }
    false
}
