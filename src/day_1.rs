//! This is my solution for [Advent of Code - Day 1](https://adventofcode.com/2020/day/1)
//!
//! The challenge was to determine which numbers in the input file summed to 2020, part 1 asking for
//! two numbers, part 2 asking for three.
//!
//! For part one, if the array is sorted it uses the property that starting with the lowest value in
//! the array (`min`) you can throw away anything larger than `2020 - min`. Then given this new
//! maximum (`max'`) then you can throw away anything smaller than `2020 - max'`. You can then
//! recursively do the same thing with the new bounds, returning success if the two bounds summed
//! equal the target.
//!
//! For part 2, it starts with the lowest value in the array, `a`, solves part one with a target of
//! `2020 - a`, if a match is found then that solution with `a` is a triple that adds up to 2020.
//! Otherwise discard `a` as it's not in the final triple, and repeat for the next number in the
//! input.

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input").expect("Failed to read file");
    let mut ints = read_to_ints(contents.as_str());

    let (a, b) = find_pair_sum(&mut ints, 2020).unwrap();
    println!("{} x {} = {}", a, b, a * b);

    let (a, b, c) = find_triple_sum(&mut ints, 2020).unwrap();
    println!("{} x {} x {} = {}", a, b, c, a * b * c);
}

/// Parse a file with each line representing an integer into usable data
fn read_to_ints(contents: &str) -> Vec<i32> {
    return contents.lines().flat_map(|line| line.parse::<i32>().ok()).collect();
}

/// This recursively narrows the array by discarding ranges from the extremes until a match is found
/// or the input is exhausted.
///
/// Given that the array is sorted, assume that the first item, `min`, is part of the sum, and use
/// [`find_new_bound`] to find the largest value in the array, `max` such that `min + max <=
/// target`. If they equal the target, the sum has been found can be returned, otherwise do the same
/// to find a new lower bound `min'`. Repeat this recursively until a pair is found or the bounds
/// meet.
fn find_pair_sum_iter(ints: &Vec<i32>, target_sum: i32, min_idx: usize, max_idx: usize) -> Option<(i32, i32)> {
    let min = ints.get(min_idx).expect("min out of range");
    let max = ints.get(max_idx).expect("max out of range");

    if min + max == target_sum {
        return Some((*min, *max));
    }

    let max_target_number = target_sum - min;
    let new_max_idx = find_new_bound(ints, max_target_number, min_idx + 1, max_idx);
    let new_max = ints.get(new_max_idx).expect("mid out of range");
    if min + new_max == target_sum {
        return Some((*min, *new_max));
    }

    let min_target_number = target_sum - new_max;
    let new_min_idx = find_new_bound(ints, min_target_number, min_idx, new_max_idx - 1);

    if new_min_idx + 1 >= new_max_idx {
        return None;
    }

    return find_pair_sum_iter(ints, target_sum, new_min_idx, new_max_idx);
}

/// Use a recursive binary search to largest number in the bounds provided that is less than or equal
/// to the target value.
fn find_new_bound(ints: &Vec<i32>, target_number: i32, min_idx: usize, max_idx: usize) -> usize {
    let mid_idx = ((min_idx + max_idx) / 2) as usize;
    if mid_idx == min_idx {
        return min_idx;
    }

    let mid = *ints.get(mid_idx).expect("mid out of range");
    if mid > target_number
    {
        find_new_bound(ints, target_number, min_idx, mid_idx)
    } else {
        find_new_bound(ints, target_number, mid_idx, max_idx)
    }
}

/// The solution to part 1, also used in part 2
///
/// This is a convenience function that takes the parsed puzzle input, sorts it, and delegates to
/// [`find_pair_sum_iter`], adding in the base values for the accumulator parameters.
///
/// # Examples from tests
/// ```
/// let mut ints = vec!(1721, 979, 366, 299, 675, 1456, 1991, 100);
/// assert_eq!(find_pair_sum(&mut ints, 2020), Some((299i32, 1721i32)));
///
/// let mut invalid_ints = vec!(1721, 979, 366, 298, 675, 1456, 1991, 100);
/// assert_eq!(find_pair_sum(&mut invalid_ints, 2020), None);
///
/// let mut invalid_ints = vec!(1,2,3,4);
/// assert_eq!(find_pair_sum(&mut invalid_ints, 2020), None);
/// ```
pub fn find_pair_sum(ints: &mut Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    ints.sort();
    let max = ints.len() - 1;
    find_pair_sum_iter(ints, target_sum, 0, max)
}

/// The solution to part 2.
///
/// This is less elegant than its [part one counter part][find_pair_sum], and just iterates through
/// the sorted array, using [`find_pair_sum_iter`] to identify if there is a pair of numbers that
/// when added together with the current value make 2020. If there is such a pair then a triple of
/// the current value and the two items in the identified pair is returned. If the whole array is
/// exhausted, then it gives up and returns `None`
///
/// # Examples from tests
/// ```
/// let mut ints = vec!(1721, 979, 366, 299, 675, 1456);
/// assert_eq!(find_triple_sum(&mut ints, 2020), Some((366i32, 675i32, 979i32)));
///
/// let mut invalid_ints = vec!(1721, 979, 366, 299, 674, 1456, 1991, 100);
/// assert_eq!(find_triple_sum(&mut invalid_ints, 2020), None);
/// ```
pub fn find_triple_sum(ints: &mut Vec<i32>, target_sum: i32) -> Option<(i32, i32, i32)> {
    ints.sort();
    let mut i = 0;
    let max = ints.len() - 1;
    while i < ints.len() - 3
    {
        let a = ints.get(i).expect("i out of range");
        let result = find_pair_sum_iter(ints, target_sum - a, i + 1, max);
        if result.is_some() {
            let (b, c) = result.unwrap();
            return Some((*a, b, c));
        }
        i = i + 1;
    }

    return None;
}


#[cfg(test)]
mod tests {
    use day_1::{read_to_ints, find_pair_sum, find_triple_sum};

    #[test]
    fn can_parse_file() {
        let ints = read_to_ints("1953
2006
1926
1946
1722
1776");
        assert_eq!(ints.len(), 6);
        assert_eq!(ints.get(0), Some(&1953i32));
        assert_eq!(ints.get(5), Some(&1776i32));
    }

    #[test]
    fn can_find_sum() {
        let mut ints = vec!(1721, 979, 366, 299, 675, 1456, 1991, 100);
        assert_eq!(find_pair_sum(&mut ints, 2020), Some((299i32, 1721i32)));

        let mut invalid_ints = vec!(1721, 979, 366, 298, 675, 1456, 1991, 100);
        assert_eq!(find_pair_sum(&mut invalid_ints, 2020), None);

        let mut invalid_ints = vec!(1,2,3,4);
        assert_eq!(find_pair_sum(&mut invalid_ints, 2020), None);
    }

    #[test]
    fn can_find_triple_sum() {
        let mut ints = vec!(1721, 979, 366, 299, 675, 1456);
        assert_eq!(find_triple_sum(&mut ints, 2020), Some((366i32, 675i32, 979i32)));

        let mut invalid_ints = vec!(1721, 979, 366, 299, 674, 1456, 1991, 100);
        assert_eq!(find_triple_sum(&mut invalid_ints, 2020), None)
    }
}
