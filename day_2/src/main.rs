// --- Day 2: Corruption Checksum ---
//
// As you walk through the door, a glowing humanoid shape yells in your
// direction. "You there! Your state appears to be idle. Come help us repair
// the corruption in this spreadsheet - if we take another millisecond, we'll
// have to display an hourglass cursor!"
//
// The spreadsheet consists of rows of apparently-random numbers. To make sure
// the recovery process is on the right track, they need you to calculate
// the spreadsheet's checksum. For each row, determine the difference between
// the largest value and the smallest value; the checksum is the sum of all of
//these differences.
//
// For example, given the following spreadsheet:
//
// 5 1 9 5
// 7 5 3
// 2 4 6 8
//
//   - The first row's largest and smallest values are 9 and 1, and their
//     difference is 8.
//   - The second row's largest and smallest values are 7 and 3, and their
//     difference is 4.
//   - The third row's difference is 6.
//
// In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
//
// What is the checksum for the spreadsheet in your puzzle input?
//
// Your puzzle answer was 47136.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// "Great work; looks like we're on the right track after all. Here's a star
// for your effort." However, the program seems a little worried. Can programs
// be worried?
//
// "Based on what we're seeing, it looks like all the User wanted is some
// information about the evenly divisible values in the spreadsheet.
// Unfortunately, none of us are equipped for that kind of calculation -
// most of us specialize in bitwise operations."
//
// It sounds like the goal is to find the only two numbers in each row where
// one evenly divides the other - that is, where the result of the division
// operation is a whole number. They would like you to find those numbers on
// each line, divide them, and add up each line's result.
//
// For example, given the following spreadsheet:
//
// 5 9 2 8
// 9 4 7 3
// 3 8 6 5
//
// - In the first row, the only two numbers that evenly divide are 8 and 2;
//   the result of this division is 4.
// - In the second row, the two numbers are 9 and 3; the result is 3.
// - In the third row, the result is 2.
//
// In this example, the sum of the results would be 4 + 3 + 2 = 9.
//
// What is the sum of each row's result in your puzzle input?
use std::io;

fn find_numerator(d: i32, candidates: &[i32]) -> Option<i32> {
    if d == 0 {
        return None;
    }

    for n in candidates {
        if n % d == 0 {
            return Some(*n);
        }
    }

    None
}

fn calculate_division_checksum(spreadsheet: &mut Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;
    for row in spreadsheet.iter_mut() {
        row.sort();
        for (i, d) in row.iter().enumerate() {
            match find_numerator(*d, &row[i + 1..]) {
                Some(n) => checksum += n / d,
                None => continue,
            }
        }
    }

    checksum
}

fn calculate_distance_checksum(spreadsheet: &mut Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;
    for row in spreadsheet.iter_mut() {
        row.sort();
        checksum += row[row.len() - 1] - row[0];
    }

    checksum
}

fn main() {
    let mut spreadsheet = Vec::new();
    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let row: Vec<i32> = buf.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        buf.clear();
        spreadsheet.push(row);
    }
    println!("checksum #1: {}", calculate_distance_checksum(&mut spreadsheet));
    println!("checksum #2: {}", calculate_division_checksum(&mut spreadsheet));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_examples_test() {
        let mut example_spreadsheet = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8]];

        assert_eq!(calculate_distance_checksum(&mut example_spreadsheet), 18);
    }

    #[test]
    fn part_two_examples_test() {
        let mut example_spreadsheet = vec![
            vec![5, 9, 2, 8],
            vec![9, 4, 7, 3],
            vec![3, 8, 6, 5]];

        assert_eq!(calculate_division_checksum(&mut example_spreadsheet), 9);
    }

}
