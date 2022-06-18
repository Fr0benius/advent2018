use std::mem::swap;

type Pt = [i64; 4];

fn dist(a: Pt, b: Pt) -> i64 {
    a.iter().zip(b.iter()).map(|(r, s)| (r-s).abs()).sum()
}

fn collect_pt<T: Iterator<Item = i64>>(mut it: T) -> Pt {
    [
        it.next().unwrap(),
        it.next().unwrap(),
        it.next().unwrap(),
        it.next().unwrap(),
    ]
}

fn main() {
    let input = include_str!("../../input/day25.txt");
    let pts: Vec<_> = input
        .lines()
        .map(|line| collect_pt(line.split(',').map(|w| w.parse().unwrap())))
        .collect();
    let n = pts.len();
    let mut dsu = DSU::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if dist(pts[i], pts[j]) <= 3 {
                dsu.join(i, j);
            }
        }
    }
    println!("Part 1: {}", dsu.comps);
}

struct DSU {
    par: Vec<usize>,
    size: Vec<i32>,
    comps: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            size: vec![1; n],
            comps: n,
        }
    }
    fn root(&mut self, a: usize) -> usize {
        if a != self.par[a] {
            self.par[a] = self.root(self.par[a]);
        }
        self.par[a]
    }
    fn join(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.root(a);
        b = self.root(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            swap(&mut a, &mut b);
        }
        self.size[a] += self.size[b];
        self.par[b] = a;
        self.comps -= 1;
        true
    }
}
