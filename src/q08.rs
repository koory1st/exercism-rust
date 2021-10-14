// http://llever.com/exercism-rust-zh/beer-song/README.zh.html

// 99 bottles of beer on the wall, 99 bottles of beer.\nTake one down and pass it around, 98 bottles of beer on the wall.
// 2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.
// 1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.
// No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.
pub fn verse(n: i32) -> String {
    let curr = Type::new(n);
    let next = curr.next().get_num();

    let curr = curr.get_num();

    format!("{} {} of beer on the wall, {} {} of beer.\n{}, {} {} of beer on the wall.\n",
            curr.count_str_initial,
            curr.bottle,
            curr.count_str,
            curr.bottle,
            curr.action,
            next.count_str,
            next.bottle
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

struct Num {
    count_str_initial: String,
    count_str: String,
    bottle: String,
    action: String,
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

    fn get_num(&self) -> Num {
        Num {
            count_str_initial: self.get_count_str(true),
            count_str: self.get_count_str(false),
            bottle: self.get_bottle(),
            action: self.get_action(),
        }
    }
}

