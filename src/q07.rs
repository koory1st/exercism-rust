// http://llever.com/exercism-rust-zh/bob/README.zh.html
pub fn reply(message: &str) -> &str {
    if check_silence(message) {
        return "Fine. Be that way!";
    }

    let is_loudly = check_loudly(message);

    let is_question = check_question(message);

    return match (is_loudly, is_question) {
        (false, true) => {
            "Sure."
        }
        (true, true) => {
            "Calm down, I know what I'm doing!"
        }
        (true, _) => {
            "Whoa, chill out!"
        }
        (_, _) => {
            "Whatever."
        }
    };
}

fn check_loudly(message: &str) -> bool {
    let mut has_alphabetic = false;
    for letter in message.chars() {
        if letter.is_alphabetic() {
            has_alphabetic = true;

            if letter.is_lowercase() {
                return false;
            }
        }
    }

    if has_alphabetic {
        return true;
    }
    false
}

fn check_silence(message: &str) -> bool {
    message.trim().len() == 0
}

fn check_question(message: &str) -> bool {
    message.trim_end().ends_with("?")
}




