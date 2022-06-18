use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(.*) players; last marble is worth (.*) points"#).unwrap();
}

fn main() {
    let input = include_str!("../../input/day9.txt");
    let (elves, last_marble) = {
        let cap = RE.captures(input).unwrap();
        let a: usize = cap[1].parse().unwrap();
        let b: i64 = cap[2].parse().unwrap();
        (a, b)
    };
    let mut arr = VecDeque::from([0]);
    fn rot<T>(arr: &mut VecDeque<T>, mut k: i64) {
        while k > 0 {
            let fst = arr.pop_front().unwrap();
            arr.push_back(fst);
            k -= 1;
        }
        while k < 0 {
            let lst = arr.pop_back().unwrap();
            arr.push_front(lst);
            k += 1;
        }
    }
    let mut score = vec![0; elves];
    let mut elf = 0;
    for marble in 1..=last_marble*100 {
        if marble % 23 == 0 {
            score[elf] += marble;
            rot(&mut arr, -7);
            score[elf] += arr[0];
            arr.pop_front();
        } else {
            rot(&mut arr, 2);
            arr.push_front(marble);
        }
        if marble == last_marble {
            println!("Part 1: {}", score.iter().max().unwrap());
        }
        elf += 1;
        if elf == elves {
            elf = 0;
        }
    }
    println!("Part 2: {}", score.iter().max().unwrap());
}
