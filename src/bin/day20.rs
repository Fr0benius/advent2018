use std::{cmp::max, collections::{HashMap, HashSet, VecDeque}};

use Dir::*;

fn main() {
    let input = include_str!("../../input/day20.txt");
    let mut g: Graph = vec![vec![]];
    construct(&mut input.chars(), &mut g, 0);
    dbg!(g.len());
    let mut b : Board = [((0, 0), vec![])].into();
    simulate(&mut b, &g, (0, 0), 0, &mut HashSet::new());
    let mut q = VecDeque::new();
    let mut dist = HashMap::new();
    q.push_back((0, 0));
    dist.insert((0, 0), 0);
    while let Some(pt) = q.pop_front() {
        if !b.contains_key(&pt) {
            continue;
        }
        let d = dist[&pt];
        for &nxt in &b[&pt] {
           if !dist.contains_key(&nxt) {
               dist.insert(nxt, d + 1);
               q.push_back(nxt);
           }
        }
    }
    println!("Part 1: {}", dist.values().max().unwrap());
    println!("Part 2: {}", dist.values().filter(|&&n| n >= 1000).count());
}

fn simulate(b: &mut Board, g: &Graph, cur: Pt, state: usize, memo: &mut HashSet<(Pt, usize)>) {
    if !memo.insert((cur, state)) {
        return;
    }
    for &(w, dir) in &g[state] {
        if let Some(d) = dir {
            let nxt = step(cur, d);
            let neigh = b.entry(cur).or_default();
            if !neigh.contains(&nxt) {
                neigh.push(nxt);
            }
            let neigh = b.entry(nxt).or_default();
            if !neigh.contains(&cur) {
                neigh.push(cur);
            }
            simulate(b, g, nxt, w, memo);
        } else {
            simulate(b, g, cur, w, memo);
        }
    }
}

fn construct<T: Iterator<Item=char>> (it: &mut T, g: &mut Graph, start: usize) -> usize {
    let mut cur = start;
    let mut ends = vec![];
    while let Some(c) = it.next() {
        if let Some(dir) = ch_to_dir(c) {
            let nxt = new_node(g);
            add(g, cur, nxt, Some(dir));
            cur = nxt;
        } else if c == '(' {
            cur = construct(it, g, cur);
        } else if c == '|' {
            ends.push(cur);
            cur = start;
        } else if c == ')' {
            ends.push(cur);
            let nxt = new_node(g);
            for v in ends {
                add(g, v, nxt, None);
            }
            return nxt;
        }
    }
    return cur;
}

fn ch_to_dir(c: char) -> Option<Dir> {
    match c {
        'N' => Some(U),
        'E' => Some(R),
        'S' => Some(D),
        'W' => Some(L),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Dir {
    U,
    R,
    D,
    L,
}

type Graph = Vec<Vec<(usize, Option<Dir>)>>;

fn new_node(g: &mut Graph) -> usize {
    g.push(vec![]);
    return g.len() - 1;
}
fn add(g: &mut Graph, v: usize, w: usize, d: Option<Dir>) {
    g[v].push((w, d));
}

type Pt = (i32, i32);
fn step((x, y): Pt, d: Dir) -> Pt {
    match d {
        U => (x, y + 1),
        R => (x + 1, y),
        D => (x, y - 1),
        L => (x - 1, y),
    }
}
type Board = HashMap<Pt, Vec<Pt>>;
