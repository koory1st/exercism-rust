extern crate exercism_rust;
use exercism_rust::q05::reverse;

/// Process a single test case for the property `reverse`
fn process_reverse_case(input: &str, expected: &str) {
    assert_eq!(&reverse(input), expected)
}

#[test]
/// empty string
fn test_empty_string() {
    process_reverse_case("", "");
}

#[test]
//#[ignore]
/// a word
fn test_a_word() {
    process_reverse_case("robot", "tobor");
}

#[test]
//#[ignore]
/// a capitalized word
fn test_a_capitalized_word() {
    process_reverse_case("Ramen", "nemaR");
}

#[test]
//#[ignore]
/// a sentence with punctuation
fn test_a_sentence_with_punctuation() {
    process_reverse_case("I'm hungry!", "!yrgnuh m'I");
}

#[test]
//#[ignore]
/// a palindrome
fn test_a_palindrome() {
    process_reverse_case("racecar", "racecar");
}

#[test]
//#[ignore]
/// wide characters
fn test_wide_characters() {
    process_reverse_case("子猫", "猫子");
}

#[test]
//#[ignore]
// #[cfg(feature = "grapheme")]
/// grapheme clusters
fn test_grapheme_clusters() {
    process_reverse_case("uüu", "uüu");
}
