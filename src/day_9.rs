use std::collections::{LinkedList, HashSet};
use std::fs;
use im::vector::Vector;
use std::ops::Add;


pub fn run() {
    let contents = fs::read_to_string("res/day-9-input").expect("Failed to read file");
    let input = contents.lines().map(|line| line.parse::<usize>().unwrap()).collect();

    let result = find_first_invalid(&input, 25).unwrap();
    println!("First invalid number is: {}", result);

    let weakness = find_weakness(&input, result).unwrap();
    println!("Encryption weakness: {}", weakness);
}

fn find_first_invalid(input: &Vec<usize>, preamble: usize) -> Option<usize> {
    let mut cache: LinkedList<(usize, HashSet<usize>)> = LinkedList::new();
    for i in input {
        // if preamble used up, cache is full, check next number and then remove earliest
        if cache.len() == preamble {
            let mut found = false;
            for (_, sums) in &cache {
                if sums.contains(&i) {
                    found = true;
                    break
                }
            }

            if !found {
                return Some(*i)
            }

            cache.pop_front();
        }

        // cache sum for each previous value
        for (j, sums) in &mut cache {
            sums.insert( i + *j);
        }

        let new_set = HashSet::new();

        // append the new number
        cache.push_back((*i, new_set));
    }

    None
}

fn find_weakness(input: &Vec<usize>, target: usize) -> Option<usize> {
    fn find_weakness_iter(target: usize, cache: Vector<(usize, usize, usize)>, remaining: Vector<usize>) -> Option<usize> {
        match remaining.head() {
            Some(i) => {
                let updated: Vector<(usize, usize, usize)> =
                    cache.iter()
                        .map(|(acc, min, max)| (acc + i, *min.min(i), *max.max(i)))
                        .filter(|(acc, _, _)| *acc <= target)
                        .collect();
                match updated.head() {
                    Some((v, min, max)) if *v == target => Some(min + max),
                    _ => find_weakness_iter(target, updated.add(Vector::unit((*i, *i, *i))), remaining.skip(1))
                }
            },
            None => None
        }
    }

    find_weakness_iter(target, Vector::new(), Vector::from(input))
}

#[cfg(test)]
mod tests {
    use day_9::{find_first_invalid, find_weakness};

    fn input() -> Vec<usize> {
        vec!(
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        )
    }

    #[test]
    fn can_find_first_invalid() {
        assert_eq!(
            Some(127usize),
            find_first_invalid(&input(), 5)
        );

        assert_eq!(
            None,
            find_first_invalid(&input().into_iter().take(14).collect(), 5)
        )
    }

    #[test]
    fn can_find_weakness() {
        assert_eq!(
            Some(62),
            find_weakness(&input(), 127)
        );

        assert_eq!(
            None,
            find_weakness(&input(), 1)
        )
    }
}
