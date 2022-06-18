use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BinaryHeap;

fn parse_line(line: &str) -> Option<(usize, usize)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"Step (.) must be finished before step (.) can begin."#).unwrap();
    }

    let cap = RE.captures(line)?;
    let a = cap[1].bytes().next()? - b'A';
    let b = cap[2].bytes().next()? - b'A';
    Some((a as usize, b as usize))
}

fn main() {
    let input = include_str!("../../input/day7.txt");
    // let input = include_str!("../../input/sample7.txt");
    let edges: Vec<(usize, usize)> = input.lines().map(|s| parse_line(s).unwrap()).collect();
    let n = edges.iter().map(|&(a, b)| a.max(b)).max().unwrap() + 1;
    dbg!(n);
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut ideg: Vec<i32> = vec![0; n];
    for (u, v) in edges {
        g[u].push(v);
        ideg[v] += 1;
    }
    let mut ord = vec![];
    {
        let mut ideg = ideg.clone();
        let mut q: BinaryHeap<i64> = Default::default();
        for v in 0..n {
            if ideg[v] == 0 {
                q.push(-(v as i64));
            }
        }
        while let Some(k) = q.pop() {
            let v = (-k) as usize;
            ord.push(v as usize);
            for &w in &g[v] {
                ideg[w] -= 1;
                if ideg[w] == 0 {
                    q.push(-(w as i64));
                }
            }
        }
    }
    let part1: String = ord.iter().map(|&b| ((b as u8) + b'A') as char).collect();
    println!("Part 1: {}", part1);

    const NWORKERS: usize = 5;
    const NONE: usize = usize::MAX;
    let mut workers: BinaryHeap<(i64, usize)> = [(0, NONE); 1].into_iter().collect();
    let mut pending: BinaryHeap<i64> = Default::default();
    for v in 0..n {
        if ideg[v] == 0 {
            pending.push(-(v as i64));
        }
    }
    let mut free_time = 0;
    while let Some((t0, v)) = workers.pop() {
        let t0 = -t0;
        free_time = free_time.max(t0);
        if v != NONE {
            for &w in &g[v] {
                ideg[w] -= 1;
                if ideg[w] == 0 {
                    pending.push(-(w as i64));
                }
            }
        }
        while workers.len() < NWORKERS {
            if let Some(x) = pending.pop() {
                let v = (-x) as usize;
                workers.push((-(t0 + (v as i64) + 61), v));
            } else {
                break;
            }
        }
    }

    println!("Part 2: {}", free_time);
}
