//! This is my solution for [Advent of Code - Day 15](https://adventofcode.com/2020/day/15) -
//! _Rambunctious Recitation_
//!
//! This was themed around thinking about optimising how the data was stored. My original
//! implementation used a HashMap, but was much slower (~3s with a release build). After submitting
//! the solution from that there were some hints online that a vector would be much quicker, so I
//! tried that, first with a Vec<usize> (~1s), then with a Vec<u32> (~.8s) - probably not worth the
//! saving but also not worth undoing. It is still easily the longest runtime of my puzzles so far.
//!
//! All of the work is done in [`play_memory_game`], which worked for both parts. The main
//! awkwardness was eliminating out by 1 errors, but the tests highlighted all of those quickly.

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-15-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 15.
pub fn run() {
    let contents = "8,11,0,19,1,2";

    let result_2020 = play_memory_game(parse(contents), 2020);
    println!("The 2020th number is: {}", result_2020);

    let result_30m = play_memory_game(parse(contents), 30000000);
    println!("The 30,000,000th number is: {}", result_30m);
}

/// Parses the seed string into a usable Vec
///
/// # Examples from test
/// ```
/// assert_eq!(vec!(0u32, 3u32, 6u32), parse("0,3,6"));
/// assert_eq!(vec!(1u32, 2u32, 3u32), parse("1,2,3"));
/// ```
pub fn parse(input: &str) -> Vec<u32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

/// Solution to both parts
///
/// > In this game, the players take turns saying numbers. They begin by taking turns reading from a
/// > list of starting numbers (your puzzle input). Then, each turn consists of considering the
/// > most recently spoken number:
/// >
/// > - If that was the first time the number has been spoken, the current player says `0`.
/// > - Otherwise, the number had been spoken before; the current player announces how many turns
/// >   apart the number is from when it was previously spoken.
/// >
/// > So, after the starting numbers, each turn results in that player speaking aloud either `0` (if
/// > the last number is new) or an age (if the last number is a repeat).
/// >
/// > For example, suppose the starting numbers are `0,3,6`:
/// >
/// > - Turn 1: The 1st number spoken is a starting number, `0`.
/// > - Turn 2: The 2nd number spoken is a starting number, `3`.
/// > - Turn 3: The 3rd number spoken is a starting number, `6`.
/// > - Turn 4: Now, consider the last number spoken, `6`. Since that was the first time the number
/// >   had been spoken, the 4th number spoken is `0`.
/// > - Turn 5: Next, again consider the last number spoken, `0`. Since it had been spoken before,
/// >   the next number to speak is the difference between the turn number when it was last spoken
/// >   (the previous turn, 4) and the turn number of the time it was most recently spoken before
/// >   then (turn 1). Thus, the 5th number spoken is `4 - 1`, `3`.
/// > - Turn 6: The last number spoken, `3` had also been spoken before, most recently on turns 5
/// >   and 2. So, the 6th number spoken is `5 - 2`, `3`.
/// > - Turn 7: Since `3` was just spoken twice in a row, and the last two turns are `1` turn apart,
/// >   the 7th number spoken is `1`.
/// > - Turn 8: Since `1` is new, the 8th number spoken is `0`.
/// > - Turn 9: `0` was last spoken on turns 8 and 4, so the 9th number spoken is the difference
/// >   between them, `4`.
/// > - Turn 10: `4` is new, so the 10th number spoken is `0`.
///
/// Loop for `iterations` storing the current number, the previous number, and a vector of when a
/// given number was last called. Use the seed values until exhausted and then
/// lookup the previous utterance in the memory array. Finally write the previous value to the
/// memory array.
///
/// # Examples from tests
/// ```
/// assert_eq!(0, play_memory_game(vec!(0, 3, 6), 1));
/// assert_eq!(3, play_memory_game(vec!(0, 3, 6), 2));
/// assert_eq!(6, play_memory_game(vec!(0, 3, 6), 3));
/// assert_eq!(0, play_memory_game(vec!(0, 3, 6), 4));
/// assert_eq!(3, play_memory_game(vec!(0, 3, 6), 5));
/// assert_eq!(3, play_memory_game(vec!(0, 3, 6), 6));
/// assert_eq!(1, play_memory_game(vec!(0, 3, 6), 7));
/// assert_eq!(0, play_memory_game(vec!(0, 3, 6), 8));
/// assert_eq!(4, play_memory_game(vec!(0, 3, 6), 9));
/// assert_eq!(0, play_memory_game(vec!(0, 3, 6), 10));
/// assert_eq!(436, play_memory_game(vec!(0, 3, 6), 2020));
/// assert_eq!(1, play_memory_game(vec!(1,3,2), 2020));
/// assert_eq!(10, play_memory_game(vec!(2,1,3), 2020));
/// assert_eq!(27, play_memory_game(vec!(1,2,3), 2020));
/// assert_eq!(78, play_memory_game(vec!(2,3,1), 2020));
/// assert_eq!(438, play_memory_game(vec!(3,2,1), 2020));
/// assert_eq!(1836, play_memory_game(vec!(3,1,2), 2020));
/// assert_eq!(175594, play_memory_game(vec!(0, 3, 6), 30000000));
/// assert_eq!(2578, play_memory_game(vec!(1,3,2), 30000000));
/// assert_eq!(3544142, play_memory_game(vec!(2,1,3), 30000000));
/// assert_eq!(261214, play_memory_game(vec!(1,2,3), 30000000));
/// assert_eq!(6895259, play_memory_game(vec!(2,3,1), 30000000));
/// assert_eq!(18, play_memory_game(vec!(3,2,1), 30000000));
/// assert_eq!(362, play_memory_game(vec!(3,1,2), 30000000));
/// ```
pub fn play_memory_game(seed: Vec<u32>, iterations: u32) -> u32 {
    let mut memory: Vec<u32> = Vec::new();
    let mut curr = 0 ;
    let mut prev: u32;
    let seed_max = seed.len() as u32;

    for pos in 0..iterations {
        prev = curr;
        if pos <  seed_max {
            curr = *seed.get(pos as usize).unwrap();
        } else {
            let last_seen = memory.get(curr as usize).unwrap_or(&0);
            if last_seen == &0u32 { curr = 0u32 } else { curr = pos - last_seen }
        };

        if pos > 0
        {
            let idx = prev as usize;
            if memory.len() < idx + 1 {
                memory.resize(idx + 1, 0)
            }
            memory[idx] = pos;
        }
    }

    curr
}

#[cfg(test)]
mod tests {
    use day_15::{parse, play_memory_game};

    #[test]
    fn can_parse() {
        assert_eq!(vec!(0u32, 3u32, 6u32), parse("0,3,6"));
        assert_eq!(vec!(1u32, 2u32, 3u32), parse("1,2,3"));
    }

    #[test]
    fn can_play_memory_game() {
        assert_eq!(0, play_memory_game(vec!(0, 3, 6), 1));
        assert_eq!(3, play_memory_game(vec!(0, 3, 6), 2));
        assert_eq!(6, play_memory_game(vec!(0, 3, 6), 3));
        assert_eq!(0, play_memory_game(vec!(0, 3, 6), 4));
        assert_eq!(3, play_memory_game(vec!(0, 3, 6), 5));
        assert_eq!(3, play_memory_game(vec!(0, 3, 6), 6));
        assert_eq!(1, play_memory_game(vec!(0, 3, 6), 7));
        assert_eq!(0, play_memory_game(vec!(0, 3, 6), 8));
        assert_eq!(4, play_memory_game(vec!(0, 3, 6), 9));
        assert_eq!(0, play_memory_game(vec!(0, 3, 6), 10));

        assert_eq!(436, play_memory_game(vec!(0, 3, 6), 2020));
        assert_eq!(1, play_memory_game(vec!(1,3,2), 2020));
        assert_eq!(10, play_memory_game(vec!(2,1,3), 2020));
        assert_eq!(27, play_memory_game(vec!(1,2,3), 2020));
        assert_eq!(78, play_memory_game(vec!(2,3,1), 2020));
        assert_eq!(438, play_memory_game(vec!(3,2,1), 2020));
        assert_eq!(1836, play_memory_game(vec!(3,1,2), 2020));

        assert_eq!(175594, play_memory_game(vec!(0, 3, 6), 30000000));
        assert_eq!(2578, play_memory_game(vec!(1,3,2), 30000000));
        assert_eq!(3544142, play_memory_game(vec!(2,1,3), 30000000));
        assert_eq!(261214, play_memory_game(vec!(1,2,3), 30000000));
        assert_eq!(6895259, play_memory_game(vec!(2,3,1), 30000000));
        assert_eq!(18, play_memory_game(vec!(3,2,1), 30000000));
        assert_eq!(362, play_memory_game(vec!(3,1,2), 30000000));
    }
}
