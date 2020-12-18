//! This is my solution for [Advent of Code - Day 17](https://adventofcode.com/2020/day/17) -
//! _Conway Cubes_
//!
//! Implement [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in 3D
//! and then 4D space. Today features lots of nested for loops. Whilst there was some code reuse,
//! it turned out to be simpler to just reimplement the same ideas from the 3D version when
//! expanding to four dimensions.
//!
//! __Part 1__ - [`ThreeDGrid`], [`parse_input_3d`], [`iterate_grid_3d`].
//!
//! __Part 2__ - [`FourDGrid`], [`parse_input_4d`], [`iterate_grid_4d`].

use std::fs;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fmt;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-17-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 17.
pub fn run() {
    let contents = fs::read_to_string("res/day-17-input").expect("Failed to read file");
    let mut grid = parse_input_3d(contents.as_str());
    for _ in 0..6 {
        grid = iterate_grid_3d(&grid)
    }
    println!("After the 6 step boot cycle there are {} active cells in the 3d grid", grid.count_active());

    let mut grid = parse_input_4d(contents.as_str());
    for _ in 0..6 {
        grid = iterate_grid_4d(&grid)
    }
    println!("After the 6 step boot cycle there are {} active cells in the 4d grid", grid.count_active());
}

/// Represents a three dimensional infinite grid.
#[derive(Clone)]
struct ThreeDGrid {
    /// Holds the grid data
    grid: HashMap<isize, HashMap<isize, HashSet<isize>>>,
    /// Lower bound of data in the x dimension
    x_min: isize,
    /// Upper bound of data in the x dimension
    x_max: isize,
    /// Lower bound of data in the y dimension
    y_min: isize,
    /// Upper bound of data in the y dimension
    y_max: isize,
    /// Lower bound of data in the z dimension
    z_min: isize,
    /// Upper bound of data in the z dimension
    z_max: isize,
}

impl ThreeDGrid {
    fn new() -> ThreeDGrid {
        ThreeDGrid {
            grid: HashMap::new(),
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            z_min: 0,
            z_max: 0,
        }
    }

    /// Get the state of a specific cell in the grid
    fn is_cell_active(&self, x: isize, y: isize, z: isize) -> bool {
        self.grid.get(&z)
            .map(|plane| plane.get(&y)).flatten()
            .map(|column| column.contains(&x))
            .unwrap_or(false)
    }

    /// Set the state of a specific state in the grid
    fn toggle_cell(&mut self, x: isize, y: isize, z: isize, active: bool) {
        if !self.grid.contains_key(&z) {
            self.grid.insert(z, HashMap::new());
        }

        let plane = self.grid.get_mut(&z).expect("Ensured existence above");

        if !plane.contains_key(&y) {
            plane.insert(y, HashSet::new());
        }

        let column = plane.get_mut(&y).expect("Ensured existence above");

        if active {
            column.insert(x);
        } else {
            column.remove(&x);
        }

        if active {
            self.x_min = self.x_min.min(x);
            self.x_max = self.x_max.max(x);

            self.y_min = self.y_min.min(y);
            self.y_max = self.y_max.max(y);

            self.z_min = self.z_min.min(z);
            self.z_max = self.z_max.max(z);
        }
    }

    /// Returns the number of active cells in the grid
    fn count_active(&self) -> usize {
        self.grid.iter()
            .flat_map(|(_, plane)| plane.iter())
            .map(|(_, column)| column.len())
            .sum()
    }

    /// How many of the 26 grid cells adjacent to the target cell are active
    ///
    /// > Each cube only ever considers its neighbors: any of the 26 other cubes where any of their
    /// > coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors
    /// >  include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
    fn count_adjacent(&self, x: isize, y: isize, z: isize) -> usize {
        let mut sum = 0;
        for z1 in (z - 1)..=(z + 1) {
            for y1 in (y - 1)..=(y + 1) {
                for x1 in (x - 1)..=(x + 1) {
                    if z1 == z && y1 == y && x1 == x { continue }
                    if self.is_cell_active(x1, y1, z1) {
                        sum = sum + 1
                    }
                }
            }
        }

        sum.to_owned()
    }
}

/// Render a 2D grid for each active z coordinate
impl Debug for ThreeDGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = "".to_owned();

        for z in (self.z_min)..=(self.z_max) {
            out = out + format!("z={}\n  ", z).as_str();
            for x in (self.x_min)..=(self.x_max) {
                out = out + format!("{:2}", x).as_str();
            }
            out = out + "\n";
            for y in (self.y_min)..=(self.y_max) {
                out = out + format!("{:2}", y).as_str();
                for x in (self.x_min)..=(self.x_max) {
                    out = out + if self.is_cell_active(x, y, z) { " #" } else { " ." }
                }
                out = out + "\n";
            }
        }

        f.write_str(out.as_str())
    }
}

/// Represents a three dimensional infinite grid.
#[derive(Clone)]
struct FourDGrid {
    /// Implements the grid as a map of nested 3D grids.
    grid: HashMap<isize, ThreeDGrid>,
    /// Upper bound of data in the w dimension
    w_min: isize,
    /// Upper bound of data in the w dimension
    w_max: isize,
}

impl FourDGrid {
    fn new() -> FourDGrid {
        FourDGrid {
            grid: HashMap::new(),
            w_min: 0,
            w_max: 0
        }
    }

    /// Get the state of a specific cell in the grid
    fn is_cell_active(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        self.grid.get(&w).map_or(false, |cube| cube.is_cell_active(x, y, z))
    }

    /// Set the state of a specific state in the grid
    ///
    /// # Examples from Tests
    /// ```
    /// let mut grid = ThreeDGrid::new();
    ///
    /// assert_eq!(false, grid.is_cell_active(1, 0, 0));
    /// assert_eq!(false, grid.is_cell_active(0, 1, 0));
    /// assert_eq!(false, grid.is_cell_active(0, 0, 1));
    ///
    /// assert_eq!(0usize, grid.count_active());
    ///
    /// assert_eq!(0isize, grid.x_min);
    /// assert_eq!(0isize, grid.x_max);
    /// assert_eq!(0isize, grid.y_min);
    /// assert_eq!(0isize, grid.y_max);
    /// assert_eq!(0isize, grid.z_min);
    /// assert_eq!(0isize, grid.z_max);
    ///
    /// grid.toggle_cell(1, 0, 0, true);
    ///
    /// assert_eq!(true, grid.is_cell_active(1, 0, 0));
    /// assert_eq!(false, grid.is_cell_active(0, 1, 0));
    /// assert_eq!(false, grid.is_cell_active(0, 0, 1));
    ///
    /// assert_eq!(1usize, grid.count_active());
    ///
    /// grid.toggle_cell(0, 1, 0, true);
    /// grid.toggle_cell(0, 0, 1, false);
    ///
    /// assert_eq!(true, grid.is_cell_active(1, 0, 0));
    /// assert_eq!(true, grid.is_cell_active(0, 1, 0));
    /// assert_eq!(false, grid.is_cell_active(0, 0, 1));
    ///
    /// assert_eq!(2usize, grid.count_active());
    ///
    /// grid.toggle_cell(1, 0, 0, false);
    /// grid.toggle_cell(0, 1, 0, true);
    ///
    /// assert_eq!(false, grid.is_cell_active(1, 0, 0));
    /// assert_eq!(true, grid.is_cell_active(0, 1, 0));
    /// assert_eq!(false, grid.is_cell_active(0, 0, 1));
    ///
    /// assert_eq!(1usize, grid.count_active());
    ///
    /// assert_eq!(0isize, grid.x_min);
    /// assert_eq!(1isize, grid.x_max);
    /// assert_eq!(0isize, grid.y_min);
    /// assert_eq!(1isize, grid.y_max);
    /// assert_eq!(0isize, grid.z_min);
    /// assert_eq!(0isize, grid.z_max);
    /// ```
    fn toggle_cell(&mut self, x: isize, y: isize, z: isize, w: isize, active: bool) {
        if !self.grid.contains_key(&w) {
            self.grid.insert(w, ThreeDGrid::new());
        }

        let cube = self.grid.get_mut(&w).expect("Ensured existence above");
        
        cube.toggle_cell(x, y, z, active);
        
        if active {
            self.w_min = self.w_min.min(w);
            self.w_max = self.w_max.max(w);
        }
    }

    /// Returns the number of active cells in the grid
    fn count_active(&self) -> usize {
        self.grid.iter().map(|(_, cube)| cube.count_active()).sum()
    }

    /// How many of the 80 grid cells adjacent to the target cell are active
    ///
    /// # Examples frm Tests
    /// ```
    /// let input = ".#.\n..#\n###";
    /// let grid = parse_input_3d(input);
    ///
    /// assert_eq!(1usize, grid.count_adjacent(0,0,0));
    /// assert_eq!(5usize, grid.count_adjacent(1,1,0));
    /// assert_eq!(2usize, grid.count_adjacent(2,2,0));
    /// assert_eq!(1usize, grid.count_adjacent(3,3,0));
    /// ```
    fn count_adjacent(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut sum = 0;
        for w1 in (w - 1)..=(w + 1) {
            for z1 in (z - 1)..=(z + 1) {
                for y1 in (y - 1)..=(y + 1) {
                    for x1 in (x - 1)..=(x + 1) {
                        if z1 == z && y1 == y && x1 == x && w1 == w { continue }
                        if self.is_cell_active(x1, y1, z1, w1) {
                            sum = sum + 1
                        }
                    }
                }
            }
        }
        
        sum.to_owned()
    }

    /// Return the bounds of the data in the grid by querying the inner 3D grids
    fn get_bounds(&self) -> ((isize, isize),(isize, isize),(isize, isize),(isize, isize)) {
        self.grid.iter().fold(
            ((0isize, 0isize), (0isize, 0isize), (0isize, 0isize), (self.w_min, self.w_max)),
            |((x_min, x_max),(y_min, y_max),(z_min, z_max),(w_min, w_max)), (_, cube)| {
                (
                    (x_min.min(cube.x_min), x_max.max(cube.x_max)),
                    (y_min.min(cube.y_min), y_max.max(cube.y_max)),
                    (z_min.min(cube.z_min), z_max.max(cube.z_max)),
                    (w_min, w_max),
                )
            } 
            
        )
        
        
    }
}

/// Build the initial 3D Grid from the puzzle input.
///
/// > In the initial state of the pocket dimension, almost all cubes start inactive. The only
/// > exception to this is a small flat region of cubes (your puzzle input); the cubes in this
/// > region start in the specified active (#) or inactive (.) state.
///
/// # Example from Tests
/// ```
/// let input = ".#.\n..#\n###";
/// let grid = parse_input_3d(input);
///
/// assert_eq!(true, grid.is_cell_active(1, 0, 0));
/// assert_eq!(true, grid.is_cell_active(2, 1, 0));
/// assert_eq!(true, grid.is_cell_active(0, 2, 0));
/// assert_eq!(true, grid.is_cell_active(1, 2, 0));
/// assert_eq!(true, grid.is_cell_active(2, 2, 0));
///
/// assert_eq!(5usize, grid.count_active());
///
/// assert_eq!(0isize, grid.x_min);
/// assert_eq!(2isize, grid.x_max);
/// assert_eq!(0isize, grid.y_min);
/// assert_eq!(2isize, grid.y_max);
/// assert_eq!(0isize, grid.z_min);
/// assert_eq!(0isize, grid.z_max);/
/// ```
fn parse_input_3d(input: &str) -> ThreeDGrid {
    let mut grid = ThreeDGrid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.toggle_cell(x as isize, y as isize, 0, char == '#')
        }
    }

    grid
}

/// Build the initial 4D Grid from the puzzle input.
///
/// > Even though the pocket dimension is 4-dimensional, this initial state represents a small
/// > 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the
/// > 4-dimensional space.)
///
/// # Example from Test
/// ```
/// let input = ".#.\n..#\n###";
/// let mut grid = parse_input_4d(input);
///
/// assert_eq!(true, grid.is_cell_active(1, 0, 0, 0));
/// assert_eq!(true, grid.is_cell_active(2, 1, 0, 0));
/// assert_eq!(true, grid.is_cell_active(0, 2, 0, 0));
/// assert_eq!(true, grid.is_cell_active(1, 2, 0, 0));
/// assert_eq!(true, grid.is_cell_active(2, 2, 0, 0));
///
/// assert_eq!(5usize, grid.count_active());
/// ```
fn parse_input_4d(input: &str) -> FourDGrid {
    let mut grid = FourDGrid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.toggle_cell(x as isize, y as isize, 0,  0,char == '#')
        }
    }

    grid
}

/// Produce the next grid by applying the rules of the game to the current gird. Solution to part 1.
///
/// > The energy source then proceeds to boot up by executing six cycles.
/// >
/// > During a cycle, all cubes simultaneously change their state according to the following rules:
/// >
/// > - If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains
/// >   active. Otherwise, the cube becomes inactive.
/// > - If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
/// >   Otherwise, the cube remains inactive.
///
/// # Examples from Test
/// ```
/// let input = ".#.\n..#\n###";
/// let mut grid = parse_input_3d(input);
///
/// grid = iterate_grid_3d(&grid);
///
/// assert_eq!(true, grid.is_cell_active(0, 1, -1));
/// assert_eq!(true, grid.is_cell_active(2, 2, -1));
/// assert_eq!(true, grid.is_cell_active(1, 3, -1));
/// assert_eq!(true, grid.is_cell_active(0, 1, 0));
/// assert_eq!(true, grid.is_cell_active(2, 1, 0));
/// assert_eq!(true, grid.is_cell_active(1, 2, 0));
/// assert_eq!(true, grid.is_cell_active(2, 2, 0));
/// assert_eq!(true, grid.is_cell_active(1, 3, 1));
/// assert_eq!(true, grid.is_cell_active(0, 1, 1));
/// assert_eq!(true, grid.is_cell_active(2, 2, 1));
/// assert_eq!(true, grid.is_cell_active(1, 3, 1));
///
/// assert_eq!(11usize, grid.count_active());
///
/// grid = iterate_grid_3d(&grid);
/// assert_eq!(21usize, grid.count_active());
///
/// grid = iterate_grid_3d(&grid);
/// assert_eq!(38usize, grid.count_active());
///
/// grid = iterate_grid_3d(&grid);
/// grid = iterate_grid_3d(&grid);
/// grid = iterate_grid_3d(&grid);
/// assert_eq!(112usize, grid.count_active());
/// ```
fn iterate_grid_3d(grid: &ThreeDGrid) -> ThreeDGrid {
    let mut new_grid = grid.clone();
    for z in (grid.z_min - 1)..=(grid.z_max + 1) {
        for y in (grid.y_min - 1)..=(grid.y_max + 1) {
            for x in (grid.x_min - 1)..=(grid.x_max + 1) {
                let adjacent = grid.count_adjacent(x, y, z);
                let active = if grid.is_cell_active(x, y, z) {
                    adjacent == 2 || adjacent == 3
                } else {
                    adjacent == 3
                };
                new_grid.toggle_cell(x, y, z, active)
            }
        }
    }

    new_grid
}

/// Produce the next grid by applying the rules of the game to the current gird. Solution to part 2.
///
/// > Furthermore, the same rules for cycle updating still apply: during each cycle, consider the
/// > number of active neighbors of each cube. See [`iterate_grid_3d`].
///
/// # Examples from Tests
/// ```
/// let input = ".#.\n..#\n###";
/// let mut grid = parse_input_4d(input);
///
/// assert_eq!(true, grid.is_cell_active(1, 0, 0, 0));
/// assert_eq!(true, grid.is_cell_active(2, 1, 0, 0));
/// assert_eq!(true, grid.is_cell_active(0, 2, 0, 0));
/// assert_eq!(true, grid.is_cell_active(1, 2, 0, 0));
/// assert_eq!(true, grid.is_cell_active(2, 2, 0, 0));
///
/// assert_eq!(5usize, grid.count_active());
///
/// grid = iterate_grid_4d(&grid);
/// assert_eq!(29usize, grid.count_active());
///
/// grid = iterate_grid_4d(&grid);
/// assert_eq!(60usize, grid.count_active());
///
/// grid = iterate_grid_4d(&grid);
/// grid = iterate_grid_4d(&grid);
/// grid = iterate_grid_4d(&grid);
/// grid = iterate_grid_4d(&grid);
/// assert_eq!(848usize, grid.count_active());
/// ```

fn iterate_grid_4d(grid: &FourDGrid) -> FourDGrid {
    let mut new_grid = grid.clone();
    let ((x_min, x_max),(y_min, y_max),(z_min, z_max),(w_min, w_max)) = grid.get_bounds();
    
    for w in (w_min - 1)..=(w_max + 1) {        
        for z in (z_min - 1)..=(z_max + 1) {
            for y in (y_min - 1)..=(y_max + 1) {
                for x in (x_min - 1)..=(x_max + 1) {
                    let adjacent = grid.count_adjacent(x, y, z, w);
                    let active = if grid.is_cell_active(x, y, z, w) {
                        adjacent == 2 || adjacent == 3
                    } else {
                        adjacent == 3
                    };
                    new_grid.toggle_cell(x, y, z, w, active)
                }
            }
        }
    }

    new_grid
}

#[cfg(test)]
mod tests {
    use day_17::{ThreeDGrid, parse_input_3d, iterate_grid_3d, parse_input_4d, iterate_grid_4d};

    #[test]
    fn can_parse() {
        let input = ".#.\n..#\n###";
        let grid = parse_input_3d(input);

        assert_eq!(true, grid.is_cell_active(1, 0, 0));
        assert_eq!(true, grid.is_cell_active(2, 1, 0));
        assert_eq!(true, grid.is_cell_active(0, 2, 0));
        assert_eq!(true, grid.is_cell_active(1, 2, 0));
        assert_eq!(true, grid.is_cell_active(2, 2, 0));

        assert_eq!(5usize, grid.count_active());

        assert_eq!(0isize, grid.x_min);
        assert_eq!(2isize, grid.x_max);
        assert_eq!(0isize, grid.y_min);
        assert_eq!(2isize, grid.y_max);
        assert_eq!(0isize, grid.z_min);
        assert_eq!(0isize, grid.z_max);
    }

    #[test]
    fn can_toggle_cell() {
        let mut grid = ThreeDGrid::new();
        assert_eq!(false, grid.is_cell_active(1, 0, 0));
        assert_eq!(false, grid.is_cell_active(0, 1, 0));
        assert_eq!(false, grid.is_cell_active(0, 0, 1));

        assert_eq!(0usize, grid.count_active());

        assert_eq!(0isize, grid.x_min);
        assert_eq!(0isize, grid.x_max);
        assert_eq!(0isize, grid.y_min);
        assert_eq!(0isize, grid.y_max);
        assert_eq!(0isize, grid.z_min);
        assert_eq!(0isize, grid.z_max);

        grid.toggle_cell(1, 0, 0, true);

        assert_eq!(true, grid.is_cell_active(1, 0, 0));
        assert_eq!(false, grid.is_cell_active(0, 1, 0));
        assert_eq!(false, grid.is_cell_active(0, 0, 1));
        assert_eq!(1usize, grid.count_active());

        grid.toggle_cell(0, 1, 0, true);
        grid.toggle_cell(0, 0, 1, false);

        assert_eq!(true, grid.is_cell_active(1, 0, 0));
        assert_eq!(true, grid.is_cell_active(0, 1, 0));
        assert_eq!(false, grid.is_cell_active(0, 0, 1));
        assert_eq!(2usize, grid.count_active());

        grid.toggle_cell(1, 0, 0, false);
        grid.toggle_cell(0, 1, 0, true);

        assert_eq!(false, grid.is_cell_active(1, 0, 0));
        assert_eq!(true, grid.is_cell_active(0, 1, 0));
        assert_eq!(false, grid.is_cell_active(0, 0, 1));
        assert_eq!(1usize, grid.count_active());

        assert_eq!(0isize, grid.x_min);
        assert_eq!(1isize, grid.x_max);
        assert_eq!(0isize, grid.y_min);
        assert_eq!(1isize, grid.y_max);
        assert_eq!(0isize, grid.z_min);
        assert_eq!(0isize, grid.z_max);
    }

    #[test]
    fn can_count_adjacent() {
        let input = ".#.\n..#\n###";
        let grid = parse_input_3d(input);

        assert_eq!(1usize, grid.count_adjacent(0,0,0));
        assert_eq!(5usize, grid.count_adjacent(1,1,0));
        assert_eq!(2usize, grid.count_adjacent(2,2,0));
        assert_eq!(1usize, grid.count_adjacent(3,3,0));
    }

    #[test]
    fn can_iterate() {
        let input = ".#.\n..#\n###";
        let mut grid = parse_input_3d(input);

        grid = iterate_grid_3d(&grid);


        assert_eq!(true, grid.is_cell_active(0, 1, -1));
        assert_eq!(true, grid.is_cell_active(2, 2, -1));
        assert_eq!(true, grid.is_cell_active(1, 3, -1));

        assert_eq!(true, grid.is_cell_active(0, 1, 0));
        assert_eq!(true, grid.is_cell_active(2, 1, 0));
        assert_eq!(true, grid.is_cell_active(1, 2, 0));
        assert_eq!(true, grid.is_cell_active(2, 2, 0));
        assert_eq!(true, grid.is_cell_active(1, 3, 1));

        assert_eq!(true, grid.is_cell_active(0, 1, 1));
        assert_eq!(true, grid.is_cell_active(2, 2, 1));
        assert_eq!(true, grid.is_cell_active(1, 3, 1));

        assert_eq!(11usize, grid.count_active());

        grid = iterate_grid_3d(&grid);
        assert_eq!(21usize, grid.count_active());

        grid = iterate_grid_3d(&grid);
        assert_eq!(38usize, grid.count_active());

        grid = iterate_grid_3d(&grid);
        grid = iterate_grid_3d(&grid);
        grid = iterate_grid_3d(&grid);

        assert_eq!(112usize, grid.count_active());
    }
    
    #[test]
    fn can_expand_to_4d() {
        let input = ".#.\n..#\n###";
        let mut grid = parse_input_4d(input);

        assert_eq!(true, grid.is_cell_active(1, 0, 0, 0));
        assert_eq!(true, grid.is_cell_active(2, 1, 0, 0));
        assert_eq!(true, grid.is_cell_active(0, 2, 0, 0));
        assert_eq!(true, grid.is_cell_active(1, 2, 0, 0));
        assert_eq!(true, grid.is_cell_active(2, 2, 0, 0));

        assert_eq!(5usize, grid.count_active());

        grid = iterate_grid_4d(&grid);
        assert_eq!(29usize, grid.count_active());

        grid = iterate_grid_4d(&grid);
        assert_eq!(60usize, grid.count_active());

        grid = iterate_grid_4d(&grid);
        grid = iterate_grid_4d(&grid);
        grid = iterate_grid_4d(&grid);
        grid = iterate_grid_4d(&grid);

        assert_eq!(848usize, grid.count_active());
    }
}
