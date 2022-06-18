use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref ENTRY: Regex = Regex::new(r#"\[(.*)\] (.*)"#).unwrap();
    static ref GUARD: Regex = Regex::new(r#"Guard #(\d+) begins shift"#).unwrap();
}

fn main() {
    let input = include_str!("../../input/day4.txt");
    let mut entries: Vec<_> = input.lines().collect();
    entries.sort();
    let mut sleep_map: HashMap<i32, [i32; 60]> = Default::default();
    let mut cur_guard = 0;
    for entry in entries {
        let cap = ENTRY.captures(entry).expect("failed parse");
        let timestamp = &cap[1];
        let min: usize = timestamp[timestamp.len() - 2..].parse().unwrap();
        let s = &cap[2];
        if let Some(gcap) = GUARD.captures(s) {
            cur_guard = gcap[1].parse().expect("failed parsing guard number");
        } else {
            if s == "falls asleep" {
                sleep_map.entry(cur_guard).or_insert([0; 60])[min] += 1;
            } else if s == "wakes up" {
                sleep_map.entry(cur_guard).or_insert([0; 60])[min] -= 1;
            } else {
                panic!("unknown guard action")
            }
        }
    }
    for (_, sleep) in &mut sleep_map {
        for i in 1..60 {
            sleep[i] += sleep[i - 1];
        }
    }
    let (_, laziest_guard) = sleep_map
        .iter()
        .map(|(g, sleep)| (sleep.iter().sum::<i32>(), g))
        .max()
        .unwrap();
    let minute = sleep_map[laziest_guard]
        .iter()
        .enumerate()
        .max_by_key(|(_, &k)| k)
        .unwrap()
        .0 as i32;
    println!("Part 1: {}", laziest_guard * minute);
    let (consistent_guard, minute, _) = sleep_map
        .iter()
        .map(|(guard, sleep)| sleep.iter().enumerate().map(|(m, k)| (*guard, m, k)))
        .flatten()
        .max_by_key(|(_, _, &k)| k)
        .unwrap();

    println!("Part 2: {}", consistent_guard * minute as i32);
}
