use std::cmp::Reverse;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("../../input/day24.txt");
    // let input = include_str!("../../input/sample24.txt");
    let mut batt = Vec::new();
    {
        let mut side = -1;
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if line.as_bytes().last() == Some(&b':') {
                side += 1;
                continue;
            }
            batt.push(Battalion::parse(line, side));
        }
    }
    println!("Part 1: {}", simulate(&batt, 117).1);
    let mut best_score = 0;
    {
        let mut l = 0;
        let mut r = 10000;
        while l < r {
            let mid = (l + r) / 2;
            dbg!(mid);
            let (side, score) = simulate(&batt, mid);
            if side == 0 {
                best_score = score;
                r = mid;
            } else {
                l = mid + 1;
            }
            dbg!(l, r);
        }
    }
    println!("Part 2: {}", best_score);
}

fn simulate(orig_batt: &[Battalion], boost: i64) -> (i64, i64) {
    let mut batt: Vec<_> = orig_batt
        .iter()
        .map(|b| {
            let mut b = b.clone();
            if b.side == 0 {
                b.damage += boost;
            }
            b
        }).collect();

    let n = batt.len();
    loop {
        assert!(batt.iter().all(|b| b.count >= 0));
        let mut alive: Vec<_> = (0..n).filter(|&i| batt[i].count > 0).collect();
        {
            let side = batt[alive[0]].side;
            if alive.iter().all(|&i| batt[i].side == side) {
                return (side, batt.iter().map(|b| b.count).sum::<i64>());
            }
        }

        let mut targeted = vec![false; n];
        alive.sort_unstable_by_key(|&i| Reverse(((batt[i].power()), batt[i].initiative)));
        let mut attacks = vec![];
        for &i in &alive {
            if let Some(&j) = alive
                .iter()
                .filter(|&&k| batt[i].side != batt[k].side && !targeted[k]
                        && batt[i].attack_value(&batt[k]) > 0)
                .max_by_key(|&&k| {
                    (
                        batt[i].attack_value(&batt[k]),
                        batt[k].power(),
                        batt[k].initiative,
                    )
                })
            {
                attacks.push((i, j));
                targeted[j] = true;
            }
        }
        assert!(!attacks.is_empty());
        assert!(attacks.iter().any(|&(i, j)| batt[i].attack_value(&batt[j]) > 0));
        // Attack is not enough to kill any enemies
        if attacks.iter().all(|&(i, j)| batt[i].attack_value(&batt[j]) < batt[j].hp) {
            return (-1, -1);
        }
    
        for &(i, j) in &attacks {
            dbg!(i, j, batt[i].attack_value(&batt[j]));
        }
        attacks.sort_unstable_by_key(|&(i, _)| Reverse(batt[i].initiative));
        for (i, j) in attacks {
            if batt[i].count == 0 {
                continue;
            }
            assert!(batt[j].count > 0);
            batt[j].count = (batt[j].count - batt[i].attack_value(&batt[j]) / batt[j].hp).max(0);
        }
    }
}

#[derive(Clone, Debug)]
struct Battalion {
    side: i64,
    count: i64,
    hp: i64,
    weak_to: Vec<String>,
    immune_to: Vec<String>,
    damage: i64,
    damage_type: String,
    initiative: i64,
}

impl Battalion {
    fn parse(s: &str, side: i64) -> Self {
        lazy_static! {
            static ref LINE: Regex = Regex::new(r#"(.*) units each with (.*) hit points(?: \((.*)\))? with an attack that does (.*) (.*) damage at initiative (.*)"#).unwrap();
            static ref WI: Regex = Regex::new(r#"(weak|immune) to ([^;]*)"#).unwrap();
        }
        let cap = LINE.captures(s).unwrap();
        let count: i64 = cap[1].parse().unwrap();
        let hp: i64 = cap[2].parse().unwrap();
        let damage: i64 = cap[4].parse().unwrap();
        let damage_type: String = cap[5].into();
        let initiative: i64 = cap[6].parse().unwrap();
        let mut weak_to = Vec::new();
        let mut immune_to = Vec::new();
        if let Some(mat) = cap.get(3) {
            for c in WI.captures_iter(mat.as_str()) {
                if &c[1] == "weak" {
                    weak_to = c[2].split(", ").map(String::from).collect();
                } else if &c[1] == "immune" {
                    immune_to = c[2].split(", ").map(String::from).collect();
                }
            }
        }
        Battalion {
            side,
            count,
            hp,
            weak_to,
            immune_to,
            damage,
            damage_type,
            initiative,
        }
    }
    fn power(&self) -> i64 {
        self.damage * self.count
    }
    fn attack_value(&self, other: &Self) -> i64 {
        if other.immune_to.contains(&self.damage_type) {
            0
        } else if other.weak_to.contains(&self.damage_type) {
            self.power() * 2
        } else {
            self.power()
        }
    }
}
