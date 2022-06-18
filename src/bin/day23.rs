use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"pos=<(.*),(.*),(.*)>, r=(.*)"#).unwrap();
}

type Pt = (i64, i64, i64);
fn main() {
    let input = include_str!("../../input/day23.txt");
    let mut bots: Vec<(Pt, i64)> = {
        let p = |s: &str| s.parse().unwrap();
        input
            .lines()
            .map(|line| {
                let cap = RE.captures(line).unwrap();
                ((p(&cap[1]), p(&cap[2]), p(&cap[3])), p(&cap[4]))
            })
            .collect()
    };
    bots.sort_unstable_by_key(|&(_, r)| -r);
    let part1 = bots
        .iter()
        .filter(|&&(p, _)| dist(p, bots[0].0) <= bots[0].1)
        .count();
    println!("Part 1: {}", part1);
    let part2 = [
        compute(&bots, |(x, y, z)| x + y + z),
        compute(&bots, |(x, y, z)| x + y - z),
        compute(&bots, |(x, y, z)| x - y + z),
        compute(&bots, |(x, y, z)| -x + y + z),
    ];
    println!("Part 2: {}", part2.iter().max().unwrap());
}

fn dist((x0, y0, z0): Pt, (x1, y1, z1): Pt) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}

fn compute<F: Fn(Pt) -> i64>(bots: &Vec<(Pt, i64)>, f: F) -> i64 {
    let mut events: Vec<_> = bots
        .iter()
        .flat_map(|&(pt, d)| [(f(pt) - d, 1), (f(pt) + d + 1, -1)].into_iter())
        .collect();
    events.sort_unstable();
    let max = events
        .iter()
        .scan(0, |acc, &(_, k)| {
            *acc += k;
            Some(*acc)
        })
        .max()
        .unwrap();
    let mut res = i64::MAX;
    {
        let mut prv = None;
        let mut acc = 0;
        for (k, v) in events {
            acc += v;
            if acc == max && v == 1 {
                prv = Some(k);
            } else if acc == max - 1 && v == -1 {
                let l = prv.unwrap();
                if l <= 0 && k - 1 >= 0 {
                    return 0;
                }
                if k - 1 < 0 {
                    res = res.min(1 - k);
                }
                if l > 0 {
                    res = res.min(l);
                }
                prv = None;
            }
        }
    }
    res
}
