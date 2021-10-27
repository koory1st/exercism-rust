// http://llever.com/exercism-rust-zh/pythagorean-triplet/README.zh.html
pub fn find() -> Option<u32> {
    for a in 3u32..1000 - 3 {
        for b in a + 1..1000 - 2 {
            for c in b + 1..1000 - 1 {
                if a * a + b * b == c * c && a + b + c == 1000 {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}
