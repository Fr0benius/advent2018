use std::{fmt, collections::HashSet};

use Op::*;

fn main() {
    let input = include_str!("../../input/day21.txt");
    let (ip, prog) = parse_input(input);
    let mut i = 0;
    let mut reg = vec![0; 6];
    let mut vals = HashSet::new();
    let mut fst = -1;
    let mut lst = -1;
    while i < prog.len() {
        reg[ip] = i as _;
        let (op, inst) = &prog[i];
        if op == &Eqrr {
            if fst == -1 {
                fst = reg[3];
            }
            if !vals.insert(reg[3]) {
                break;
            }
            lst = reg[3];
        }
        reg = apply(*op, inst, &reg);
        i = reg[ip] as usize + 1;
    }
    println!("Part 1: {}", fst);
    println!("Part 2: {}", lst);
}

fn apply(op: Op, inst: &Vec<i64>, reg: &Vec<i64>) -> Vec<i64> {
    let a = match op {
        Seti | Gtir | Eqir => inst[0],
        _ => reg[inst[0] as usize],
    };
    let b = match op {
        Addi | Muli | Bani | Bori | Setr | Seti | Gtri | Eqri => inst[1],
        _ => reg[inst[1] as usize],
    };
    let mut res = reg.clone();
    let c = inst[2] as usize;
    match op {
        Addr | Addi => res[c] = a + b,
        Mulr | Muli => res[c] = a * b,
        Banr | Bani => res[c] = a & b,
        Borr | Bori => res[c] = a | b,
        Setr | Seti => res[c] = a,
        Gtir | Gtrr | Gtri => res[c] = (a > b) as i64,
        Eqir | Eqrr | Eqri => res[c] = (a == b) as i64,
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
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

const OPS: [Op; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtrr, Gtri, Eqir, Eqrr, Eqri,
];

fn str_to_op(s: &str) -> Option<Op> {
    for op in OPS {
        if op.to_string().to_lowercase() == s.to_lowercase() {
            return Some(op);
        }
    }
    dbg!(s);
    None
}

fn parse_input(input: &str) -> (usize, Vec<(Op, Vec<i64>)>) {
    let mut lines = input.lines();
    let ip = lines
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut prog: Vec<(Op, Vec<i64>)> = Vec::new();
    for line in lines {
        let words: Vec<_> = line.split(" ").collect();
        let op = str_to_op(words[0]).unwrap();
        let params = words[1..4].iter().map(|w| w.parse().unwrap()).collect();
        prog.push((op, params));
    }

    (ip, prog)
}
