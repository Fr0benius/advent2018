fn main() {
    let input = include_str!("../../input/day8.txt");
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (g, meta) = {
        let mut g = vec![];
        let mut meta = vec![];
        let mut cur = 0;
        fn init(
            it: &mut impl Iterator<Item = i64>,
            g: &mut Vec<Vec<usize>>,
            meta: &mut Vec<Vec<i64>>,
            cur: &mut usize,
        ) -> Option<usize> {
            let n_children = it.next()?;
            let n_meta = it.next()?;
            let v = *cur;
            if g.len() <= v {
                g.resize_with(v + 1, Default::default);
                meta.resize_with(v + 1, Default::default);
            }
            g[v] = (0..n_children)
                .map(|_| {
                    *cur += 1;
                    init(it, g, meta, cur).unwrap()
                })
                .collect();
            meta[v] = it.take(n_meta as usize).collect();
            Some(v)
        }
        _ = init(&mut nums.iter().cloned(), &mut g, &mut meta, &mut cur);
        (g, meta)
    };
    println!("Part 1: {}", meta.iter().flatten().sum::<i64>());

    fn value(g: &Vec<Vec<usize>>, meta: &Vec<Vec<i64>>, v: usize) -> i64 {
        if g[v].is_empty() {
            meta[v].iter().sum()
        } else {
            meta[v]
                .iter()
                .map(|&k| {
                    g[v].get(k as usize - 1)
                        .map(|&w| value(g, meta, w))
                        .unwrap_or(0)
                })
                .sum()
        }
    }
    println!("Part 2: {}", value(&g, &meta, 0));
}
