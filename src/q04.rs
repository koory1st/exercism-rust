// http://llever.com/exercism-rust-zh/raindrops/README.zh.html
pub fn raindrops(n: u32) -> String {
    let mut ret = String::new();

    let has_factor = |y: u32| n % y == 0;

    if has_factor(3) {
        ret.push_str("Pling");
    }

    if has_factor(5) {
        ret.push_str("Plang");
    }

    if has_factor(7) {
        ret.push_str("Plong");
    }

    if ret.len() == 0 {
        return n.to_string();
    }

    ret
}
