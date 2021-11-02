// http://llever.com/exercism-rust-zh/prime-factors/README.zh.html
pub fn factors(n: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();

    if n < 2 {
        return ret;
    }

    let mut factor = 2u64;

    let mut n = n;

    while factor <= n {
        if n % factor == 0 {
            ret.push(factor);
            n = n / factor;
            continue;
        }

        factor += 1;
    }
    ret
}

