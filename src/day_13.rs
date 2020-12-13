//! This is my solution for [Advent of Code - Day 13](https://adventofcode.com/2020/day/13) Shuttle
//! Search
//!
//! The first part was relatively simple, and seemed to be more a setup to give a push in the right
//! direction for solving part 2: find the next time after timestamp `t` that a bus with a certain
//! frequency will leave, which is just divide, ceiling, multiply. For part two the task was to find
//! a timestamp where each bus would leave a number of minutes after that timestamp equal to it's
//! index in the input array. Reading around after finding my solution, the puzzle seems to have
//! been inspired by [The Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem).
//! which has uses in cryptography. There was a fairly obvious brute force solution that was
//! suitable for the simple tests, but took way too long for the more complex real input. It was
//! however quick enough to calculate for any pair of busses, and from that build a much faster
//! recursive solution.

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-13-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 13.
pub fn run() {
    let contents = fs::read_to_string("res/day-13-input").expect("Failed to read file");
    let (timestamp, bus_ids) = parse_input(contents.as_str());
    let (bus_id, wait) = find_best_departure(
        timestamp,
        bus_ids.iter().map(|(_, bus_id)| *bus_id).collect(),
    );
    println!("The next bus: {} x wait time: {} minutes = {}", bus_id, wait, bus_id * wait);

    let sequence_start = find_sequential_departure(bus_ids);
    println!("The first sequential start begins at timestamp {}", sequence_start)
}

/// Takes the puzzle input and returns the starting timestamp, and a list of bus IDs
///
/// > Your notes (your puzzle input) consist of two lines. The first line is your estimate of the
/// > earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in
/// > service according to the shuttle company; entries that show x must be out of service, so you
/// > decide to ignore them.
fn parse_input(input: &str) -> (usize, Vec<(usize, usize)>) {
    let mut lines = input.lines();
    let timestamp =
        lines.next().expect("Missing line 1")
            .parse::<usize>().expect("Line 1 was not a valid timestamp");

    let bus_ids: Vec<(usize, usize)> =
        lines.next().expect("Missing line 2")
            .split(',')
            .enumerate()
            .filter_map(|(index, id)| id.parse::<usize>().ok().map(|bus_id| (index, bus_id)))
            .collect();

    (timestamp, bus_ids)
}

/// Calculate the wait time from timestamp until bus_id will depart
/// > The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs
/// > from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11,
/// > 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!
fn next_departure(timestamp: usize, bus_id: usize) -> usize {
    if timestamp == 0 {
        return 0;
    }

    ((timestamp - 1) / bus_id + 1) * bus_id - timestamp
}

/// The solution to part 1 - which is the next bus to depart after your  arrival time?
/// If you can arrive at the bus stop at earliest time, what is the next bus you can take, and how
/// long will you need to wait.
fn find_best_departure(earliest_time: usize, bus_ids: Vec<usize>) -> (usize, usize) {
    let (best_id, departure_time) =
        bus_ids.iter().map(|id| (id, next_departure(earliest_time, *id)))
            .min_by_key(|(_, time)| *time)
            .unwrap();

    (*best_id, departure_time)
}

/// Merges the next index/bus_id pair into an accumulator that satisfies both of the merged roots
///
/// The index/bus_id pair can be seen as an offset from the required timestamp, and a modulus. For
/// any two offset/modulus pairs, `a` and `b` there is a combined offset/modulus pair `ab` that is
/// satisfied only for timestamps that also satisfy both `a` and `b`.
///
/// Iterating through the departures of `a`, and deducting the required offset gives a set of
/// timestamps suitable as a base starting point for a sequence that had `a` at the required offset.
/// For each of those base timestamps, if the next departure of `b` from that point matches the
/// expected offset for `b` then that timestamp would also be valid for sequence containing `b`.
/// These timestamps that are applicable for both will occur with a regular frequency. If the first
/// and second confluence are determined, subtracting the first from the second gives us the modulus
/// of these confluences. The merged offset is given by the time to the next departure after one of
/// the base timestamps.
///
/// This merged offset/modulus pair can then be merged in the same way with the next bus in the
/// input. Once all have been merged then the first timestamp that starts a sequence that matches
/// all of the busses in the input will be the merged modulus - merged offset.
///
/// Given a sequence `x,2,3` i.e 2 with an offset of 1 three with an offset of 2. Then this happens
/// with a base timestamp of 1, and then 7:
///
/// ```
/// t  Valid?    ID:2    ID:3                    ID:6  |
/// ---------------------------------------------------|
/// 0             X       X                       X    |
/// 1    Y                                             |
/// 2             X                                    |  2 = 1 + offset 1
/// 3                     X                            |  3 = 1 + offset 2
/// 4             X                                    |
/// 5                                                  |
/// 6             X       X                       X    |  6 = 1 + offset 5
/// 7    Y                                             |
/// 8             X                                    |  8 = 7 + offset 1
/// 9                     X                            |  9 = 7 + offset 2
/// 10            X                                    |
/// 11                                                 |
/// 12            X       X                       X    | 12 = 7 + offset 5
/// ```
///
/// A timestamp where bus ID 2 has an offset of 1, and bus id 3 has an offset of 2 occur if and only
/// if that timestamp is also valid for bus id 6 with an offset of 5.
fn find_sequential_departure_iter(acc: (usize, usize), next: (usize, usize), remaining_bus_ids: Vec<&(usize, usize)>) -> usize {
    // solve using the larger bus_id as the incrementer
    if acc.1 < next.1 {
        return find_sequential_departure_iter(next, acc, remaining_bus_ids)
    }

    let (offset_a, period_a) = acc;
    let (offset_b, period_b ) = next;

    let mut position = 0;
    let mut first_timestamp = 0;
    let second_timestamp;
    let mut base_offset;

    loop {
        position = position + period_a;
        // prevent -ve starts
        if position < offset_a {
            continue
        }

        base_offset = position - offset_a;
        let next_departure_b = next_departure(base_offset, period_b);
        if next_departure_b == offset_b % period_b {
            if first_timestamp == 0 {
                first_timestamp = base_offset;
            } else {
                second_timestamp = base_offset;
                break;
            }
        }
    };

    let new_period = second_timestamp - first_timestamp;
    let new_offset = next_departure(base_offset, new_period);

    match remaining_bus_ids.split_first() {
        Some((&&next, rest)) => find_sequential_departure_iter(
            (new_offset, new_period),
            next,
            rest.to_vec()
        ),
        None => new_period - new_offset
    }
}

/// The solution to part 2. Sets up the data for ['find_sequential_departure_iter`] and delegates
fn find_sequential_departure(bus_ids: Vec<(usize, usize)>) -> usize {
    let &(pos, first_bus) = bus_ids.get(0).expect("First bus id empty");
    find_sequential_departure_iter((0, 1),  (pos, first_bus), bus_ids.iter().skip(1).collect())
}

#[cfg(test)]
mod tests {
    use day_13::{parse_input, next_departure, find_best_departure, find_sequential_departure};

    #[test]
    fn can_parse() {
        let input = "939
7,13,x,x,59,x,31,19";

        assert_eq!(
            (939usize, vec!((0, 7), (1, 13), (4, 59), (6, 31), (7, 19))),
            parse_input(input)
        )
    }

    #[test]
    fn can_find_next_departure() {
        assert_eq!(0, next_departure(0, 7));
        assert_eq!(6, next_departure(1, 7));
        assert_eq!(1, next_departure(6, 7));
        assert_eq!(0, next_departure(7, 7));
        assert_eq!(6, next_departure(8, 7));

        assert_eq!(6, next_departure(939, 7));
        assert_eq!(10, next_departure(939, 13));
        assert_eq!(5, next_departure(939, 59));
        assert_eq!(22, next_departure(939, 31));
        assert_eq!(11, next_departure(939, 19));
    }

    #[test]
    fn can_find_best_departure() {
        assert_eq!(
            (59, 5),
            find_best_departure(939usize, vec!(7, 13, 59, 31, 19))
        )
    }

    #[test]
    fn can_find_sequential_departure() {
        assert_eq!(1068781, find_sequential_departure(vec!((0, 7), (1, 13), (4, 59), (6, 31), (7, 19))));
        assert_eq!(3417, find_sequential_departure(vec!((0, 17), (2, 13), (3, 19))));
        assert_eq!(754018, find_sequential_departure(vec!((0, 67), (1, 7), (2, 59), (3, 61))));
        assert_eq!(779210, find_sequential_departure(vec!((0, 67), (2, 7), (3, 59), (4, 61))));
        assert_eq!(1261476, find_sequential_departure(vec!((0, 67), (1, 7), (3, 59), (4, 61))));
        assert_eq!(1202161486, find_sequential_departure(vec!((0, 1789), (1, 37), (2, 47), (3, 1889))));
    }
}
