use std::fs;

pub fn run() {
    let contents = fs::read_to_string("res/day-10-input").expect("Failed to read file");
    let adapters = parse(contents.as_str());

    let (ones, threes) = calculate_jolts(&adapters);
    println!("{} ones x {} threes = {}", ones, threes, ones * threes);

    let combinations = calculate_combinations(&adapters);
    println!("{} possible combinations", combinations);
}

fn parse(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();
    adapters.sort();

    adapters
}

fn calculate_jolts(adapters: &Vec<usize>) -> (usize, usize) {
    let (ones, threes, _) = adapters.iter().fold(
        (0, 0, 0),
        |(ones, threes, prev), &adapter| match adapter - prev {
            1 => (ones + 1, threes, adapter),
            3 => (ones, threes + 1, adapter),
            _ => (ones, threes, adapter)
        },
    );

    // add the final jump of three to the device
    (ones, threes + 1)
}

fn calculate_combinations(adapters: &Vec<usize>) -> usize {
    let (combinations, run, _) = adapters.iter().fold(
        (1, 0, 0),
        |(acc, run, prev), &adapter| match adapter - prev {
            1 => (acc, run + 1, adapter),
            3 => (acc * run_combinations(run), 0, adapter),
            _ => panic!("not just 1s and 3s")
        });

        combinations * run_combinations(run)
}

fn run_combinations(run: usize) -> usize {
   match run {
       0 => 1,
       1 => 1,
       2 => 2,
       _ => run_combinations(run - 1) + run_combinations(run - 2) + run_combinations(run - 3)
   }
}

#[cfg(test)]
mod tests {
    use day_10::{calculate_jolts, parse, calculate_combinations};

    fn small_input() -> &'static str {
        "16
10
15
5
1
11
7
19
6
12
4"
    }

    fn medium_input() -> &'static str {
        "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
    }

    #[test]
    fn can_parse() {
        assert_eq!(
            vec!(1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19),
            parse(small_input())
        );

        assert_eq!(
            vec!(
                1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
                32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49
            ),
            parse(medium_input())
        );
    }

    #[test]
    fn can_calculate_jolts() {
        assert_eq!(
            (7usize, 5usize),
            calculate_jolts(&parse(small_input()))
        );

        assert_eq!(
            (22usize, 10usize),
            calculate_jolts(&parse(medium_input()))
        );
    }

    #[test]
    fn can_calculate_combinations() {
        assert_eq!(
            8,
            calculate_combinations(&parse(small_input()))
        );

        assert_eq!(
            19208,
            calculate_combinations(&parse(medium_input()))
        );
    }
}
