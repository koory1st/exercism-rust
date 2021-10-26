// http://llever.com/exercism-rust-zh/grains/README.zh.html
pub fn square(s: u32) -> u64 {
    if s == 1 {
        return 1;
    }

    let mut ret = 1u64;
    for _ in 2..=s {
        ret *= 2;
    }

    ret
}

pub fn total() -> u64 {
    (1..=64)
        .map(|x| { square(x) })
        .sum()
}
