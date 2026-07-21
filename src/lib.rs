//! small utilities — xxjaemin68

pub fn levenshtein(a: &str, b: &str) -> usize {
    let (m, n) = (a.chars().count(), b.chars().count());
    if m == 0 { return n; }
    if n == 0 { return m; }
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = *[
                curr[j] + 1,
                prev[j + 1] + 1,
                prev[j] + cost,
            ].iter().min().unwrap();
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}
