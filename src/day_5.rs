use std::fs;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Seat { id: usize }

impl Seat {
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

    #[allow(dead_code)] // used only by tests
    fn row(&self) -> usize {
        self.id >> 3
    }

    #[allow(dead_code)] // used only by tests
    fn column(&self) -> usize {
        self.id & 7
    }
}

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

fn find_seat(allocated_ids: &HashSet<usize>) -> Option<usize> {
    for i in 1..(1 << 11) as usize {
        if !allocated_ids.contains(&i) && allocated_ids.contains(&(i - 1)) && allocated_ids.contains(&(i + 1)) {
            return Some(i)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use day_5::{Seat, find_seat};

    #[test]
    fn can_parse_seat_code() {
        assert_eq!(Seat { id: 357 }, Seat::from_line("FBFBBFFRLR"));
        assert_eq!(Seat { id: 567 }, Seat::from_line("BFFFBBFRRR"));
        assert_eq!(Seat { id: 119 }, Seat::from_line("FFFBBBFRRR"));
        assert_eq!(Seat { id: 820 }, Seat::from_line("BBFFBBFRLL"));
    }

    #[test]
    fn can_extract_row() {
        assert_eq!(44, Seat::from_line("FBFBBFFRLR").row());
        assert_eq!(70, Seat::from_line("BFFFBBFRRR").row());
        assert_eq!(14, Seat::from_line("FFFBBBFRRR").row());
        assert_eq!(102, Seat::from_line("BBFFBBFRLL").row());
    }

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
