// --- Day 6: Memory Reallocation ---
// A debugger program here is having an issue: it is trying to repair a memory
// reallocation routine, but it keeps getting stuck in an infinite loop.
//
// In this area, there are sixteen memory banks; each memory bank can hold any
// number of blocks. The goal of the reallocation routine is to balance
// the blocks between the memory banks.
//
// The reallocation routine operates in cycles. In each cycle, it finds
// the memory bank with the most blocks (ties won by the lowest-numbered
// memory bank) and redistributes those blocks among the banks. To do this,
// it removes all of the blocks from the selected bank, then moves to
// the next (by index) memory bank and inserts one of the blocks. It continues
// doing this until it runs out of blocks; if it reaches the last memory bank,
// it wraps around to the first one.
//
// The debugger would like to know how many redistributions can be done
// before a blocks-in-banks configuration is produced that has been seen
// before.
//
// For example, imagine a scenario with only four memory banks:
//
// - The banks start with 0, 2, 7, and 0 blocks. The third bank has
//   the most blocks, so it is chosen for redistribution.
// - Starting with the next bank (the fourth bank) and then continuing to
//   the first bank, the second bank, and so on, the 7 blocks are spread out
//   over the memory banks. The fourth, first, and second banks get two blocks
//   each, and the third bank gets one back. The final result looks
//   like this: 2 4 1 2.
// - Next, the second bank is chosen because it contains the most
//   blocks (four). Because there are four memory banks, each gets one block.
//   The result is: 3 1 2 3.
// - Now, there is a tie between the first and fourth memory banks, both of
//   which have three blocks. The first bank wins the tie, and its three
//   blocks are distributed evenly over the other three banks, leaving it with
//   none: 0 2 3 4.
// - The fourth bank is chosen, and its four blocks are distributed such that
//   each of the four banks receives one: 1 3 4 1.
// - The third bank is chosen, and the same thing happens: 2 4 1 2.
// - At this point, we've reached a state we've seen before: 2 4 1 2 was
//   already seen. The infinite loop is detected after the fifth block
//   redistribution cycle, and so the answer in this example is 5.
//
// Given the initial block counts in your puzzle input, how many redistribution
// cycles must be completed before a configuration is produced that has been
// seen before?
//
// Your puzzle answer was 3156.
//
// The first half of this puzzle is complete! It provides one gold star: *¨
//
// --- Part Two ---
//
// Out of curiosity, the debugger would also like to know the size of
// the loop: starting from a state that has already been seen, how many block
// redistribution cycles must be performed before that same state is seen
// again?
//
// In the example above, 2 4 1 2 is seen again after four cycles, and so
// the answer in that example would be 4.
//
// How many cycles are in the infinite loop that arises from the configuration
// in your puzzle input?
//
// Your puzzle answer was 1610.
//
// Both parts of this puzzle are complete! They provide two gold stars: **
use std::io;
use std::collections::HashSet;

fn find_max(banks: &[u32]) -> Option<usize> {
    if banks.is_empty() {
        return None;
    }

    let mut max_index = 0;
    let mut max = banks[0];
    for i in 1..banks.len() {
        if banks[i] > max {
            max = banks[i];
            max_index = i;
        }
    }

    Some(max_index)
}

fn reallocate(banks: &mut [u32]) -> Option<u32> {
    let mut seen_states: HashSet<Vec<u32>> = HashSet::new();
    let mut cycles = 0;
    while seen_states.insert(banks.to_owned()) {
        let index = match find_max(banks) {
            Some(i) => i,
            None => return None,
        };
        let blocks = banks[index];
        let len = banks.len();
        banks[index] = 0;
        for n in 0..blocks as usize {
            banks[(index + 1 + n) % len] += 1;
        }
        cycles += 1;
    }

    Some(cycles)
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut banks: Vec<u32> = buf.trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    println!("Part one: {}", reallocate(&mut banks).unwrap());
    println!("Part two: {}", reallocate(&mut banks).unwrap());
}

mod test {
    use super::*;

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&vec![]), None);
        assert_eq!(find_max(&vec![1]), Some(0));
        assert_eq!(find_max(&vec![1, 2, 3]), Some(2));
        assert_eq!(find_max(&vec![3, 2, 1]), Some(0));
        assert_eq!(find_max(&vec![1, 3, 3, 2]), Some(1));
    }

    #[test]
    fn test_reallocate() {
        assert_eq!(reallocate(&mut vec![]), None);
        assert_eq!(reallocate(&mut vec![1]), Some(1));
        assert_eq!(reallocate(&mut vec![1, 0]), Some(2));
        assert_eq!(reallocate(&mut vec![0, 0, 1]), Some(3));
    }

    #[test]
    fn test_examples() {
        let mut banks = vec![0, 2, 7, 0];
        assert_eq!(reallocate(&mut banks), Some(5));
        assert_eq!(reallocate(&mut banks), Some(4));
        assert_eq!(banks, vec![2, 4, 1, 2])
    }
}
