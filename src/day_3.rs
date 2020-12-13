//! This is my solution for [Advent of Code - Day 3](https://adventofcode.com/2020/day/3) -
//! _Toboggan Trajectory_
//!
//! Given an input grid of boolean cells representing a map of a forested slope, find how many trees
//! (cells with #/`true`) a toboggan passed if it slides down the hill at a certain angle. Part 2
//! expands on this, expecting the same calculation for four other slopes, including one that
//! increments the `y` value by more than one, essentially skipping some of the input lines.
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input").expect("Failed to read file");
    let lines: Vec<Vec<bool>> = contents.lines().map(|l| parse_line(l)).collect();
    let count31 = count_trees(lines.clone(), 3, 1);
    println!("Encountered {} trees.", count31);

    let count11 = count_trees(lines.clone(), 1, 1);
    let count51 = count_trees(lines.clone(), 5, 1);
    let count71 = count_trees(lines.clone(), 7, 1);
    let count12 = count_trees(lines.clone(), 1, 2);

    println!(
        "Encountered {} x {} x {} x {} x {} = {} trees.",
        count11, count31, count51, count71, count12,
        count11 * count31 * count51 * count71 * count12
    );
}

/// Parse a line of the input to a usable format
///
/// The line format uses `.` for empty and `#` for a tree, e.g. `.#...##..#.`. This is represented
/// as a Vec<bool>/
///
/// # Examples from Tests
/// ```
///  assert_eq!(
///      vec!(false, false, true, true, false, false, false, false, false, false, false),
///      parse_line("..##.......")
///  );
///  assert_eq!(
///      vec!(false, false, true, false, true, false, false, false, true, false, true),
///      parse_line("..#.#...#.#")
///  );
/// ```
fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

/// Starting at (0,0) iterate over the input grid and count the trees encountered
///
/// > You start on the open square (.) in the top-left corner and need to reach the bottom
/// > (below the bottom-most row on your map).
/// >
/// > -- Part 1 --
/// > From your starting position at the top-left, check the position that is right 3 and down 1.
/// > Then, check the position that is right 3 and down 1 from there, and so on until you go past
/// > the bottom of the map.
/// >
/// > -- Part 2 --
/// > Determine the number of trees you would encounter if, for each of the following slopes, you
/// > start at the top-left corner and traverse the map all the way to the bottom:
/// >     * Right 1, down 1.
/// >     * Right 3, down 1. (This is the slope you already checked.)
/// >     * Right 5, down 1.
/// >     * Right 7, down 1.
/// >     * Right 1, down 2.
///
/// Originally written with just the slope as an input for part one (I had an inkling that multiple
/// slopes might be required), speed was added as an extra parameter to cover the final slope.
///
/// The trees form a repeating pattern so the `x` position can increase indefinitely and the
/// current value can be looked up using a modulus of the line length.
///
/// # Examples from Tests
/// ```
/// assert_eq!(
///     2usize,
///     count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 1, 1)
/// );
/// assert_eq!(
///     7usize,
///     count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 3, 1)
/// );
/// assert_eq!(
///     3usize,
///     count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 5, 1)
/// );
/// assert_eq!(
///     4usize,
///     count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 7, 1)
/// );
/// assert_eq!(
///     2usize,
///     count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 1, 2)
/// );
/// ```
fn count_trees(lines: Vec<Vec<bool>>, slope: usize, speed: usize) -> usize {
    lines.iter().fold(
        (0usize, 0usize, 0usize),
        |(pos_x, pos_y, acc), line|
            if pos_y % speed == 0 {
                (
                    (pos_x + slope) % line.len(),
                    pos_y + 1,
                    acc + (line.get(pos_x).map(|b| match b {
                        true => 1,
                        false => 0
                    }).unwrap_or(0))
                )
            } else {
                (pos_x, pos_y + 1, acc)
            },
    ).2
}

#[cfg(test)]
mod tests {
    use day_3::{count_trees, parse_line};

    fn test_lines() -> Vec<&'static str> {
        vec!(
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        )
    }

    #[test]
    fn can_parse_line() {
        assert_eq!(
            vec!(false, false, true, true, false, false, false, false, false, false, false),
            parse_line(test_lines().get(0).unwrap())
        );
        assert_eq!(
            vec!(false, false, true, false, true, false, false, false, true, false, true),
            parse_line(test_lines().get(3).unwrap())
        );
    }

    #[test]
    fn can_count_trees() {
        assert_eq!(
            2usize,
            count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 1, 1)
        );
        assert_eq!(
            7usize,
            count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 3, 1)
        );
        assert_eq!(
            3usize,
            count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 5, 1)
        );
        assert_eq!(
            4usize,
            count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 7, 1)
        );
        assert_eq!(
            2usize,
            count_trees(test_lines().iter().map(|l| parse_line(l)).collect(), 1, 2)
        );
    }
}
