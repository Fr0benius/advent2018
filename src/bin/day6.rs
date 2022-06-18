use std::collections::{HashMap, VecDeque};

fn main() {
    type Pt = (i64, i64);
    let input = include_str!("../../input/day6.txt");
    let pts: Vec<Pt> = input
        .lines()
        .map(|s| {
            let mut nums = s.split(", ");
            let a: i64 = nums.next().unwrap().parse().unwrap();
            let b: i64 = nums.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();
    let mut d: HashMap<Pt, (i64, usize)> = HashMap::new();
    let mut q: VecDeque<Pt> = VecDeque::new();
    for (i, &pt) in pts.iter().enumerate() {
        d.insert(pt, (0, i));
        q.push_back(pt);
    }
    let minx = pts.iter().map(|pt| pt.0).min().unwrap();
    let maxx = pts.iter().map(|pt| pt.0).max().unwrap();
    let miny = pts.iter().map(|pt| pt.1).min().unwrap();
    let maxy = pts.iter().map(|pt| pt.1).max().unwrap();
    let mut counts = vec![0; pts.len()];
    const BAD: usize = usize::MAX;
    while let Some((x0, y0)) = q.pop_front() {
        let &(d0, owner) = d.get(&(x0, y0)).unwrap();
        if x0.clamp(minx, maxx) != x0 || y0.clamp(miny, maxy) != y0 {
            if owner != BAD {
                counts[owner] = -1;
            }
            continue;
        }
        if owner != BAD && counts[owner] != -1 {
            counts[owner] += 1;
        }
        for (x, y) in [(x0 + 1, y0), (x0, y0 + 1), (x0 - 1, y0), (x0, y0 - 1)] {
            let (d1, prv) = d.entry((x, y)).or_insert((i64::MAX, owner));
            if *d1 == d0 + 1 && *prv != owner && *prv != BAD {
                if counts[*prv] > 0 {
                    counts[*prv] -= 1;
                }
                *prv = BAD;
            }
            if *d1 > d0 + 1 {
                *d1 = d0 + 1;
                q.push_back((x, y));
            }
        }
    }
    println!("Part 1: {}", counts.iter().max().unwrap());
    let mut part2 = 0;
    const D: i64 = 10000;
    let del = (D + pts.len() as i64 - 1) / pts.len() as i64;
    for x0 in minx - del..=maxx + del {
        for y0 in miny - del..=maxy + del {
            let sum: i64 = pts
                .iter()
                .map(|&(x, y)| (x - x0).abs() + (y - y0).abs())
                .sum();
            if sum < D {
                part2 += 1;
            }
        }
    }
    println!("Part 2: {}", part2);
}
