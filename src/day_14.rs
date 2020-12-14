//! This is my solution for [Advent of Code - Day 14](https://adventofcode.com/2020/day/14) -
//! _Docking Data_
//!
//! This was themed around bitwise operations. The challenge was mostly parsing the puzzle
//! description into the bitwise operations needed. This was the first time I needed an Either
//! implementation rather than just using an enum as I needed to be able to store the current Mask
//! in a variable that is explicitly a Mask rather than an Instruction that could be either a Mask
//! or a Mem.

use std::fs;
use regex::Regex;
use im::{HashMap, HashSet};
use either::Either;
use either::Either::*;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-14-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 14.
pub fn run() {
    let contents = fs::read_to_string("res/day-14-input").expect("Failed to read file");

    let memory = run_program_v1(contents.as_str());
    let sum = sum_memory(memory);
    println!("The sum of memory values after running the program v1 is: {}", sum);

    let memory = run_program_v2(contents.as_str());
    let sum = sum_memory(memory);
    println!("The sum of memory values after running the program v2 is: {}", sum);
}

/// Representing an input line that overwrites the current bitmask, see [`parse_line`].
#[derive(Debug, Eq, PartialEq)]
struct Mask { mask: usize, data: usize }

/// Represents an input line that updates the current memory values, see [`parse_line`].
#[derive(Debug, Eq, PartialEq)]
struct Mem { address: usize, value: usize }

/// Parse a line from the puzzle input into structured data
///
/// A line will be of one of the two following formats:
/// * `mask = 000000000000000000000000000000X1001X`
/// * `mem[8] = 11`
///
/// ## Masks
/// For both parts of the puzzle the mask has two uses, where the character is a `0 `or `1` it
/// should be treated a raw data that will in someway override other input, and `X` will be used as
/// the mask. It is easier to store this as two bitmaps, one for the data and one for the mask, as
/// these are used separately.
///
/// ## Memory Updates
/// Whilst the two parts use the mask to modify where/what actually gets written `mem[8] = 11`
/// should be interpreted as address = 8, value = 11.
///
/// # Examples from Tests
/// ```
/// assert_eq!(
///     Left(Mask {
///         mask: 0b111111111111111111111111111111111111,
///         data: 0b000000000000000000000000000000000000,
///     }),
///     parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
/// );
/// assert_eq!(
///     Left(Mask {
///         mask: 0b111111111111111111111111111110111101,
///         data: 0b000000000000000000000000000001000000,
///     }),
///     parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
/// );
///
/// assert_eq!(
///     Right(Mem { address: 8, value: 11 }),
///     parse_line("mem[8] = 11")
/// );
/// assert_eq!(
///     Right(Mem { address: 7, value: 101 }),
///     parse_line("mem[7] = 101")
/// );
/// assert_eq!(
///     Right(Mem { address: 8, value: 0 }),
///     parse_line("mem[8] = 0")
/// );
/// ```
fn parse_line(line: &str) -> Either<Mask, Mem> {
    let mut parts = line.split(" = ");
    let inst = parts.next().expect("Invalid line");
    let value = parts.next().expect("Invalid line");

    if inst == "mask" {
        let (mask, data) =
            value.chars().fold(
                (0usize, 0usize),
                |(mask, data), char| (
                    mask << 1 | if char == 'X' { 1 } else { 0 },
                    data << 1 | if char == '1' { 1 } else { 0 }
                ),
            );

        Left(Mask { mask, data })
    } else {
        let re = Regex::new(r"^mem\[(\d+)]$").unwrap();

        match re.captures(inst) {
            Some(cap) => Right(Mem {
                address: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                value: value.parse::<usize>().unwrap(),
            }),
            None => panic!("Invalid line")
        }
    }
}

/// Takes the string input and returns the memory state after that has been interpreted using the
/// part 1 protocol
///
/// > The current bitmask is applied to values immediately before they are written to memory: a 0 or
/// > 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value
/// > unchanged.
///
/// # Example from Tests
/// ```
/// let mut expected: HashMap<usize, usize> = HashMap::new();
///
/// let program_1 = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11";
///
/// expected.insert(8, 73);
/// assert_eq!(expected, run_program_v1(program_1));
///
/// let program_2 = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// mem[8] = 11
/// mem[7] = 101
/// mem[8] = 0";
///
/// expected.insert(7, 101);
/// expected.insert(8, 64);
/// let memory = run_program_v1(program_2);
///
/// assert_eq!(expected, memory);
///
/// assert_eq!(165usize, sum_memory(memory));
/// ```
fn run_program_v1(program: &str) -> HashMap<usize, usize> {
    let mut memory = HashMap::new();
    let mut current_mask = Mask { mask: 0, data: 0 };

    for line in program.lines() {
        match parse_line(line) {
            Left(Mask { mask, data }) => current_mask = Mask { mask, data },
            Right(Mem { address, value }) => {
                memory.insert(
                    address,
                    value & current_mask.mask | current_mask.data,
                );
            }
        }
    }

    return memory;
}

/// Takes the string input and returns the memory state after that has been interpreted using the
/// part 2 protocol.
///
/// > Immediately before a value is written to memory, each bit in the bitmask modifies the
/// > corresponding bit of the destination memory address in the following way:
/// > - If the bitmask bit is 0, the corresponding memory address bit is unchanged.
/// > - If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
/// > - If the bitmask bit is X, the corresponding memory address bit is floating.
/// >
/// > A floating bit is not connected to anything and instead fluctuates unpredictably. In practice,
/// > this means the floating bits will take on all possible values, potentially causing many memory
/// > addresses to be written all at once!
///
/// The set of addresses a mask will write to is given by [`explode_addresses`]
///
/// # Example from Tests
/// ```
/// let program = "mask = 000000000000000000000000000000X1001X
/// mem[42] = 100
/// mask = 00000000000000000000000000000000X0XX
/// mem[26] = 1";
///
/// let memory = run_program_v2(program);
/// assert_eq!(208usize, sum_memory(memory));
/// ```
fn run_program_v2(program: &str) -> HashMap<usize, usize> {
    let mut memory = HashMap::new();
    let mut current_mask = Mask { mask: 0, data: 0 };

    for line in program.lines() {
        match parse_line(line) {
            Left(Mask { mask, data }) => current_mask = Mask { mask, data },
            Right(Mem { address, value }) =>
                for address in explode_addresses(&current_mask, address) {
                    memory.insert(address, value);
                },
        }
    }

    return memory;
}

/// Because floating bits can take on any value, this returns all the addresses that a given mask
/// applied to the input address refers to.
///
/// 1. The base address is the address where all the `X` values in the mask are `0`. Additionally
///    bits where the mask data is 1 all should be 1 for all addresses in the final output i.e.
///    `(input | mask.data) & !mask.mask`
/// 2. Iterate through the bits, and where the mask is `X` add an additional address to each of the
///    existing combinations for the address where that bit is `1` rather than `0`, so the set
///    doubles in size each time we encounter an `X`. With some boiler plate as the existing set
///    can't be appended to as it's being iterated.
///
/// # Examples from Tests
/// ```
/// let expected: HashSet<usize> = vec!(26usize, 27usize, 58usize, 59usize).into_iter().collect();
/// assert_eq!(
///     expected,
///     explode_addresses(
///         &Mask {
///             mask: 0b000000000000000000000000000000100001,
///             data: 0b000000000000000000000000000000010010,
///         },
///         42,
///     )
/// );
///
/// let expected: HashSet<usize> =
///     vec!(16usize, 17usize, 18usize, 19usize, 24usize, 25usize, 26usize, 27usize)
///         .into_iter().collect();
/// assert_eq!(
///     expected,
///     explode_addresses(
///         &parse_line("mask = 00000000000000000000000000000000X0XX")
///             .expect_left("Failed to parse as mask"),
///         26,
///     )
/// );
/// ```
fn explode_addresses(mask: &Mask, input: usize) -> HashSet<usize> {
    let mut addresses = HashSet::new();
    addresses.insert((input | mask.data) & !mask.mask);

    for i in 0..36 {
        if (1 << i) & mask.mask != 0 {
            let mut new_addresses = HashSet::new();

            for &address in addresses.iter() {
                new_addresses.insert(address | (1 << i));
            }

            for &new_address in new_addresses.iter() {
                addresses.insert(new_address);
            };
        }
    }

    addresses
}

/// Sum a memory snapshot
///
/// Both puzzle parts finally sum all the memory registers into a single number as the expected
/// answer. Extracted into a function to avoid repetition.
fn sum_memory(memory: HashMap<usize, usize>) -> usize {
    memory.iter().map(|(_, v)| *v).sum()
}

#[cfg(test)]
mod tests {
    use day_14::{parse_line, Mask, Mem, run_program_v1, sum_memory, explode_addresses, run_program_v2};
    use either::Either::*;
    use im::{HashMap, HashSet};

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse() {
        assert_eq!(
            Left(Mask {
                mask: 0b111111111111111111111111111111111111,
                data: 0b000000000000000000000000000000000000,
            }),
            parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
        );
        assert_eq!(
            Left(Mask {
                mask: 0b111111111111111111111111111110111101,
                data: 0b000000000000000000000000000001000000,
            }),
            parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        );
        assert_eq!(
            Right(Mem { address: 8, value: 11 }),
            parse_line("mem[8] = 11")
        );
        assert_eq!(
            Right(Mem { address: 7, value: 101 }),
            parse_line("mem[7] = 101")
        );
        assert_eq!(
            Right(Mem { address: 8, value: 0 }),
            parse_line("mem[8] = 0")
        );
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_run_program_v1() {
        let mut expected: HashMap<usize, usize> = HashMap::new();

        let program_1 = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11";
        expected.insert(8, 73);
        assert_eq!(expected, run_program_v1(program_1));

        let program_2 = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        expected.insert(7, 101);
        expected.insert(8, 64);
        let memory = run_program_v1(program_2);
        assert_eq!(expected, memory);

        assert_eq!(165usize, sum_memory(memory));
    }

    #[test]
    fn can_explode_addresses() {
        let expected: HashSet<usize> = vec!(26usize, 27usize, 58usize, 59usize).into_iter().collect();

        assert_eq!(
            expected,
            explode_addresses(
                &Mask {
                    mask: 0b000000000000000000000000000000100001,
                    data: 0b000000000000000000000000000000010010,
                },
                42,
            )
        );

        let expected: HashSet<usize> =
            vec!(16usize, 17usize, 18usize, 19usize, 24usize, 25usize, 26usize, 27usize)
                .into_iter().collect();

        assert_eq!(
            expected,
            explode_addresses(
                &parse_line("mask = 00000000000000000000000000000000X0XX")
                    .expect_left("Failed to parse as mask"),
                26,
            )
        );
    }

    #[test]
    fn can_run_program_v2() {
        let program = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let memory = run_program_v2(program);

        assert_eq!(208usize, sum_memory(memory));
    }
}
