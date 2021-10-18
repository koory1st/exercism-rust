// http://llever.com/exercism-rust-zh/proverb/README.zh.html
pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut res_vec: Vec<String> = Vec::new();

    let mut first: String = String::new();
    let mut prev: String = String::new();
    for (i, x) in list.iter().enumerate() {
        if i == 0 {
            first = x.to_string();
            prev = x.to_string();
            continue;
        }
        res_vec.push(format!("For want of a {} the {} was lost.", prev, x));
        prev = x.to_string();
    }

    res_vec.push(format!("And all for the want of a {}.", first.as_str()));
    res_vec.join("\n")
}
