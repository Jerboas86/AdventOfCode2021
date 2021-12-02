use std::{error::Error, fs, str::FromStr};

fn main() {
    let raw_input = fs::read_to_string("day2B.txt").expect("Reading input file failed");
    let instructions = Instructions::from_str(&raw_input).expect("Failed to parse raw input");
    let position = Position::from(instructions);

    println!("{}", position.x * position.y);
}

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Movement {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        let key = tokens[0];
        let count: i32 = tokens[1].parse()?;

        match (key, count) {
            ("down", c) => Ok(Self::Down(c)),
            ("up", c) => Ok(Self::Up(c)),
            ("forward", c) => Ok(Self::Forward(c)),
            _ => Err("Parsing tokens failed".into()),
        }
    }
}

struct Instructions(Vec<Movement>);

impl FromStr for Instructions {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instrunctions: Vec<&str> = s.split('\n').collect();

        let mut movements = Vec::new();
        for instr in instrunctions {
            let mov: Movement = instr.parse()?;
            movements.push(mov);
        }

        Ok(Self(movements))
    }
}

#[derive(Debug)]
struct Position {
    aim: i32,
    x: i32,
    y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { aim: 0, x: 0, y: 0 }
    }
}

impl From<Instructions> for Position {
    fn from(instructions: Instructions) -> Self {
        let mut position = Self::default();
        for mov in instructions.0 {
            match mov {
                Movement::Down(c) => {
                    position.aim += c;
                }
                Movement::Up(c) => {
                    position.aim -= c;
                }
                Movement::Forward(c) => {
                    position.x += c;
                    position.y += c * position.aim;
                }
            }
        }
        position
    }
}
