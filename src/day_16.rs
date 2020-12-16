//! This is my solution for [Advent of Code - Day 16](https://adventofcode.com/2020/day/16) -
//! _Ticket Translation_
//!
//! Today felt like a step up in complexity, and the second part required two multiple passes over
//! the data to reduce it down to a unique solution.
//!
//! The hard work for part 1 was mostly getting the data into a usable format [`parse_input`]. After
//! that the solution to the actual puzzle fell out of the data [`get_scan_error_rate`]
//!
//! Part 2 is mostly handled by [`get_valid_positions`] with help from [`map_ticket`]. Also there
//! was more work to be done than usual to turn the puzzle solution into a single number output for
//! submission in [`run`].

use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-16-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 16.
pub fn run() {
    let contents = fs::read_to_string("res/day-16-input").expect("Failed to read file");
    let (constraints, my_ticket, tickets) = parse_input(contents.as_str());
    let invalid = get_scan_error_rate(&constraints, &tickets);
    println!("The scan error rate was: {}", invalid.iter().sum::<usize>());

    let mapping = get_valid_positions(&constraints, &tickets);
    let mapped_ticket = map_ticket(mapping, my_ticket);

    let departure_location = mapped_ticket.get("departure location").expect("missing departure location");
    let departure_station = mapped_ticket.get("departure station").expect("missing departure station");
    let departure_platform = mapped_ticket.get("departure platform").expect("missing departure platform");
    let departure_track = mapped_ticket.get("departure track").expect("missing departure track");
    let departure_date = mapped_ticket.get("departure date").expect("missing departure date");
    let departure_time = mapped_ticket.get("departure time").expect("missing departure time");

    println!(
        "location: {} x station: {} x platform: {} x track: {} x date: {} x time: {} = {}",
        departure_location,
        departure_station,
        departure_platform,
        departure_track,
        departure_date,
        departure_time,
        departure_location * departure_station * departure_platform * departure_track * departure_date * departure_time
    );
}

/// Holds constraints on a fields value
///
/// The constraints in the input file all have the format `class: 1-3 or 5-7`. For that example this
/// would be Constraint { (1,3), (5,7) }
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Constraint {
    lower_range: (usize, usize),
    upper_range: (usize, usize),
}

impl Constraint {
    /// Given a field value, does it fall within either of the constraint's ranges?
    ///
    /// Ranges are inclusive at both ends
    pub(crate) fn validate(&self, number: usize) -> bool {
        (number >= self.lower_range.0 && number <= self.lower_range.1)
            || (number >= self.upper_range.0 && number <= self.upper_range.1)
    }
}

/// The input is in three sections, parses each of these into structured data
///
/// The three sections are separated by blank lines. Sections 2 and 3 have a section label as the
/// first line. See also [`parse_constraints`].
///
/// __Constraints__:  A list of field names followed by a pair of number ranges that constrain the
/// valid values for that field.
///
/// __Your Ticket__: A single line with a list of comma separated, unlabelled field values. See also
/// [`parse_ticket`].
///
/// __Nearby Tickets__: One ticket per line, each is a list of comma separated, unlabelled field
/// values. See also [`parse_ticket`].
///
/// # Examples from Tests
/// ```
/// let input = "class: 1-3 or 5-7
/// row: 6-11 or 33-44
/// seat: 13-40 or 45-50
///
/// your ticket:
/// 7,1,14
///
/// nearby tickets:
/// 7,3,47
/// 40,4,50
/// 55,2,20
/// 38,6,12";
///
/// let mut expected_constraints: HashMap<&str, Constraint> = HashMap::new();
/// let mut expected_constraints: HashMap<&str, Constraint> = HashMap::new();
/// expected_constraints.insert("class", Constraint { lower_range: (1, 3), upper_range: (5, 7) });
/// expected_constraints.insert("row", Constraint { lower_range: (6, 11), upper_range: (33, 44) });
/// expected_constraints.insert("seat", Constraint { lower_range: (13, 40), upper_range: (45, 50) });
/// let expected = (
///     expected_constraints,
///     vec!(7usize, 1usize, 14usize),
///     vec!(
///         vec!(7usize, 3usize, 47usize),
///         vec!(40usize, 4usize, 50usize),
///         vec!(55usize, 2usize, 20usize),
///         vec!(38usize, 6usize, 12usize)
///     )
/// );
/// assert_eq!(expected, parse_input(input));
/// ```
fn parse_input(input: &str) -> (HashMap<&str, Constraint>, Vec<usize>, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let constraints = parse_constraints(parts.next().expect("Invalid input - missing part 1"));
    let my_ticket = parse_ticket(
        parts.next().expect("Invalid input - missing part 2")
            .lines().nth(1).expect("Invalid input, failed to find my ticket numbers")
    );
    let other_tickets =
        parts.next().expect("Invalid input - missing part 3")
            .lines().skip(1).map(|line| parse_ticket(line))
            .collect();

    (constraints, my_ticket, other_tickets)
}

/// Parses the constraint section.
fn parse_constraints(input: &str) -> HashMap<&str, Constraint> {
    let re = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").expect("Invalid Regex");

    input.lines().map(|line| {
        let cap = re.captures(line).expect("Failed to parse constraint line");
        (
            cap.get(1).expect("Missing constraint label").as_str(),
            Constraint {
                lower_range: (
                    cap.get(2).expect("missing min 1").as_str().parse().expect("min 1 not a number"),
                    cap.get(3).expect("missing max 1").as_str().parse().expect("max 1 not a number")
                ),
                upper_range: (
                    cap.get(4).expect("missing min 2").as_str().parse().expect("min 2 not a number"),
                    cap.get(5).expect("missing max 2").as_str().parse().expect("max 2 not a number")
                ),
            }
        )
    }
    ).collect()
}

/// Parses a single line with a list of comma separated, unlabelled field values.
fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',').flat_map(|num| num.parse()).collect()
}

/// The solution to part 1. Delegates most of the work to [`get_invalid_numbers`].
///
/// > Start by determining which tickets are completely invalid; these are tickets that contain
/// > values which aren't valid for any field. Ignore your ticket for now. Adding together all of
/// > the invalid values produces your ticket scanning error rate. Consider the validity of the
/// > nearby tickets you scanned. What is your ticket scanning error rate?
///
/// # Example from Test:
/// ```
/// let input = "class: 1-3 or 5-7
/// row: 6-11 or 33-44
/// seat: 13-40 or 45-50
///
/// your ticket:
/// 7,1,14
///
/// nearby tickets:
/// 7,3,47
/// 40,4,50
/// 55,2,20
/// 38,6,12";
/// let (constraints, _, tickets) = parse_input(input);
/// assert_eq!(
///    vec!(4usize, 55usize, 12usize),
///    get_scan_error_rate(&constraints, &tickets)
/// )
/// ```
fn get_scan_error_rate(constraints: &HashMap<&str, Constraint>, tickets: &Vec<Vec<usize>>) -> Vec<usize> {
    tickets.into_iter().flat_map(|ticket| get_invalid_numbers(&constraints, &ticket)).collect()
}

/// Given a ticket, and set of constraints return a list of numbers that are not valid for __any__
/// of the constraints, regardless of position.
fn get_invalid_numbers(constraints: &HashMap<&str, Constraint>, ticket: &Vec<usize>) -> Vec<usize> {
    ticket.into_iter()
        .flat_map(|&number|
            if constraints.iter().any(|(_, cons)| cons.validate(number)) {
                None
            } else {
                Some(number)
            }
        )
        .collect()
}

/// Most of the solution to part 2.
///
/// Given a set of constraints, and ticket data where there is a
/// one-to-one mapping between a constraint field name, and position in the list solve the unique
/// mapping of field to list position.
///
/// > Now that you've identified which tickets contain invalid values, discard those tickets
/// > entirely. Use the remaining valid tickets to determine which field is which.
///
/// > Using the valid ranges for each field, determine what order the fields appear on the tickets.
/// > The order is consistent between all tickets: if seat is the third field, it is the third field
/// > on every ticket, including your ticket.
///
/// Firstly this builds a mapping of field -> Set<usize> of the positions for which all valid
/// tickets have a valid number in that field. These start valid for each field, and get reduced by
/// iterating over all tickets, and removing positions that hold invalid numbers for a constraint.
///
/// Tickets are discarded as invalid if [`get_invalid_numbers`] is not empty for that ticket.
///
/// We then repeatedly loop over this map of sets, where a singleton set is encountered we write
/// that position to the output array, and remove that position from all fields' sets. This
/// generates more singletons, and the process is repeated until the output map is fully populated.
///
/// # Example from Tests
/// ```
/// let input = "class: 0-1 or 4-19
/// row: 0-5 or 8-19
/// seat: 0-13 or 16-19
///
/// your ticket:
/// 11,12,13
///
/// nearby tickets:
/// 3,9,18
/// 15,1,5
/// 5,14,9";
///
/// let mut expected: HashMap<&str, usize> = HashMap::new();
/// expected.insert("class", 1usize);
/// expected.insert("row", 0usize);
/// expected.insert("seat", 2usize);
///
/// let (constraints, _, tickets) = parse_input(input);
///
/// assert_eq!(expected, get_valid_positions(&constraints, &tickets));
/// ```
fn get_valid_positions<'a>(constraints: &'a HashMap<&str, Constraint>, tickets: &Vec<Vec<usize>>) -> HashMap<&'a str, usize> {
    let mut validity: HashMap<&str, HashSet<usize>> = HashMap::new();
    for ticket in tickets {
        // discard invalid
        if !get_invalid_numbers(constraints, &ticket).is_empty() {
            continue
        }

        // now we have a ticket length initialise the sets
        if validity.is_empty()
        {
            let len = ticket.len();
            constraints.into_iter().for_each(|(&key, _)| {
                let set: HashSet<usize> = (0..len).into_iter().collect();
                validity.insert(key, set);
            })
        }

        ticket.into_iter().enumerate().for_each(|(i, &number)| {
            for (&key, &constraint) in constraints {
                let not_valid = !constraint.validate(number);
                if not_valid {
                    let set = validity.get_mut(key).expect("missing validity");
                    set.remove(&i);
                }
            }
        });
    }

    let mut output: HashMap<&str, usize> = HashMap::new();
    let keys: Vec<&str> = validity.keys().map(|k| *k).collect();

    loop {
        let singletons: HashMap<&str, usize> =
            validity.iter()
                .filter_map(
                    |(key, set)| {
                        if set.len() == 1 {
                            Some((*key, *set.iter().next().expect("set has length 1")))
                        } else {
                            None
                        }
                    }
                )
                .collect();

        if singletons.is_empty() {
            panic!("failed to find singleton")
        }

        singletons.into_iter().for_each(|(key, position)| {
            output.insert(key, position);
            for &key in keys.as_slice() {
                let set = validity.get_mut(key).expect("missing validity");
                set.remove(&position);
            }
        });

        if output.len() == validity.len() {
            break;
        }
    }

    output
}

/// The final step of part 2: combine a mapping from [`get_valid_positions`] with ticket data.
fn map_ticket(mapping: HashMap<&str, usize>, ticket: Vec<usize>) -> HashMap<&str, usize> {
    mapping.into_iter().map(|(key, pos)| (key, *ticket.get(pos).unwrap())).collect()
}

#[cfg(test)]
mod tests {
    use day_16::{Constraint, parse_input, get_scan_error_rate, get_valid_positions, map_ticket};
    use std::collections::HashMap;

    fn get_input() -> &'static str {
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
    }

    #[test]
    fn can_parse() {
        let mut expected_constraints: HashMap<&str, Constraint> = HashMap::new();
        expected_constraints.insert("class", Constraint { lower_range: (1, 3), upper_range: (5, 7) });
        expected_constraints.insert("row", Constraint { lower_range: (6, 11), upper_range: (33, 44) });
        expected_constraints.insert("seat", Constraint { lower_range: (13, 40), upper_range: (45, 50) });

        let expected = (
            expected_constraints,
            vec!(7usize, 1usize, 14usize),
            vec!(
                vec!(7usize, 3usize, 47usize),
                vec!(40usize, 4usize, 50usize),
                vec!(55usize, 2usize, 20usize),
                vec!(38usize, 6usize, 12usize)
            )
        );

        assert_eq!(expected, parse_input(get_input()));
    }

    #[test]
    fn can_calculate_error_rate() {
        let (constraints, _, tickets) = parse_input(get_input());

        assert_eq!(vec!(4usize, 55usize, 12usize), get_scan_error_rate(&constraints, &tickets));
    }

    #[test]
    fn can_validate_positions() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let (constraints, _, tickets) = parse_input(input);

        assert_eq!(get_expected_mapping(), get_valid_positions(&constraints, &tickets));
    }

    fn get_expected_mapping() -> HashMap<&'static str, usize> {
        let mut expected: HashMap<&str, usize> = HashMap::new();
        expected.insert("class", 1usize);
        expected.insert("row", 0usize);
        expected.insert("seat", 2usize);
        expected
    }

    #[test]
    fn can_map_ticket() {
        let mut expected: HashMap<&str, usize> = HashMap::new();
        expected.insert("class", 12);
        expected.insert("row", 11);
        expected.insert("seat", 13);

        assert_eq!(expected, map_ticket(get_expected_mapping(), vec!(11, 12, 13)));
    }
}
