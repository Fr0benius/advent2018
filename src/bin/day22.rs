use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

fn main() {
    let input = include_str!("../../input/day22.txt");
    let (dep, tx, ty) = {
        let mut lines = input.lines();
        let dep: i64 = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut it = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .split(",");
        let tx: usize = it.next().unwrap().parse().unwrap();
        let ty: usize = it.next().unwrap().parse().unwrap();
        (dep, tx, ty)
        // (510, 10, 10)
    };
    let mx = tx * 8;
    let my = ty * 8;
    let mut geo = vec![vec![0; my + 1]; mx + 1];
    for x in 1..=mx {
        geo[x][0] = x as i64 * 16807;
    }
    for y in 1..=my {
        geo[0][y] = y as i64 * 48271;
    }
    let erosion = |z| (z + dep) % 20183;
    for x in 1..=mx {
        for y in 1..=my {
            if (x, y) == (tx, ty) {
                continue;
            }
            geo[x][y] = erosion(geo[x-1][y]) * erosion(geo[x][y-1]);
        }
    }
    let mut typ = vec![vec![0; my + 1]; mx + 1];
    for x in 0..=mx {
        for y in 0..=my {
            typ[x][y] = erosion(geo[x][y]) % 3;
        }
    }
    let part1: i64 = {
        let mut res = 0;
        for x in 0..=tx {
            for y in 0..=ty {
                res += typ[x][y];
            }
        }
        res
    };
    println!("Part 1: {}", part1);
    let mut q = BinaryHeap::new();
    let mut dist = HashMap::new();
    q.push(Reverse((0, (0, 0, 1))));
    dist.insert((0, 0, 1), 0);
    while let Some(Reverse((d0, (x0, y0, c0)))) = q.pop() {
        if d0 > dist[&(x0, y0, c0)] {
            continue;
        }
        if (x0, y0, c0) == (tx, ty, 1) {
            println!("Part 1: {}", d0);
            break;
        }
        for (x, y) in neighbors(x0, y0, mx, my) {
            if typ[x][y] != c0 {
                let entry = dist.entry((x, y, c0)).or_insert(i64::MAX);
                if *entry > d0 + 1 {
                    *entry = d0 + 1;
                    q.push(Reverse((d0 + 1, (x, y, c0))));
                }
            }
        }
        let c = 3 - c0 - typ[x0][y0];
        let entry = dist.entry((x0, y0, c)).or_insert(i64::MAX);
        if *entry > d0 + 7 {
            *entry = d0 + 7;
            q.push(Reverse((d0 + 7, (x0, y0, c))));
        }
    }

}

fn neighbors(x: usize, y: usize, mx: usize, my: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if x > 0 {
        res.push((x-1, y));
    }
    if x < mx {
        res.push((x+1, y));
    }
    if y > 0 {
        res.push((x, y-1));
    }
    if y < my {
        res.push((x, y+1));
    }
    res
}
