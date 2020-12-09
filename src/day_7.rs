use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};

type Label = str;

#[derive(Debug, Eq, PartialEq)]
struct Rule<'a> {
    label: &'a Label,
    contents: HashMap<&'a Label, usize>,
}

impl<'a> Rule<'a> {
    fn from_line(line: &'a str) -> Rule<'a> {
        let parts: Vec<&str> = line.split(" bags contain ").into_iter().collect();
        let &bag = parts.get(0).unwrap();
        let &content_str = parts.get(1).unwrap();
        let re = Regex::new(r"(\d+) ([a-z]+ [a-z]+)").unwrap();

        let contents: HashMap<&'a Label, usize> = match content_str {
            "no other bags." => HashMap::new(),
            str => {
                let mut map = HashMap::new();
                re.captures_iter(str).for_each(|cap| {
                    map.insert(
                        cap.get(2).unwrap().as_str(),
                        cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    );
                });

                map
            }
        };

        Rule {
            label: bag,
            contents,
        }
    }
}

pub fn run() {
    let contents = fs::read_to_string("res/day-7-input").expect("Failed to read file");
    let rules = contents.lines().map(|line| Rule::from_line(line)).into_iter().collect();

    let containers = find_all_containers(&rules, "shiny gold");
    println!("There are {} possible containers.", containers.len());

    let count = count_bag_contents(&rules, "shiny gold");
    println!("There are {} bags in a shiny gold bag.", count);
}

fn build_direct_containers<'a>(rules: &Vec<Rule<'a>>) -> HashMap<&'a Label, HashSet<&'a Label>> {
    let mut parent_map = HashMap::new();

    rules.iter()
        .flat_map(|rule| rule.contents.iter().map(move |child| (rule.label, *child.0)))
        .for_each(|(parent, child)| {
            if !parent_map.contains_key(child) {
                parent_map.insert(child, HashSet::new());
            }

            parent_map.get_mut(child).unwrap().insert(parent);
        });

    parent_map
}


fn find_all_containers<'a>(rules: &Vec<Rule<'a>>, seed: &'a Label) -> HashSet<&'a Label> {
    let direct_containers = build_direct_containers(rules);

    let mut possible_containers: HashSet<&Label> = HashSet::new();
    let mut to_check: LinkedList<&Label> = LinkedList::new();

    to_check.push_back(seed);

    while !to_check.is_empty() {
        let next = to_check.pop_front().unwrap();
        match direct_containers.get(next) {
            Some(set) =>
                set.iter()
                    .filter_map(|&container|
                        if possible_containers.contains(container) { None } else { Some(container) }
                    )
                    .collect(),
            None => HashSet::new()
        }.iter().for_each(|&container| {
            possible_containers.insert(container);
            to_check.push_back(container);
        });
    }

    possible_containers
}

fn count_bag_contents(rules: &Vec<Rule>, outer_bag: &Label) -> usize {
    let mut rule_map: HashMap<&Label, Vec<(&Label, usize)>> = HashMap::new();
    rules.iter().for_each(
        |rule| {
            rule_map.insert(
                rule.label,
                rule.contents.iter().map(|(&k, &v)| (k, v)).collect()
            );
        }
    );

    count_bag_contents_iter(&rule_map, outer_bag) - 1 // exclude the outer bag from the count
}

fn count_bag_contents_iter(rule_map: &HashMap<&Label, Vec<(&Label, usize)>>, bag: &Label) -> usize {
    match rule_map.get(bag) {
        Some(contents) =>
            contents.iter().map(|(inner_bag, count)| count_bag_contents_iter(rule_map, inner_bag) * count).sum::<usize>() + 1usize,
        None => 1
    }
}

#[cfg(test)]
mod tests {
    use day_7::{Rule, Label, build_direct_containers, find_all_containers, count_bag_contents};
    use std::collections::{HashMap, HashSet};

    // https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
    macro_rules! map (
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    fn sample_rules<'a>() -> Vec<Rule<'a>> {
        vec!(
            Rule {
                label: "light red",
                contents: map!("bright white" => 1usize, "muted yellow" => 2usize),
            },
            Rule {
                label: "dark orange",
                contents: map!("bright white" => 3usize,  "muted yellow" => 4usize),
            },
            Rule {
                label: "bright white",
                contents: map!("shiny gold" => 1usize),
            },
            Rule {
                label: "muted yellow",
                contents: map!("shiny gold" => 2usize,  "faded blue" => 9usize),
            },
            Rule {
                label: "shiny gold",
                contents: map!("dark olive" => 1usize,  "vibrant plum" => 2usize),
            },
            Rule {
                label: "dark olive",
                contents: map!("faded blue" => 3usize,  "dotted black" => 4usize),
            },
            Rule {
                label: "vibrant plum",
                contents: map!("faded blue" => 5usize,  "dotted black" => 6usize),
            },
            Rule {
                label: "faded blue",
                contents: HashMap::new(),
            },
            Rule {
                label: "dotted black",
                contents: HashMap::new(),
            },
        )
    }

    fn small_rules<'a>() -> Vec<Rule<'a>> {
        vec!(
            Rule {
                label: "light red",
                contents: map!("bright white" => 1usize, "muted yellow" => 2usize),
            },
            Rule {
                label: "dark orange",
                contents: map!("bright white" => 3usize,  "muted yellow" => 4usize),
            },
            Rule {
                label: "bright white",
                contents: map!("shiny gold" => 1usize),
            },
        )
    }

    #[test]
    fn can_parse_rule() {
        assert_eq!(
            Rule::from_line("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Rule {
                label: "light red",
                contents: map!("bright white" => 1usize, "muted yellow" => 2usize),
            }
        )
    }

    #[test]
    fn can_parse_rules() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let expected = sample_rules();

        assert_eq!(
            expected,
            input.lines().map(|line| Rule::from_line(line)).into_iter().collect::<Vec<Rule>>()
        )
    }

    #[test]
    fn can_build_direct_containers() {
        let expected: HashMap<&Label, HashSet<&Label>> = map!(
            "bright white" => vec!("light red", "dark orange").into_iter().collect(),
            "muted yellow" => vec!("light red", "dark orange").into_iter().collect(),
            "shiny gold" => vec!("bright white").into_iter().collect()
        );

        assert_eq!(
            expected,
            build_direct_containers(&small_rules())
        )
    }

    #[test]
    fn can_find_possible_containers() {
        let expected_sg_sml: HashSet<&Label> = vec!("bright white", "light red", "dark orange").into_iter().collect();
        let expected_sg_lrg: HashSet<&Label> = vec!("bright white", "light red", "dark orange", "muted yellow").into_iter().collect();
        let expected_my: HashSet<&Label> = vec!("light red", "dark orange").into_iter().collect();

        assert_eq!(expected_sg_sml, find_all_containers(&small_rules(), "shiny gold"));
        assert_eq!(expected_sg_lrg, find_all_containers(&sample_rules(), "shiny gold"));
        assert_eq!(expected_my, find_all_containers(&small_rules(), "muted yellow"));
    }

    #[test]
    fn can_count_bag_contents() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let rainbow_rules = input.lines().map(|line| Rule::from_line(line)).into_iter().collect::<Vec<Rule>>();

        assert_eq!(0, count_bag_contents(&small_rules(), "shiny gold"));
        assert_eq!(4, count_bag_contents(&small_rules(), "light red"));
        assert_eq!(10, count_bag_contents(&small_rules(), "dark orange"));
        assert_eq!(32, count_bag_contents(&sample_rules(), "shiny gold"));
        assert_eq!(126, count_bag_contents(&rainbow_rules, "shiny gold"));
    }
}
