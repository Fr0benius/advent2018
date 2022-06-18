fn main() {
    let input = include_str!("../../input/day14.txt").trim();
    let n_rec: usize = input.parse().unwrap();
    let wsize = input.len();
    assert_eq!(wsize, 6);
    // let n_rec = 92510;
    let mut rec = vec![3, 7];
    let mut a = 0;
    let mut b = 1;
    let (mut part1, mut part2) = (false, false);
    while !(part1 && part2) {
        let k = rec[a] + rec[b];
        if k <= 9 {
            rec.push(k);
        } else {
            rec.push(1);
            rec.push(k - 10);
        }
        let n = rec.len();
        a = (a + rec[a] + 1) % n;
        b = (b + rec[b] + 1) % n;
        if n >= n_rec + 10 && !part1 {
            println!(
                "Part 1: {}",
                rec[n_rec..n_rec + 10]
                    .iter()
                    .map(|&k| ((b'0' + k as u8) as char))
                    .collect::<String>()
            );
            part1 = true;
        }
        if n >= wsize + 1 && !part2 {
            for j in 0..=1 {
                let num = rec[n - wsize - 1 + j..n - 1 + j]
                    .iter()
                    .fold(0, |a, &b| a * 10 + b);
                if num == n_rec {
                    println!("Part 2: {}", n - wsize - 1 + j);
                    part2 = true;
                    break;
                }
            }
        }
    }
}
