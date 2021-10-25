// http://llever.com/exercism-rust-zh/difference-of-squares/README.zh.html
pub fn square_of_sum(n: u32) -> u32 {
    let x: u32 = (0..=n).sum();
    x * x
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..=n).map(|x| { x * x }).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
