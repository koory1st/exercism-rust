// http://llever.com/exercism-rust-zh/nth-prime/README.zh.html
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    input.to_owned().graphemes(true).rev().collect()
}
