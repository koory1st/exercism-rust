// http://llever.com/exercism-rust-zh/series/README.zh.html
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    let mut ret = Vec::new();

    if len > digits.len() {
        return ret;
    }

    let mut start_index = 0;


    while digits.len() >= start_index + len {
        let x = digits[start_index..start_index + len].to_string();
        ret.push(x);
        start_index += 1;
    }
    ret
}