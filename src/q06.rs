// http://llever.com/exercism-rust-zh/nth-prime/README.zh.html

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut num = 3u32;
    let mut index = 0u32;
    loop {
        if is_prime(num) {
            index += 1;
            if index == n {
                return num;
            }
        }

        num += 2;
    }
}

fn is_prime(num: u32) -> bool {
    let mut f = 2u32;

    while f * f <= num {
        if num % f == 0 {
            return false;
        }

        f += 1;
    }
    return true;
}
