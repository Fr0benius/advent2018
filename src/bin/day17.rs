use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("../../input/day17.txt");
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(.)=(.*), (.)=(.*)\.\.(.*)"#).unwrap();
    }
    let blocked = input
        .lines()
        .map(|line| RE.captures(line).unwrap())
        .flat_map(|cap| {
            dbg!(&cap[2], &cap[4], &cap[5]);
            let p = cap[2].parse().unwrap();
            let l = cap[4].parse().unwrap();
            let r = cap[5].parse().unwrap();
            let flipped = &cap[1] == "y";
            (l..=r).map(move |z| if flipped { (z, p) } else { (p, z) })
        })
        .collect();
    let mut ground = Ground::new(blocked);
    ground.dfs(500, 0, 0);
    println!("Part 1: {}", ground.seen.len());
    println!("Part 2: {}", ground.seen.intersection(&ground.blocked).count());
}

struct Ground {
    blocked: HashSet<(i32, i32)>,
    seen: HashSet<(i32, i32)>,
    min_y: i32,
    max_y: i32,
}

impl Ground {
    fn new(blocked: HashSet<(i32, i32)>) -> Self {
        Self {
            min_y: blocked.iter().map(|&(_, y)| y).min().unwrap(),
            max_y: blocked.iter().map(|&(_, y)| y).max().unwrap(),
            blocked,
            seen: HashSet::new(),
        }
    }

    fn dfs(&mut self, x: i32, y: i32, dir: i32) -> bool {
        if self.blocked.contains(&(x, y)) {
            return true;
        }
        if y > self.max_y {
            return false;
        }
        if y.clamp(self.min_y, self.max_y) == y {
            self.seen.insert((x, y));
        }
        if !self.dfs(x, y + 1, 0) {
            return false;
        }
        if dir != 0 {
            return self.dfs(x + dir, y, dir);
        }
        let l = self.dfs(x - 1, y, -1);
        let r = self.dfs(x + 1, y, 1);
        if l && r {
            for i in 0.. {
                if !self.blocked.insert((x - i, y)) {
                    break;
                }
            }
            for i in 1.. {
                if !self.blocked.insert((x + i, y)) {
                    break;
                }
            }
            true
        } else {
            false
        }
    }
}
