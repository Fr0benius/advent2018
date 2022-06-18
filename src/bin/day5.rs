fn alchemize(stream: impl Iterator<Item = char>) -> usize {
    let mut st = vec![];
    for c in stream {
        if let Some(&d) = st.last() {
            if c != d && c.to_ascii_lowercase() == d.to_ascii_lowercase() {
                st.pop();
                continue;
            }
        }
        st.push(c);
    }
    st.len()
}

fn main() {
    let input = include_str!("../../input/day5.txt").trim();
    let mut st = vec![];
    for c in input.chars() {
        if let Some(&d) = st.last() {
            if c != d && c.to_ascii_lowercase() == d.to_ascii_lowercase() {
                st.pop();
                continue;
            }
        }
        st.push(c);
    }
    println!("Part 1: {}", alchemize(input.chars()));
    let part2 = ('a'..='z')
        .map(|c| alchemize(input.chars().filter(|&d| d.to_ascii_lowercase() != c)))
        .min()
        .unwrap();
    println!("Part 2: {}", part2);
}
