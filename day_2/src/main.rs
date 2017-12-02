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
// 5 1 9 5
// 7 5 3
// 2 4 6 8
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

use std::io;

fn calculate_checksum(spreadsheet: &mut Vec<Vec<i32>>) -> i32 {
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
    println!("checksum: {}", calculate_checksum(&mut spreadsheet));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_examples_test() {
        let mut example_spreadsheet = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];

        assert_eq!(calculate_checksum(&mut example_spreadsheet), 18);
    }

}
