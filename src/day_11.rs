use std::fs;
use day_11::Seat::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Seat {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid<T> {
    row_length: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn new(row_length: usize) -> Grid<T> {
        Grid {
            row_length,
            data: Vec::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.row_length { return None };
        self.data.get(self.row_length * y + x)
    }

    fn insert(&mut self, x: usize, y: usize, value: T) -> () {
        assert!(
            x < self.row_length,
            format!("x = {} is out of bounds for Grid with row size {}", x, self.row_length)
        );
        self.data.insert(self.row_length * y + x, value)
    }

    fn size(&self) -> (usize, usize) {
        (self.row_length, (self.data.len() - 1) / self.row_length + 1)
    }
}

pub fn run() {
    let contents = fs::read_to_string("res/day-11-input").expect("Failed to read file");
    let grid = parse_grid(contents.as_str());

    let adjacent_count = count_stable_adjacent_occupation(&grid);
    println!("Once adjacent model has stabilised, there are {} occupied seats", adjacent_count);

    let visible_count = count_stable_visible_occupation(&grid);
    println!("Once visible model has stabilised, there are {} occupied seats", visible_count);
}

fn parse_grid(input: &str) -> Grid<Seat> {
    let row_length = input.lines().next().unwrap().len();

    let mut grid = Grid::new(row_length);

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => grid.insert(x, y, FLOOR),
                'L' => grid.insert(x, y, EMPTY),
                '#' => grid.insert(x, y, OCCUPIED),
                _ => panic!("Invalid char")
            }
        }
    }

    grid
}

fn lookup_surrounds(grid: &Grid<Seat>, x: usize, y: usize) -> Vec<Seat> {
    vec!(
        (x.checked_sub(1), y.checked_sub(1)), (Some(x), y.checked_sub(1)), (x.checked_add(1), y.checked_sub(1)),
        (x.checked_sub(1), Some(y)), /*                                 */ (x.checked_add(1), Some(y)),
        (x.checked_sub(1), y.checked_add(1)), (Some(x), y.checked_add(1)), (x.checked_add(1), y.checked_add(1))
    )
        .iter()
        .flat_map(|(x1, y1)| match (*x1, *y1) {
            (Some(x), Some(y)) => grid.get(x, y).map(|s| *s),
            _ => None
        })
        .collect()
}

fn lookup_visible_seats(grid: &Grid<Seat>, x: usize, y: usize) -> Vec<Seat> {
    vec!(
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), /*     */ (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ).iter().flat_map(|(dx, dy)| lookup_visible_seat(grid, x, y, *dx, *dy)).collect()
}

fn lookup_visible_seat(grid: &Grid<Seat>, x: usize, y: usize, dx: isize, dy: isize) -> Option<Seat> {
    let (x1, y1) = ((x as isize).checked_add(dx), (y as isize).checked_add(dy));

    let adjacent_seat = match (x1, y1) {
        (Some(x), Some(y)) if x >= 0 && y >= 0 => grid.get(x as usize, y as usize),
        _ => None,
    };

    match adjacent_seat {
        Some(FLOOR) => lookup_visible_seat(grid, x1.unwrap() as usize, y1.unwrap() as usize, dx, dy),
        Some(other) => Some(*other),
        _ => None
    }
}

fn iterate_cell <F> (grid: &Grid<Seat>, x: usize, y: usize, mapper: &F, occupation_threshold: usize) -> Option<Seat> where
    F: Fn(&Grid<Seat>, usize, usize) -> Vec<Seat>
{
    match grid.get(x, y) {
        Some(FLOOR) => Some(FLOOR),
        Some(EMPTY) =>
            if mapper(grid, x, y).iter().filter(|&&s| s == OCCUPIED).count() == 0 {
                Some(OCCUPIED)
            } else {
                Some(EMPTY)
            },
        Some(OCCUPIED) =>
            if mapper(grid, x, y).iter().filter(|&&s| s == OCCUPIED).count() >= occupation_threshold {
                Some(EMPTY)
            } else {
                Some(OCCUPIED)
            }
        None => None
    }
}

fn iterate_grid<F>(grid: &Grid<Seat>, mapper: &F, occupation_threshold: usize) -> (Grid<Seat>, usize) where
    F: Fn(&Grid<Seat>, usize, usize) -> Vec<Seat>
{
    let (x_max, y_max) = grid.size();
    let mut new_grid = Grid::new(grid.row_length);
    let mut mod_count = 0;

    for y in 0..y_max {
        for x in 0..x_max {
            let new_seat = iterate_cell(grid, x, y, mapper, occupation_threshold).unwrap();
            new_grid.insert(x, y, new_seat);
            if grid.get(x, y) != Some(&new_seat) {
                mod_count = mod_count + 1
            }
        }
    }

    (new_grid, mod_count)
}

fn count_stable_adjacent_occupation(grid: &Grid<Seat>) -> usize {
    let (new_grid, mod_count) = iterate_grid(grid, &lookup_surrounds, 4);
    if mod_count == 0 {
        new_grid.data.iter().filter(|s| **s == OCCUPIED).count()
    } else {
        count_stable_adjacent_occupation(&new_grid)
    }
}

fn count_stable_visible_occupation(grid: &Grid<Seat>) -> usize {
    let (new_grid, mod_count) = iterate_grid(grid, &lookup_visible_seats, 5);
    if mod_count == 0 {
        new_grid.data.iter().filter(|s| **s == OCCUPIED).count()
    } else {
        count_stable_visible_occupation(&new_grid)
    }
}

#[cfg(test)]
mod tests {
    use day_11::Seat::*;
    use day_11::{parse_grid, Grid, lookup_surrounds, Seat, iterate_cell, iterate_grid, count_stable_adjacent_occupation, lookup_visible_seat, lookup_visible_seats, count_stable_visible_occupation};

    //noinspection SpellCheckingInspection
    fn input() -> &'static str {
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
    }

    fn tiny_grid() -> Grid<Seat> {
        Grid {
            row_length: 3,
            data: vec!(
                EMPTY, FLOOR, EMPTY,
                EMPTY, OCCUPIED, FLOOR,
                EMPTY, EMPTY, FLOOR
            ),
        }
    }

    #[test]
    fn can_parse<'a>() {
        assert_eq!(
            Grid {
                row_length: 10,
                data: vec!(
                    EMPTY, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY,
                    EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY,
                    EMPTY, FLOOR, EMPTY, FLOOR, EMPTY, FLOOR, FLOOR, EMPTY, FLOOR, FLOOR,
                    EMPTY, EMPTY, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY,
                    EMPTY, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY,
                    EMPTY, FLOOR, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY,
                    FLOOR, FLOOR, EMPTY, FLOOR, EMPTY, FLOOR, FLOOR, FLOOR, FLOOR, FLOOR,
                    EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                    EMPTY, FLOOR, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, FLOOR, EMPTY,
                    EMPTY, FLOOR, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY
                ),
            },
            parse_grid(input())
        )
    }

    #[test]
    fn can_size_grid() {
        assert_eq!((3, 3), tiny_grid().size());
        assert_eq!((10, 10), parse_grid(input()).size());
        assert_eq!((3, 3), parse_grid("###\n###\n#").size());
        assert_eq!((3, 3), parse_grid("###\n###\n##").size());
    }

    #[test]
    fn can_lookup_surrounds() {
        let sample_grid = tiny_grid();

        assert_eq!(
            vec!(EMPTY, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, EMPTY, FLOOR),
            lookup_surrounds(&sample_grid, 1, 1)
        );

        assert_eq!(
            vec!(FLOOR, EMPTY, OCCUPIED),
            lookup_surrounds(&sample_grid, 0, 0)
        );

        assert_eq!(
            vec!(FLOOR, EMPTY, OCCUPIED, EMPTY, FLOOR),
            lookup_surrounds(&sample_grid, 2, 1)
        );
    }

    #[test]
    fn can_iterate_cell() {
        assert_eq!(Some(EMPTY), iterate_cell(&tiny_grid(), 0, 0, &lookup_surrounds, 4));
        assert_eq!(Some(OCCUPIED), iterate_cell(&tiny_grid(), 1, 1, &lookup_surrounds, 4));
        assert_eq!(Some(FLOOR), iterate_cell(&tiny_grid(), 2, 2, &lookup_surrounds, 4));

        let empty_grid = parse_grid("L.L\n.L.\nL.L");
        assert_eq!(Some(OCCUPIED), iterate_cell(&empty_grid, 1, 1, &lookup_surrounds, 4));
        assert_eq!(Some(OCCUPIED), iterate_cell(&empty_grid, 0, 0, &lookup_surrounds, 4));

        let full_grid = parse_grid("#.#\n.#.\n#.#");
        assert_eq!(Some(EMPTY), iterate_cell(&full_grid, 1, 1, &lookup_surrounds, 4));
        assert_eq!(Some(OCCUPIED), iterate_cell(&full_grid, 0, 0, &lookup_surrounds, 4));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_iterate_grid() {
        let iter_1_expected = parse_grid("#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##");
        let iter_2_expected = parse_grid("#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##");
        let iter_3_expected = parse_grid("#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##");
        let iter_4_expected = parse_grid("#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##");
        let iter_5_expected = &parse_grid("#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##");


        let (iter_1_actual, iter_1_count) = iterate_grid(&parse_grid(input()), &lookup_surrounds, 4);
        let (iter_2_actual, iter_2_count) = iterate_grid(&iter_1_actual, &lookup_surrounds, 4);
        let (iter_3_actual, _iter_3_count) = iterate_grid(&iter_2_actual, &lookup_surrounds, 4);
        let (iter_4_actual, _iter_4_count) = iterate_grid(&iter_3_actual, &lookup_surrounds, 4);
        let (iter_5_actual, _iter_5_count) = iterate_grid(&iter_4_actual, &lookup_surrounds, 4);
        let (iter_6_actual, iter_6_count) = iterate_grid(&iter_5_actual, &lookup_surrounds, 4);

        assert_eq!((iter_1_expected, 71usize), (iter_1_actual, iter_1_count));
        assert_eq!((iter_2_expected, 51usize), (iter_2_actual, iter_2_count));
        assert_eq!(iter_3_expected, iter_3_actual);
        assert_eq!(iter_4_expected, iter_4_actual);
        assert_eq!(iter_5_expected, &iter_5_actual);
        assert_eq!((iter_5_expected, 0usize), (&iter_6_actual, iter_6_count));
    }

    #[test]
    fn can_count_stable_adjacent_occupation() {
        assert_eq!(1usize, count_stable_adjacent_occupation(&tiny_grid()));
        assert_eq!(37usize, count_stable_adjacent_occupation(&parse_grid(input())));
    }

    #[test]
    fn can_look_up_visible_seat() {
        assert_eq!(Some(EMPTY), lookup_visible_seat(&parse_grid("#L"), 0, 0, 1, 0));
        assert_eq!(Some(OCCUPIED), lookup_visible_seat(&parse_grid("#L"), 1, 0, -1, 0));

        assert_eq!(Some(EMPTY), lookup_visible_seat(&parse_grid("#.L"), 0, 0, 1, 0));
        assert_eq!(Some(OCCUPIED), lookup_visible_seat(&parse_grid("#.L"), 2, 0, -1, 0));

        assert_eq!(None, lookup_visible_seat(&parse_grid("#.."), 0, 0, 1, 0));
        assert_eq!(None, lookup_visible_seat(&parse_grid("..L"), 2, 0, -1, 0));
    }

    #[test]
    fn can_lookup_visible_seats() {
        assert_eq!(
            vec!(OCCUPIED, OCCUPIED, OCCUPIED, OCCUPIED, OCCUPIED, OCCUPIED, OCCUPIED, OCCUPIED),
            lookup_visible_seats(
                &parse_grid(
                    ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."
                ),
                3, 4,
            )
        );

        assert_eq!(
            vec!(EMPTY),
            lookup_visible_seats(
                &parse_grid(
                    ".............
.L.L.#.#.#.#.
............."
                ),
                1, 1,
            )
        );

        let empty: Vec<Seat> = vec!();

        assert_eq!(
            empty,
            lookup_visible_seats(
                &parse_grid(
                    ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##."
                ),
                3, 3,
            )
        );
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_iterate_part_2() {
        let iter_1_expected = parse_grid("#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##");
        let iter_2_expected = parse_grid("#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#");
        let iter_3_expected = parse_grid("#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#");
        let iter_4_expected = parse_grid("#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#");
        let iter_5_expected = parse_grid("#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#");

        let iter_6_expected = &parse_grid("#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#");


        let (iter_1_actual, iter_1_count) = iterate_grid(&parse_grid(input()), &lookup_visible_seats, 5);
        let (iter_2_actual, iter_2_count) = iterate_grid(&iter_1_actual, &lookup_visible_seats, 5);
        let (iter_3_actual, _iter_3_count) = iterate_grid(&iter_2_actual, &lookup_visible_seats, 5);
        let (iter_4_actual, _iter_4_count) = iterate_grid(&iter_3_actual, &lookup_visible_seats, 5);
        let (iter_5_actual, _iter_5_count) = iterate_grid(&iter_4_actual, &lookup_visible_seats, 5);
        let (iter_6_actual, _iter_6_count) = iterate_grid(&iter_5_actual, &lookup_visible_seats, 5);
        let (iter_7_actual, iter_7_count) = iterate_grid(&iter_6_actual, &lookup_visible_seats, 5);

        assert_eq!((iter_1_expected, 71usize), (iter_1_actual, iter_1_count));
        assert_eq!((iter_2_expected, 64usize), (iter_2_actual, iter_2_count));
        assert_eq!(iter_3_expected, iter_3_actual);
        assert_eq!(iter_4_expected, iter_4_actual);
        assert_eq!(iter_5_expected, iter_5_actual);
        assert_eq!(iter_6_expected, &iter_6_actual);
        assert_eq!((iter_6_expected, 0usize), (&iter_7_actual, iter_7_count));
    }

    #[test]
    fn can_count_stable_visible_occupation() {
        assert_eq!(1usize, count_stable_visible_occupation(&tiny_grid()));
        assert_eq!(26usize, count_stable_visible_occupation(&parse_grid(input())));
    }
}
