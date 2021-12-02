use anyhow::{anyhow, bail};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("inputs/day02.txt")?;
    let instructions = parse(&contents)?;

    let simulator = SubmarineSimulator::default();
    let submarine: SimpleSubmarine = simulator.sail(instructions.iter());
    let position = submarine.position();

    println!(
        "(day 01) part 1: {}",
        position.depth * position.horizontal_position
    );

    let submarine: AimingSubmarine = simulator.sail(instructions.iter());
    let pos = submarine.position();

    println!("(day 01) part 2: {}", pos.horizontal_position * pos.depth);

    Ok(())
}

fn parse(contents: &str) -> anyhow::Result<Vec<Instruction>> {
    contents
        .lines()
        .map(Instruction::from_str)
        .collect::<anyhow::Result<_>>()
}

#[derive(Default)]
struct SubmarineSimulator;

impl SubmarineSimulator {
    fn sail<'instr, T: Simulation + Default>(
        &self,
        instructions: impl Iterator<Item = &'instr Instruction>,
    ) -> T {
        instructions.fold(T::default(), |mut sub, instruction| {
            sub.step(instruction);
            sub
        })
    }
}

#[derive(Debug, Default)]
struct SimpleSubmarine {
    position: Position,
}

impl Simulation for SimpleSubmarine {
    fn step(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Down => self.position.depth += instruction.amount,
            Direction::Up => self.position.depth -= instruction.amount,
            Direction::Forward => self.position.horizontal_position += instruction.amount,
        };
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

#[derive(Debug, Default)]
struct AimingSubmarine {
    position: Position,
    aim: isize,
}

impl Simulation for AimingSubmarine {
    fn step(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Down => self.aim += instruction.amount,
            Direction::Up => self.aim -= instruction.amount,
            Direction::Forward => {
                self.position.horizontal_position += instruction.amount;
                self.position.depth += self.aim * instruction.amount
            }
        };
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

trait Simulation {
    fn step(&mut self, instruction: &Instruction);

    fn position(&self) -> &Position;
}

#[derive(Debug, Default)]
struct Position {
    horizontal_position: isize,
    depth: isize,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: isize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = item
            .split_once(|c: char| c.is_ascii_whitespace())
            .ok_or_else(|| anyhow!("Invalid instruction"))?;

        Ok(Instruction {
            direction: Direction::from_str(direction)?,
            amount: amount
                .parse()
                .map_err(|e| anyhow!("Invalid amount {}", e))?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        Ok(match item {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => bail!("Invalid instruction"),
        })
    }
}
