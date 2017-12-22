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

use std::io;
use std::collections::HashSet;

fn is_valid(line: &str) -> bool {
    let mut word_set: HashSet<&str> = HashSet::new();
    for word in line.split_whitespace() {
        if word_set.insert(word) == false {
            return false
        }
    }

    true
}

fn main() {
    let mut valid_count = 0;
    let mut buf = String::new();
    while let Ok(bytes) = io::stdin().read_line(&mut buf) {
        if bytes == 0 {
            break;
        }
        if is_valid(&buf) {
            valid_count += 1;
        }
        buf.clear();
    }
    println!("{}", valid_count);
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
}