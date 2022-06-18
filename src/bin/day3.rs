use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$"#).unwrap();
}
fn main() {
    let input = include_str!("../../input/day3.txt");
    let mut claims = [[0; 1001]; 1001];

    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();
    let mut dx: Vec<i32> = Vec::new();
    let mut dy: Vec<i32> = Vec::new();
    for line in input.lines() {
        let cap = RE.captures(line.trim()).expect("failed parse");
        x.push(cap[1].parse().unwrap());
        y.push(cap[2].parse().unwrap());
        dx.push(cap[3].parse().unwrap());
        dy.push(cap[4].parse().unwrap());
    }
    for i in 0..x.len() {
        for j in x[i]..x[i] + dx[i] {
            for k in y[i]..y[i] + dy[i] {
                claims[j as usize][k as usize] += 1;
            }
        }
    }
    let mut part1 = 0;
    for arr in claims {
        for k in arr {
            if k > 1 {
                part1 += 1;
            }
        }
    }
    println!("Part 1: {}", part1);
    for i in 0..x.len() {
        if (|| {
            for j in x[i]..x[i] + dx[i] {
                for k in y[i]..y[i] + dy[i] {
                    if claims[j as usize][k as usize] != 1 {
                        return false;
                    }
                }
            }
            return true;
        })() {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}
