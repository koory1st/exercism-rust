// http://llever.com/exercism-rust-zh/collatz-conjecture/README.zh.html
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut n = n;

    let mut times = 0u64;
    loop {
        if n == 1 {
            return Some(times);
        }
        times += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
    }
}