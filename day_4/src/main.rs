// --- Day 4: High-Entropy Passphrases ---
// A new system policy has been put in place that requires all accounts to
// use a passphrase instead of simply a password. A passphrase consists of a
// series of words (lowercase letters) separated by spaces.
//
// To ensure security, a valid passphrase must contain no duplicate words.
//
// For example:
//
// aa bb cc dd ee is valid.
// aa bb cc dd aa is not valid - the word aa appears more than once.
// aa bb cc dd aaa is valid - aa and aaa count as different words.
//
// The system's full passphrase list is available as your puzzle input.
// How many passphrases are valid?
//
// Your puzzle answer was 451.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
// For added security, yet another system policy has been put in place.
// Now, a valid passphrase must contain no two words that are anagrams of each
// other - that is, a passphrase is invalid if any word's letters can be
// rearranged to form any other word in the passphrase.
//
// For example:
//
// - abcde fghij is a valid passphrase.
// - abcde xyz ecdab is not valid - the letters from the third word can be
//   rearranged to form the first word.
// - a ab abc abd abf abj is a valid passphrase, because all letters need to be
//   used when forming another word.
// - iiii oiii ooii oooi oooo is valid.
// - oiii ioii iioi iiio is not valid - any of these words can be rearranged to
//   form any other word.
//
// Under this new system policy, how many passphrases are valid?
//
// Your puzzle answer was 223.
//
// Both parts of this puzzle are complete! They provide two gold stars: **

use std::io;
use std::collections::HashSet;

fn is_valid_anagram(line: &str) -> bool {
    let mut word_set: HashSet<Vec<char>> = HashSet::new();
    for word in line.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        if word_set.insert(chars) == false {
            return false;
        }
    }

    true
}

fn is_valid(line: &str) -> bool {
    let mut word_set: HashSet<&str> = HashSet::new();
    for word in line.split_whitespace() {
        if word_set.insert(word) == false {
            return false;
        }
    }

    true
}

fn main() {
    let mut part_one_valid_count = 0;
    let mut part_two_valid_count = 0;
    let mut buf = String::new();
    while let Ok(bytes) = io::stdin().read_line(&mut buf) {
        if bytes == 0 {
            break;
        }
        if is_valid(&buf) {
            part_one_valid_count += 1;
        }
        if is_valid_anagram(&buf) {
            part_two_valid_count += 1;
        }
        buf.clear();
    }
    println!("Part one: {}", part_one_valid_count);
    println!("Part two: {}", part_two_valid_count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_examples_test() {
        assert_eq!(is_valid(&"aa bb cc dd ee"), true);
        assert_eq!(is_valid(&"aa bb cc dd aa"), false);
        assert_eq!(is_valid(&"aa bb cc dd aaa"), true);
    }

    #[test]
    fn part_two_examples_test() {
        assert_eq!(is_valid_anagram(&"abcde fghij"), true);
        assert_eq!(is_valid_anagram(&"abcde xyz ecdab"), false);
        assert_eq!(is_valid_anagram(&"iiii oiii ooii oooi oooo"), true);
        assert_eq!(is_valid_anagram(&"oiii ioii iioi iiio"), false);
    }
}
