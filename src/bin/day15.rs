use std::collections::{HashMap, VecDeque};

const START_HP: i16 = 200;
const GOBLIN_ATTACK: i16 = 3;

fn main() {
    let input = include_str!("../../input/day15.txt");
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    println!("Part 1: {}", simulate(&grid, 3, false).unwrap());
    for elf_attack in 4.. {
        if let Some(score) = simulate(&grid, elf_attack, true) {
            println!("Part 2: {}", score);
            break;
        }
    }
}

fn simulate(orig_grid: &Vec<Vec<u8>>, elf_attack: i16, elves_must_win: bool) -> Option<i64> {
    let mut grid = orig_grid.clone();
    let n = grid.len();
    let m = grid[0].len();
    let mut dudes: Vec<Dude> = vec![];
    for r in 0..n {
        for c in 0..m {
            match grid[r][c] {
                b'G' | b'E' => dudes.push(Dude {
                    r,
                    c,
                    side: grid[r][c],
                    hp: START_HP,
                }),
                _ => {}
            }
        }
    }
    for round in 0.. {
        dudes.sort_unstable_by_key(|dude| (dude.r, dude.c));
        for i in 0..dudes.len() {
            if dudes[i].hp <= 0 {
                if elves_must_win && dudes[i].side == b'E' {
                    return None;
                }
                continue;
            }
            let (r0, c0) = (dudes[i].r, dudes[i].c);
            if try_attacking(r0, c0, &mut dudes, &mut grid, elf_attack) {
                continue;
            }
            if dudes
                .iter()
                .all(|dude| dude.hp <= 0 || dude.side == dudes[i].side)
            {
                if elves_must_win && dudes[i].side != b'E' {
                    return None;
                }
                return Some(
                    dudes
                        .iter()
                        .map(|dude| dude.hp as i64)
                        .filter(|&hp| hp > 0)
                        .sum::<i64>()
                        * round,
                );
            }
            let mut in_range: Vec<_> = dudes
                .iter()
                .filter(|dude| dude.side != dudes[i].side && dude.hp > 0)
                .flat_map(|dude| neighbors((dude.r, dude.c), n, m).into_iter())
                .filter(|&(r, c)| grid[r][c] == b'.')
                .collect();
            in_range.sort_unstable();
            in_range.dedup();
            let mut q: VecDeque<Pos> = Default::default();
            let mut dist: HashMap<Pos, (i64, Pos)> = Default::default();
            for pos in in_range {
                q.push_back(pos);
                dist.insert(pos, (0, pos));
            }
            while let Some(pos0) = q.pop_front() {
                let (d0, id) = dist[&pos0];
                if pos0 == (r0, c0) {
                    for pos in neighbors(pos0, n, m) {
                        if dist.get(&pos) == Some(&(d0 - 1, id)) {
                            grid[pos.0][pos.1] = dudes[i].side;
                            grid[r0][c0] = b'.';
                            let (r0, c0) = pos;
                            dudes[i].r = r0;
                            dudes[i].c = c0;
                            try_attacking(r0, c0, &mut dudes, &mut grid, elf_attack);
                            assert_eq!(dudes[i].side, grid[r0][c0]);
                            break;
                        }
                    }
                    break;
                }
                for pos @ (r, c) in neighbors(pos0, n, m) {
                    if grid[r][c] != b'.' && pos != (r0, c0) {
                        continue;
                    }
                    if !dist.contains_key(&pos) {
                        dist.insert(pos, (d0 + 1, id));
                        assert!(pos == (r0, c0) || grid[r][c] == b'.');
                        q.push_back(pos);
                    }
                }
            }
        }
    }
    None
}

fn try_attacking(
    r0: usize,
    c0: usize,
    dudes: &mut [Dude],
    grid: &mut Vec<Vec<u8>>,
    elf_attack: i16,
) -> bool {
    let enemy = if grid[r0][c0] == b'G' { b'E' } else { b'G' };
    let midx = neighbors((r0, c0), grid.len(), grid[0].len())
        .into_iter()
        .filter(|&(r, c)| grid[r][c] == enemy)
        .map(|(r, c)| {
            dudes
                .iter()
                .enumerate()
                .find(|(_, dude)| (dude.r, dude.c) == (r, c) && dude.hp > 0)
                .unwrap()
        })
        .min_by_key(|&(_, dude)| (dude.hp, dude.r, dude.c))
        .map(|(i, _)| i);
    if let Some(idx) = midx {
        let other = &mut dudes[idx];
        other.hp -= if grid[r0][c0] == b'E' {
            elf_attack
        } else {
            GOBLIN_ATTACK
        };
        if other.hp <= 0 {
            grid[other.r][other.c] = b'.';
        }
        return true;
    }
    false
}

#[derive(Clone, Debug)]
struct Dude {
    r: usize,
    c: usize,
    side: u8,
    hp: i16,
}

type Pos = (usize, usize);
fn neighbors((r, c): Pos, n: usize, m: usize) -> Vec<Pos> {
    return [(r - 1, c), (r, c - 1), (r, c + 1), (r + 1, c)]
        .into_iter()
        .filter(|&(a, b)| a.clamp(0, n - 1) == a && b.clamp(0, m - 1) == b)
        .collect();
}
