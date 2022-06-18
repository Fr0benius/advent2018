use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day18.txt");
    let orig_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n = orig_grid.len();
    assert!(orig_grid[0].len() == n);
    let mut grid_map = HashMap::new();
    grid_map.insert(orig_grid.clone(), 0);
    let mut grids = vec![orig_grid];
    for k in 1.. {
        let grid = grids.last().unwrap();
        let mut new_grid = grid.clone();
        for i0 in 0..n {
            for j0 in 0..n {
                let mut cnt = HashMap::new();
                for i in [i0 - 1, i0, i0 + 1] {
                    for j in [j0 - 1, j0, j0 + 1] {
                        if i.clamp(0, n - 1) == i && j.clamp(0, n - 1) == j && (i, j) != (i0, j0) {
                            *cnt.entry(grid[i][j]).or_insert(0) += 1;
                        }
                    }
                }
                new_grid[i0][j0] = match grid[i0][j0] {
                    '.' => {
                        if *cnt.get(&'|').unwrap_or(&0) >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        if *cnt.get(&'#').unwrap_or(&0) >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    }

                    '#' => {
                        if *cnt.get(&'|').unwrap_or(&0) >= 1 && *cnt.get(&'#').unwrap_or(&0) >= 1 {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        grids.push(new_grid);
        let calc = |j: usize| {
            let trees: usize = grids[j]
                .iter()
                .map(|v| v.iter().filter(|&&c| c == '|').count())
                .sum();
            let yards: usize = grids[j]
                .iter()
                .map(|v| v.iter().filter(|&&c| c == '#').count())
                .sum();
            trees * yards
        };

        if k == 10 {
            println!("Part 1: {}", calc(k));
        }
        if let Some(j) = grid_map.insert(grids.last().unwrap().clone(), k) {
            let z = 1_000_000_000;
            println!("Part 2: {}", calc((z - j) % (k - j) + j));
            return;
        }
    }
}
