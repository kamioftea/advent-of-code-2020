use std::fs;
use day_12::Instruction::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

#[derive(Debug, Eq, PartialEq)]
struct Ship {
    x: isize,
    y: isize,
    facing: Facing,
}

#[derive(Debug, Eq, PartialEq)]
struct Facing {dx: isize, dy: isize}

impl Facing {
    const NORTH:Facing = Facing {dx: 0, dy: -1};
    const EAST:Facing = Facing {dx: 1, dy: 0};
    const SOUTH:Facing = Facing {dx: 0, dy: 1};
    const WEST:Facing = Facing {dx: -1, dy: 0};
    
    fn multiply(&self, magnitude: isize) -> Facing {
        Facing {dx: self.dx * magnitude, dy: self.dy * magnitude}
    }
    
    fn rotate(&self, degrees: isize) -> Facing {
        match degrees % 360 { 
            0 => Facing {dx: self.dx, dy: self.dy},
            90 => Facing {dx: -self.dy, dy: self.dx},
            180 => Facing {dx: -self.dx, dy: -self.dy},
            270 => Facing {dx: self.dy, dy: -self.dx},
            deg => panic!("Invalid angle: {}° ({}°)", deg, degrees)
        }
    }

    fn merge(&self, other: Facing) -> Facing {
        Facing {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy
        }
    }
}

impl Ship {
    fn new() -> Ship {
        Ship { x: 0, y: 0, facing: Facing::EAST }
    }

    fn new_waypoint() -> Ship {
        Ship { x: 0, y: 0, facing: Facing { dx: 10, dy: -1 } }
    }

    fn navigate(&mut self, inst: Instruction) {
        match inst {
            North(n) => self.advance(Facing::NORTH.multiply(n)),
            East(n) => self.advance(Facing::EAST.multiply(n)),
            South(n) => self.advance(Facing::SOUTH.multiply(n)),
            West(n) => self.advance(Facing::WEST.multiply(n)),
            Left(n) => self.facing = self.facing.rotate(360 - n),
            Right(n) => self.facing = self.facing.rotate( n),
            Forward(n) => self.advance(self.facing.multiply(n))
        }
    }

    fn navigate_all(&mut self, instructions: &Vec<Instruction>) {
        instructions.iter().for_each(|&i| self.navigate(i))
    }

    fn navigate_with_waypoint(&mut self, inst: Instruction) {
        match inst {
            North(n) => self.facing = self.facing.merge(Facing::NORTH.multiply(n)),
            East(n) => self.facing = self.facing.merge(Facing::EAST.multiply(n)),
            South(n) => self.facing = self.facing.merge(Facing::SOUTH.multiply(n)),
            West(n) => self.facing = self.facing.merge(Facing::WEST.multiply(n)),
            Left(n) => self.facing = self.facing.rotate(360 - n),
            Right(n) => self.facing = self.facing.rotate( n),
            Forward(n) => self.advance(self.facing.multiply(n))
        }
    }

    fn navigate_all_with_waypoint(&mut self, instructions: &Vec<Instruction>) {
        instructions.iter().for_each(|&i| self.navigate_with_waypoint(i))
    }
    
    fn advance(&mut self, vector: Facing) {
        self.x = self.x + vector.dx;
        self.y = self.y + vector.dy;
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

pub fn run() {
    let contents = fs::read_to_string("res/day-12-input").expect("Failed to read file");
    let instructions = parse_input(contents.as_str());

    let mut ship = Ship::new();
    ship.navigate_all(&instructions);
    println!("{:?} has a manhattan distance of {} from its starting position.", ship, ship.manhattan_distance());

    let mut waypoint_ship = Ship::new_waypoint();
    waypoint_ship.navigate_all_with_waypoint(&instructions);
    println!("Using a waypoint, {:?} has a manhattan distance of {} from its starting position.", waypoint_ship, waypoint_ship.manhattan_distance());
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| line.split_at(1))
        .map(|(letter, number)| (letter.chars().next().unwrap(), number.parse::<isize>().unwrap()))
        .map(|(instruction, magnitude)| match instruction {
            'N' => North(magnitude),
            'S' => South(magnitude),
            'E' => East(magnitude),
            'W' => West(magnitude),
            'L' => Left(magnitude),
            'R' => Right(magnitude),
            'F' => Forward(magnitude),
            _ => panic!(format!("Invalid instruction {}", magnitude))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use day_12::{parse_input, Ship, Facing};
    use day_12::Instruction::*;

    #[test]
    fn can_parse() {
        assert_eq!(
            vec![Forward(10), North(3), Forward(7), Right(90), Forward(11)],
            parse_input(
                "F10
N3
F7
R90
F11"
            )
        )
    }

    #[test]
    fn can_navigate() {
        let mut ship = Ship::new();
        assert_eq!(Ship { x: 0, y: 0, facing: Facing::EAST }, ship);

        ship.navigate(North(10));
        assert_eq!(Ship { x: 0, y: -10, facing: Facing::EAST }, ship);

        ship.navigate(East(10));
        assert_eq!(Ship { x: 10, y: -10, facing: Facing::EAST }, ship);

        ship.navigate(Left(90));
        assert_eq!(Ship { x: 10, y: -10, facing: Facing::NORTH }, ship);

        ship.navigate(Forward(5));
        assert_eq!(Ship { x: 10, y: -15, facing: Facing::NORTH }, ship);

        ship.navigate(South(15));
        assert_eq!(Ship { x: 10, y: 0, facing: Facing::NORTH }, ship);

        ship.navigate(Right(270));
        assert_eq!(Ship { x: 10, y: 0, facing: Facing::WEST }, ship);

        ship.navigate(Forward(10));
        assert_eq!(Ship { x: 0, y: 0, facing: Facing::WEST }, ship);

        ship.navigate(West(10));
        assert_eq!(Ship { x: -10, y: 0, facing: Facing::WEST }, ship);

        ship.navigate(Left(90));
        assert_eq!(Ship { x: -10, y: 0, facing: Facing::SOUTH }, ship);

        ship.navigate(Forward(90));
        assert_eq!(Ship { x: -10, y: 90, facing: Facing::SOUTH }, ship);

        let mut other = Ship::new();
        other.navigate_all(
            &vec!(
                Left(90),
                Left(180),
                Left(270),
            )
        );
        assert_eq!(Facing::WEST, other.facing);
        other.navigate_all(
            &vec!(
                Right(90),
                Right(180),
                Right(270),
            )
        );
        assert_eq!(Facing::EAST, other.facing);
    }

    #[test]
    fn can_calc_distance() {
        assert_eq!(0usize, Ship::new().manhattan_distance());
        assert_eq!(2usize, Ship { x: 1, y: 1, facing: Facing::EAST }.manhattan_distance());
        assert_eq!(4usize, Ship { x: -1, y: 3, facing: Facing::EAST }.manhattan_distance());
        assert_eq!(8usize, Ship { x: -6, y: -2, facing: Facing::WEST }.manhattan_distance());
    }

    #[test]
    fn can_process_example() {
        let mut ship = Ship::new();
        ship.navigate_all(&parse_input(
            "F10
N3
F7
R90
F11"
        ));

        assert_eq!(Ship { x: 17, y: 8, facing: Facing::SOUTH }, ship);
        assert_eq!(25, ship.manhattan_distance());
    }

    #[test]
    fn can_navigate_with_waypoint() {
        let mut ship = Ship::new_waypoint();
        ship.navigate_all_with_waypoint(&parse_input(
            "F10
N3
F7
R90
F11"
        ));

        assert_eq!(Ship { x: 214, y: 72, facing: Facing {dx: 4, dy: 10} }, ship);
        assert_eq!(286, ship.manhattan_distance());
    }
}
