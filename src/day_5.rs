//! This is my solution for [Advent of Code - Day 5](https://adventofcode.com/2020/day/5) -
//! _Binary Boarding_
//!
//! I found this one quite easy. The 'binary space partitioning' was a very complex way of saying
//! treat the seat code as a binary number where `F` or `L` is `0` and `B` or `R`. I put some effort
//! when parsing them for part one to provide methods to extract the row an column from the full
//! number, but these turned out not to be needed for part 2 either.
//!
//! Part 1 was just parsing the seat ids, see [`Seat::from_line`] and then using
//! [`Iterator::max`] on the resulting Vec of seats.
//!
//! Part 2 is solved by [`find_seat`].

use std::fs;
use std::collections::HashSet;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let contents = fs::read_to_string("res/day-5-input").expect("Failed to read file");
    let allocated_ids: HashSet<usize> =
        contents.lines()
            .map(|line| Seat::from_line(line))
            .map(|seat| seat.id).collect();

    let max_id = allocated_ids.iter().max().unwrap();
    println!("Max Seat ID: {} ", max_id);

    let seat_id = find_seat(&allocated_ids).unwrap();
    println!("My Seat ID: {} ", seat_id);
}

/// A Seat identified by its numerical seat ID.
///
/// In the final implementation, this could have just been a usize as [`Seat::row`] and
/// [`Seat::column`] were not needed.
#[derive(Debug, Eq, PartialEq)]
struct Seat { id: usize }

impl Seat {
    /// Parse a line from the puzzle input into a Seat ID
    ///
    /// > The first 7 characters will either be `F` or `B`; these specify exactly one of the 128
    /// > rows on the plane (numbered 0 through 127). Each letter tells you which half of a region
    /// > the given seat is in. Start with the whole list of rows; the first letter indicates
    /// > whether the seat is in the front (0 through 63) or the back (64 through 127). The next
    /// > letter indicates which half of that region the seat is in, and so on until you're left
    /// > with exactly one row.
    /// >
    /// > For example, consider just the first seven characters of `FBFBBFFRLR`:
    /// >
    /// > - Start by considering the whole range, rows 0 through 127.
    /// > - `F` means to take the lower half, keeping rows 0 through 63.
    /// > - `B` means to take the upper half, keeping rows 32 through 63.
    /// > - `F` means to take the lower half, keeping rows 32 through 47.
    /// > - `B` means to take the upper half, keeping rows 40 through 47.
    /// > - `B` keeps rows 44 through 47.
    /// > - `F` keeps rows 44 through 45.
    /// > - The final `F` keeps the lower of the two, row 44.
    /// >
    /// > The last three characters will be either `L` or `R`; these specify exactly one of the 8
    /// > columns of seats on the plane (numbered 0 through 7). The same process as above proceeds
    /// > again, this time with only three steps. `L` means to keep the lower half, while `R` means
    /// > to keep the upper half.
    /// >
    /// > For example, consider just the last 3 characters of `FBFBBFFRLR`:
    /// >
    /// > - Start by considering the whole range, columns 0 through 7.
    /// > - `R` means to take the upper half, keeping columns 4 through 7.
    /// > - `L` means to take the lower half, keeping columns 4 through 5.
    /// > - The final `R` keeps the upper of the two, column 5.
    /// >
    /// > So, decoding `FBFBBFFRLR` reveals that it is the seat at row 44, column 5.
    /// >
    /// > Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this
    /// > example, the seat has ID `44 * 8 + 5 = 357`.
    ///
    /// Which all reduces down to "treat the seat code as a binary number where `F` or `L` is `0`
    /// and `B` or `R`"
    ///
    /// # Examples from Tests
    /// ```
    /// assert_eq!(Seat { id: 357 }, Seat::from_line("FBFBBFFRLR"));
    /// assert_eq!(Seat { id: 567 }, Seat::from_line("BFFFBBFRRR"));
    /// assert_eq!(Seat { id: 119 }, Seat::from_line("FFFBBBFRRR"));
    /// assert_eq!(Seat { id: 820 }, Seat::from_line("BBFFBBFRLL"));
    /// ```
    fn from_line(line: &str) -> Seat {
        Seat {
            id: line.chars().fold(
                0,
                |acc, char| (acc << 1) | match char {
                    'F' | 'L' => 0b0,
                    'B' | 'R' => 0b1,
                    unexpected => panic!("Unexpected input char {}", unexpected)
                },
            )
        }
    }


    /// The row is the 7 most significant bits.
    ///
    /// # Examples from Tests
    /// ```
    /// assert_eq!(44, Seat::from_line("FBFBBFFRLR").row());
    /// assert_eq!(70, Seat::from_line("BFFFBBFRRR").row());
    /// assert_eq!(14, Seat::from_line("FFFBBBFRRR").row());
    /// assert_eq!(102, Seat::from_line("BBFFBBFRLL").row());
    /// ```
    #[allow(dead_code)] // used only by tests
    fn row(&self) -> usize {
        self.id >> 3
    }

    /// The column is the three least significant digits.
    ///
    /// # Examples from Tests:
    /// ```
    /// assert_eq!(5, Seat::from_line("FBFBBFFRLR").column());
    /// assert_eq!(7, Seat::from_line("BFFFBBFRRR").column());
    /// assert_eq!(7, Seat::from_line("FFFBBBFRRR").column());
    /// assert_eq!(4, Seat::from_line("BBFFBBFRLL").column());
    /// ```
    #[allow(dead_code)] // used only by tests
    fn column(&self) -> usize {
        self.id & 0b111
    }
}

/// Solution to part 2
///
/// > It's a completely full flight, so your seat should be the only missing boarding pass in your
/// > list. However, there's a catch: some of the seats at the very front and back of the plane
/// > don't exist on this aircraft, so they'll be missing from your list as well.
/// >
/// > Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours
/// > will be in your list.
///
/// This loops through all possible ids until a gap of size one id found.
///
/// # Examples from Tests
/// ```
/// assert_eq!(
///     Some(7),
///     find_seat(&vec!(4, 5, 6, 8, 9).into_iter().collect())
/// );
/// assert_eq!(
///     None,
///     find_seat(&vec!(4, 5, 6, 7, 8, 9).into_iter().collect())
/// );
/// ```
fn find_seat(allocated_ids: &HashSet<usize>) -> Option<usize> {
    for i in 1..(1 << 11) as usize {
        if !allocated_ids.contains(&i)
            && allocated_ids.contains(&(i - 1))
            && allocated_ids.contains(&(i + 1)) {
            return Some(i)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use day_5::{Seat, find_seat};

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse_seat_code() {
        assert_eq!(Seat { id: 357 }, Seat::from_line("FBFBBFFRLR"));
        assert_eq!(Seat { id: 567 }, Seat::from_line("BFFFBBFRRR"));
        assert_eq!(Seat { id: 119 }, Seat::from_line("FFFBBBFRRR"));
        assert_eq!(Seat { id: 820 }, Seat::from_line("BBFFBBFRLL"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_extract_row() {
        assert_eq!(44, Seat::from_line("FBFBBFFRLR").row());
        assert_eq!(70, Seat::from_line("BFFFBBFRRR").row());
        assert_eq!(14, Seat::from_line("FFFBBBFRRR").row());
        assert_eq!(102, Seat::from_line("BBFFBBFRLL").row());
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_extract_column() {
        assert_eq!(5, Seat::from_line("FBFBBFFRLR").column());
        assert_eq!(7, Seat::from_line("BFFFBBFRRR").column());
        assert_eq!(7, Seat::from_line("FFFBBBFRRR").column());
        assert_eq!(4, Seat::from_line("BBFFBBFRLL").column());
    }

    #[test]
    fn can_find_seat() {
        assert_eq!(
            Some(7),
            find_seat(&vec!(4, 5, 6, 8, 9).into_iter().collect())
        );
        assert_eq!(
            None,
            find_seat(&vec!(4, 5, 6, 7, 8, 9).into_iter().collect())
        );
    }
}
