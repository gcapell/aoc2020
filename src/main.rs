mod day2;

fn main() {
    println!(
        "count:{}",
        day2::count_matches(if true { day2::policy1 } else { day2::policy2 })
    );
}
