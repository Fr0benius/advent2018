use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

struct Pots {
    offset: i64,
    stable: bool,
    state: VecDeque<char>,
    rules: HashMap<String, char>,
}

impl Pots {
    fn from_input(input: &'static str) -> Self {
        lazy_static! {
            static ref HEAD: Regex = Regex::new(r#"initial state: (.*)"#).unwrap();
            static ref LINE: Regex = Regex::new(r#"(.{5}) => (.)"#).unwrap();
        }
        let mut lines = input.lines();
        let state = HEAD.captures(lines.next().unwrap()).unwrap()[1]
            .chars()
            .collect();
        let rules = lines
            .skip(1)
            .map(|line| {
                let cap = LINE.captures(line).unwrap();
                let k = cap[1].into();
                let v = cap[2].chars().next().unwrap();
                (k, v)
            })
            .collect();
        Pots {
            offset: 0,
            state,
            rules,
            stable: false,
        }
    }

    fn normalize(&mut self) {
        while self.state.front() == Some(&'.') {
            self.state.pop_front();
            self.offset += 1;
        }
        while self.state.back() == Some(&'.') {
            self.state.pop_back();
        }
        for _ in 0..4 {
            self.state.push_front('.');
            self.state.push_back('.');
            self.offset -= 1;
        }
    }

    fn step(&mut self) {
        self.normalize();
        let old_state = self.state.clone();
        self.state = (0..self.state.len() - 4)
            .map(|k| {
                *self
                    .rules
                    .get(&(k..k + 5).map(|i| self.state[i]).collect::<String>())
                    .unwrap_or(&'.')
            })
            .collect();
        self.offset += 2;
        self.normalize();
        if self.state == old_state {
            self.stable = true;
        }
    }

    // Running some experiments showed that the state stabilizes at
    // "...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###...###..###..###."
    // moving to the right by 1 with every step.
    fn step_n(&mut self, mut n: i64) {
        while n > 0 && !self.stable {
            self.step();
            n -= 1;
        }
        self.offset += n;
    }

    fn eval(&self) -> i64 {
        self.state
            .iter()
            .enumerate()
            .map(|(i, &k)| (self.offset + i as i64) * (k == '#') as i64)
            .sum::<i64>()
    }
}

fn main() {
    let input = include_str!("../../input/day12.txt");
    let mut pots = Pots::from_input(input);
    pots.step_n(20);
    println!("Part 1: {}", pots.eval());
    pots.step_n(50000000000 - 20);
    println!("Part 2: {}", pots.eval());
}
