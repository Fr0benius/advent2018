use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;
use Op::*;

fn main() {
    let input = include_str!("../../input/day16.txt");
    let (before, testp, after, prog) = parse_input(input);
    let mut part1 = 0;
    for ((bef, test), aft) in before.iter().zip(&testp).zip(&after) {
        if OPS
            .into_iter()
            .filter(|&op| apply(op, test, bef) == *aft)
            .count()
            >= 3
        {
            part1 += 1;
        }
    }
    println!("Part 1: {}", part1);

    let mut allowed_ops: Vec<Vec<Op>> = vec![OPS.into_iter().collect(); OPS.len()];
    for ((bef, test), aft) in before.iter().zip(&testp).zip(&after) {
        let idx = test[0] as usize;
        allowed_ops[idx] = allowed_ops[idx]
            .iter()
            .filter(|&&op| apply(op, test, bef) == *aft)
            .cloned()
            .collect();
    }
    dbg!(&allowed_ops);
    let matching = find_matching(&allowed_ops);
    let mut reg = vec![0; 4];
    for inst in prog {
        reg = apply(matching[inst[0] as usize], &inst, &reg);
    }
    println!("Part 2: {}", reg[0]);
}

fn find_matching(allowed_ops: &Vec<Vec<Op>>) -> Vec<Op> {
    let n = allowed_ops.len();
    let mut ltor = HashMap::new();
    let mut rtol = HashMap::new();
    let mut seen = HashSet::new();
    fn dfs(
        k: usize,
        allowed_ops: &Vec<Vec<Op>>,
        seen: &mut HashSet<usize>,
        ltor: &mut HashMap<usize, Op>,
        rtol: &mut HashMap<Op, usize>,
    ) -> bool {
        if seen.contains(&k) {
            return false;
        }
        seen.insert(k);
        for &o in &allowed_ops[k] {
            if !rtol.contains_key(&o) {
                rtol.insert(o, k);
                ltor.insert(k, o);
                return true;
            }
        }
        for &o in &allowed_ops[k] {
            if dfs(rtol[&o], allowed_ops, seen, ltor, rtol) {
                rtol.insert(o, k);
                ltor.insert(k, o);
                return true;
            }
        }
        false
    }
    let mut done = false;
    while !done {
        done = true;
        seen.clear();
        for i in 0..n {
            if !ltor.contains_key(&i) && dfs(i, &allowed_ops, &mut seen, &mut ltor, &mut rtol) {
                done = false;
            }
        }
    }

    (0..n).map(|i| ltor[&i]).collect()
}

fn apply(op: Op, inst: &Vec<i32>, reg: &Vec<i32>) -> Vec<i32> {
    let a = match op {
        Seti | Gtir | Eqir => inst[1],
        _ => reg[inst[1] as usize],
    };
    let b = match op {
        Addi | Muli | Bani | Bori | Setr | Seti | Gtri | Eqri => inst[2],
        _ => reg[inst[2] as usize],
    };
    let mut res = reg.clone();
    let c = inst[3] as usize;
    match op {
        Addr | Addi => res[c] = a + b,
        Mulr | Muli => res[c] = a * b,
        Banr | Bani => res[c] = a & b,
        Borr | Bori => res[c] = a | b,
        Setr | Seti => res[c] = a,
        Gtir | Gtrr | Gtri => res[c] = (a > b) as i32,
        Eqir | Eqrr | Eqri => res[c] = (a == b) as i32,
    }
    res
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtrr,
    Gtri,
    Eqir,
    Eqrr,
    Eqri,
}
const OPS: [Op; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtrr, Gtri, Eqir, Eqrr, Eqri,
];

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut before = vec![];
    let mut testp = vec![];
    let mut after = vec![];
    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        before.push(parse_state(line));
        testp.push(parse_instr(lines.next().unwrap()));
        after.push(parse_state(lines.next().unwrap()));
        _ = lines.next();
    }

    let prog: Vec<Vec<i32>> = lines.skip(1).map(parse_instr).collect();
    (before, testp, after, prog)
}

fn parse_state(line: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#".*\[(.*)\]"#).unwrap();
    }

    let cap = RE.captures(line).unwrap();
    cap[1].split(", ").map(|s| s.parse().unwrap()).collect()
}

fn parse_instr(line: &str) -> Vec<i32> {
    line.split(" ").map(|s| s.parse().unwrap()).collect()
}
