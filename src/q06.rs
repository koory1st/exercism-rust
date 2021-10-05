// http://llever.com/exercism-rust-zh/nth-prime/README.zh.html
pub fn nth(n: u32) -> u32 {
    let first = Prime { curr: 2 };

    if n == 0 {
        return first.curr;
    }

    let a = first.skip(n as usize - 1).next().unwrap();

    return a;
}

struct Prime {
    curr: u32,
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        for num in self.curr + 1.. {
            if num % 2 == 0 {
                continue;
            }

            let mut has_factor: bool = false;
            for j in 2..num {
                if num % j == 0 {
                    has_factor = true;
                    break;
                }
            }

            if has_factor {
                continue;
            }

            self.curr = num;

            return Some(self.curr);
        }

        None
    }
}
