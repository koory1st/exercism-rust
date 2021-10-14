// http://llever.com/exercism-rust-zh/beer-song/README.zh.html

// 99 bottles of beer on the wall, 99 bottles of beer.\nTake one down and pass it around, 98 bottles of beer on the wall.
// 2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.
// 1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.
// No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.
pub fn verse(n: i32) -> String {
    // todo 使用 struct 保存结果
    let curr = Type::new(n);
    let curr_count_str_initial = curr.get_count_str(true);
    let curr_count_str = curr.get_count_str(false);
    let curr_bottle = curr.get_bottle();
    let curr_action = curr.get_action();
    let next = curr.next();
    let next_count_str = next.get_count_str(false);
    let next_bottle = next.get_bottle();
    format!("{} {} of beer on the wall, {} {} of beer.\n{}, {} {} of beer on the wall.\n",
            curr_count_str_initial,
            curr_bottle,
            curr_count_str,
            curr_bottle,
            curr_action,
            next_count_str,
            next_bottle
    )
}

pub fn sing(start: i32, end: i32) -> String {
    let mut ret: Vec<String> = Vec::new();
    let mut i = start;
    while i >= end {
        ret.push(verse(i));
        i = i - 1;
    }
    ret.join("\n")
}

enum Type {
    Multi(i32),
    Single,
    None,
}

impl Type {
    fn new(count: i32) -> Type {
        match count {
            0 => Type::None,
            1 => Type::Single,
            n => Type::Multi(n)
        }
    }

    fn next(&self) -> Type {
        match self {
            Type::Multi(n) => {
                let next_count = n - 1;
                if next_count == 1 {
                    return Type::Single;
                }
                Type::Multi(next_count)
            }
            Type::Single => {
                Type::None
            }
            Type::None => {
                Type::Multi(99)
            }
        }
    }

    fn get_count_str(&self, is_uppercase: bool) -> String {
        match self {
            Type::Multi(n) => { n.to_string() }
            Type::Single => { 1.to_string() }
            Type::None => {
                let mut initial = "n".to_string();
                if is_uppercase {
                    initial = initial.to_uppercase();
                }
                format!("{}o more", initial)
            }
        }
    }

    fn get_bottle(&self) -> String {
        let mut bottle = "bottle".to_string();
        match self {
            Type::Multi(_) | Type::None => { bottle.push_str("s"); }
            Type::Single => {}
        }
        bottle
    }

    fn get_action(&self) -> String {
        if let Type::None = self {
            return "Go to the store and buy some more".to_string();
        }

        let ret = match self {
            Type::Multi(_) => { "one" }
            Type::Single => { "it" }
            _ => { "" }
        };
        format!("Take {} down and pass it around", ret)
    }
}

