// http://llever.com/exercism-rust-zh/sum-of-multiples/README.zh.html
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 0 {
        return 0;
    }

    // 首先列出 1 到 limit, 然后看每个数能否被 factors 里的数整除
    (1..limit).filter(|x| {
        factors.iter().any(|y| { *x % *y == 0 })
    }).sum()
}
