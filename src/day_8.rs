use std::fs;
use regex::Regex;
use day_8::Instruction::*;
use std::collections::HashSet;
use day_8::ProgramResult::*;
use im::Vector;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

#[derive(Debug, Eq, PartialEq)]
enum ProgramResult {
    INFINITE(isize),
    COMPLETE(isize),
}

pub fn run() {
    let contents = fs::read_to_string("res/day-8-input").expect("Failed to read file");
    let program = parse_lines(contents.as_str());

    let original_result = run_program(&program);
    println!("Original result = {:?}", original_result);

    let fixed_result = find_finite_program(&program);
    println!("Fixed result = {:?}", fixed_result);
}

fn parse_lines(input: &str) -> Vector<Instruction> {
    let re = Regex::new(r"(acc|jmp|nop) ([+-]\d+)").unwrap();

    input.lines()
        .flat_map(|line| re.captures(line))
        .map(|cap| match (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str().parse::<isize>().unwrap()) {
            ("acc", v) => ACC(v),
            ("jmp", v) => JMP(v),
            ("nop", v) => NOP(v),
            _ => panic!("unexpected instruction '{}'", cap.get(0).unwrap().as_str())
        })
        .collect()
}

fn run_program(program: &Vector<Instruction>) -> ProgramResult {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut pos: usize = 0;
    let mut acc: isize = 0;

    while !visited.contains(&pos) {
        visited.insert(pos);
        if pos == program.len()
        {
            return COMPLETE(acc);
        }
        match program.get(pos) {
            Some(ACC(v)) => {
                acc = acc + v;
                pos = pos + 1;
            },
            Some(JMP(v)) => pos = (pos as isize + v) as usize,
            Some(NOP(_)) => pos = pos + 1,
            None => panic!("No instruction at position {}", pos)
        }
    }

    INFINITE(acc)
}

fn find_finite_program(program: &Vector<Instruction>) -> Option<isize> {
    for i in 0..program.len() {
        let result = match program.get(i) {
            Some(JMP(v)) => run_program(&program.update(i, NOP(*v))),
            Some(NOP(v)) => run_program(&program.update(i, JMP(*v))),
            _ => INFINITE(0)
        };

        match result {
            INFINITE(_) => (),
            COMPLETE(v) => return Some(v),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use day_8::Instruction::*;
    use day_8::ProgramResult::*;
    use day_8::{parse_lines, run_program, find_finite_program};
    use im::vector;

    fn get_input() -> &'static str {
        "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
    }

    #[test]
    fn can_parse() {
        assert_eq!(
            vector!(NOP(0), ACC(1), JMP(4), ACC(3), JMP(-3), ACC(-99), ACC(1), JMP(-4), ACC(6)),
            parse_lines(get_input())
        )
    }

    #[test]
    fn can_run_infinite_program() {
        assert_eq!(
            INFINITE(5),
            run_program(&vector!(NOP(0), ACC(1), JMP(4), ACC(3), JMP(-3), ACC(-99), ACC(1), JMP(-4), ACC(6)))
        )
    }

    #[test]
    fn can_run_finite_program() {
        assert_eq!(
            COMPLETE(8),
            run_program(&vector!(NOP(0), ACC(1), JMP(4), ACC(3), JMP(-3), ACC(-99), ACC(1), NOP(-4), ACC(6)))
        )
    }

    #[test]
    fn can_find_finite_program() {
        assert_eq!(
            Some(8),
            find_finite_program(&vector!(NOP(0), ACC(1), JMP(4), ACC(3), JMP(-3), ACC(-99), ACC(1), JMP(-4), ACC(6)))
        )
    }
}
