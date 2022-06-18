use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/day1.txt");
    let nums: Vec<i64> = input
        .lines()
        .map(|s| s.parse().expect("failed parse"))
        .collect();
    let s: i64 = nums.iter().sum();
    println!("Part 1: {}", s);
    let mut set: HashSet<i64> = HashSet::new();
    let mut acc = 0i64;
    set.insert(0);
    for &x in nums.iter().cycle() {
        acc += x;
        if set.contains(&acc) {
            break;
        }
        set.insert(acc);
    }
    println!("Part 2: {}", acc);
}
