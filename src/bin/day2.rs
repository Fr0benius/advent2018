use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day2.txt");
    let words: Vec<_> = input.lines().collect();
    fn count(s: &str) -> HashMap<char, i64> {
        let mut mp: HashMap<char, i64> = HashMap::new();
        s.chars().for_each(|c| {
            *(mp.entry(c).or_insert(0)) += 1;
        });
        mp
    }

    let (twos, threes) = words
        .iter()
        .map(|&w| count(w))
        .map(|mp| (mp.values().any(|&n| n == 2), mp.values().any(|&n| n == 3)))
        .fold((0, 0), |(a, b), (c, d)| (a + c as i64, b + d as i64));
    println!("Part 1: {}", twos * threes);

    let bwords: Vec<&[u8]> = words.iter().map(|&s|s.as_bytes()).collect();
    for w1 in &bwords {
        for w2 in &bwords {
            let mut j = 0;
            let mut cnt = 0;
            for i in 0..w1.len() {
                if w1[i] != w2[i] {
                    j = i;
                    cnt += 1;
                }
            }
            if cnt == 1 {
                let mut arr = vec![];
                for i in 0..w1.len() {
                    if i != j {
                        arr.push(char::from(w1[i]));
                    }
                }
                println!("Part 2: {}", arr.iter().collect::<String>());
                return
            }
        }
    }
}
