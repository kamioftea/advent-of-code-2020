use std::fs;

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

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
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
    fn can_parse_file() {
        assert_eq!(
            vec!(false, false, true, true, false, false, false, false, false, false, false),
            parse_line(test_lines().get(0).unwrap())
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
