use std::iter::repeat;
fn main() {
    let input = include_str!("../../input/day11.txt");
    let serial: i64 = input.trim().parse().unwrap();
    let pow = |x, y| ((x + 10) * y + serial) * (x + 10) / 100 % 10 - 5;

    let sp = {
        let mut pows = [[0; 301]; 301];
        for x in 1..=300 {
            for y in 1..=300 {
                pows[x][y] =
                    pows[x - 1][y] + pows[x][y - 1] - pows[x - 1][y - 1] + pow(x as i64, y as i64);
            }
        }
        pows
    };
    let square = |x: usize, y: usize, k: usize| {
        if x.max(y) + k - 1 > 300 {
            0
        } else {
            sp[x + k - 1][y + k - 1] - sp[x + k - 1][y - 1] - sp[x - 1][y + k - 1]
                + sp[x - 1][y - 1]
        }
    };
    let best = (1..=300)
        .flat_map(|x| repeat(x).zip(1..=300))
        .map(|(x, y)| (square(x, y, 3), x, y))
        .max()
        .unwrap();
    println!("Part 1: {},{}", best.1, best.2);
    let best = (1..=300)
        .flat_map(|x| repeat(x).zip(1..=300))
        .flat_map(|pr| repeat(pr).zip(1..=300))
        .map(|((x, y), k)| (square(x, y, k), x, y, k))
        .max()
        .unwrap();
    println!("Part 1: {},{},{}", best.1, best.2, best.3);
}
