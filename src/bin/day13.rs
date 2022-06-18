use std::collections::{HashSet, HashMap};

use Dir::*;

fn main() {
    let input = include_str!("../../input/day13.txt");
    let mut carts: Vec<Cart> = Vec::new();
    let mut cartpos: HashMap<(usize, usize), i8> = HashMap::new();
    let mut crashed: HashSet<i8> = HashSet::new();
    let mut id = 0;
    let grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    ch_to_dir(c).map(|dir| {
                        carts.push(Cart { y, x, dir, rot: 0, id });
                        cartpos.insert((y, x), id);
                        id += 1;
                    });
                    c
                })
                .collect()
        })
        .collect();
    loop {
        carts.sort();
        for cart in &mut carts {
            if crashed.contains(&cart.id) {
                continue;
            }
            let c = grid[cart.y][cart.x];
            assert!(cartpos.remove(&(cart.y, cart.x)) != None);
            match c {
                '+' => {
                    cart.dir = cart.dir.rotate(cart.rot);
                    cart.rot = (cart.rot + 1) % 3;
                }
                '/' | '\\' => {
                    cart.dir = cart.dir.flip(c);
                }
                _ => {}
            }
            match cart.dir {
                U => cart.y -= 1,
                R => cart.x += 1,
                D => cart.y += 1,
                L => cart.x -= 1,
            }
            if let Some(&old_cart) = cartpos.get(&(cart.y, cart.x)) {
                if crashed.is_empty() {
                    println!("Part 1: {},{}", cart.x, cart.y);
                }
                crashed.insert(cart.id);
                crashed.insert(old_cart);
                cartpos.remove(&(cart.y, cart.x));
            } else {
                cartpos.insert((cart.y, cart.x), cart.id);
            }
        }
        carts = carts.iter().cloned().filter(|cart| !crashed.contains(&cart.id)).collect();
        if carts.len() == 1 {
            println!("Part 1: {},{}", carts[0].x, carts[0].y);
            break;
        }
    }
}

fn ch_to_dir(c: char) -> Option<Dir> {
    match c {
        '^' => Some(U),
        '>' => Some(R),
        'v' => Some(D),
        '<' => Some(L),
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

impl Dir {
    fn rotate(self, rot: i8) -> Self {
        match rot {
            0 => match self {
                U => L,
                R => U,
                D => R,
                L => D,
            },
            2 => match self {
                U => R,
                R => D,
                D => L,
                L => U,
            },
            _ => self,
        }
    }
    fn flip(self, track: char) -> Self {
        match track {
            '/' => match self {
                U => R,
                R => U,
                D => L,
                L => D,
            },
            '\\' => match self {
                U => L,
                R => D,
                D => R,
                L => U,
            },
            _ => self,
        }
    }
}

#[derive(Clone,Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Cart {
    y: usize,
    x: usize,
    dir: Dir,
    rot: i8,
    id: i8,
}
