// http://llever.com/exercism-rust-zh/armstrong-numbers/README.zh.html
pub fn is_armstrong_number(num: u32) -> bool {
    let string = num.to_string();
    let chars = string.chars();
    let count = string.len() as u32;

    let mut sum = 0u32;
    for x in chars {
        match x.to_string().parse::<u32>() {
            Ok(n) => {
                sum += n.pow(count)
            }
            _ => {}
        };
    }

    if sum == num {
        return true;
    }
    false
}
