use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r#"position=<\s*(.*),\s*(.*)> velocity=<\s*(.*),\s*(.*)>"#).unwrap();
}

fn main() {
    let input = include_str!("../../input/day10.txt");
    type Pt = (i64, i64);
    let data: Vec<(Pt, Pt)> = input
        .lines()
        .map(|line| {
            let cap = RE.captures(line.trim()).unwrap();
            let p = |s: &str| s.parse().unwrap();
            ((p(&cap[1]), p(&cap[2])), (p(&cap[3]), p(&cap[4])))
        })
        .collect();
    fn limits(pts: &Vec<Pt>) -> (i64, i64, i64, i64) {
        let xl = pts.iter().map(|&(x, _)|x).min().unwrap();
        let xr = pts.iter().map(|&(x, _)|x).max().unwrap();
        let yl = pts.iter().map(|&(_, y)|y).min().unwrap();
        let yr = pts.iter().map(|&(_, y)|y).max().unwrap();
        (xl, xr, yl, yr)
    }
    fn print(pts: &Vec<Pt>) {
        let (xl, xr, yl, yr) = limits(pts);
        let ptset: HashSet<Pt> = pts.iter().cloned().collect();
        for y in yl..=yr {
            for x in xl..=xr {
                print!("{}", if ptset.contains(&(x, y)) { '#' } else { '.' })
            }
            println!("");
        }
    }
    let vel: Vec<Pt> = data.iter().map(|&(_, v)| v).collect();
    let mut state: Vec<Pt> = data.iter().map(|&(p, _)| p).collect();
    for time in 0.. {
        let (xl0, xr0, yl0, yr0) = limits(&state);
        let new_state = state.iter().zip(vel.iter())
            .map(|(&(x, y), &(dx, dy))| (x + dx, y + dy)).collect();
        let (xl, xr, yl, yr) = limits(&new_state);
        if xr0-xl0 < xr-xl && yr0-yl0 < yr-yl {
            println!("Part 1:");
            print(&state);
            println!("Part 2: {}", time);
            break;
        }
        state = new_state;
    }
}
