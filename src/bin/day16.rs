use anyhow::{anyhow, Context};
use std::fmt::{Display, Formatter};
use proc_macro::TokenTree::Literal;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/example/day16.txt").trim();
    let mut parser = Parser::new(input);

    println!("{}", parser);

    let version = parser.read_version();
    let type_id = parser.read_type_id();

    println!("{:?}", version);
    println!("{:?}", type_id);

    println!("{}", parser);

    println!("(day 16) part 1: {}", -1);

    println!("(day 16) part 2: {}", -1);

    Ok(())
}

macro_rules! map_digit {
    ($digit:expr) => {
        [
            map_bit!($digit, 0b1000),
            map_bit!($digit, 0b0100),
            map_bit!($digit, 0b0010),
            map_bit!($digit, 0b0001),
        ]
    };
}

macro_rules! map_bit {
    ($digit:expr, $bit:expr) => {
        if $digit & $bit != 0 {
            Bit::High
        } else {
            Bit::Low
        }
    };
}

struct Parser {
    bits: Vec<Bit>,
    current: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        let bits = input
            .chars()
            .map(|v| {
                let digit = v.to_digit(16).unwrap();
                map_digit!(digit)
            })
            .fold(Vec::with_capacity(input.len()), |mut acc, next| {
                acc.extend_from_slice(&next);
                acc
            });

        Self { bits, current: 0 }
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read_version();

        todo!()
    }

    fn read_version(&mut self) -> Version {
        let slice = &self.bits[self.current..self.current + Version::SIZE];
        self.current += Version::SIZE;

        Version(into_number(slice) as u8)
    }

    fn read_type_id(&mut self) -> TypeId {
        let slice = &self.bits[self.current..self.current + TypeId::SIZE];
        self.current += TypeId::SIZE;

        TypeId::from(into_number(slice))
    }

    fn read_literal(&mut self) -> usize {
        let mut literal = self.bits[self.current..]
            .chunks_exact(5)
            .enumerate()
            .take_while(|(nth, chunk)| {
                chunk[0] == Bit::High
            }).fold(Vec::new(), |mut acc, (n, bits)| {
                acc.extend_from_slice(&bits[1..]); // skip the stop marker
                self.current += Literal::SIZE;
                acc
            });

        literal.extend_from_slice()


        0
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .bits
                .iter()
                .map(|&bit| if bit == Bit::Low { '0' } else { '1' })
                .collect::<String>(),
        )?;

        f.write_fmt(format_args!(" @ {}", self.current))
    }
}

#[derive(Debug)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

#[derive(Debug)]
struct Literal {
    version: Version,
}

#[derive(Debug)]
struct Operator {
    version: Version,
    type_id: TypeId,
    operands: Vec<Packet>,
}

#[derive(Debug, Copy, Clone)]
struct Version(u8);

impl Version {
    const SIZE: usize = 3;
}

#[derive(Debug, Copy, Clone)]
enum TypeId {
    Literal,
}

impl From<usize> for TypeId {
    fn from(n: usize) -> Self {
        match n {
            4 => Self::Literal,
            n => unimplemented!("{}", n),
        }
    }
}

impl TypeId {
    const SIZE: usize = 3;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Bit {
    High,
    Low,
}

impl From<Bit> for char {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::High => '1',
            Bit::Low => '0',
        }
    }
}

struct Bits<'l>(&'l [Bit]);

impl Bits<'_> {
    fn as_number(&self) -> usize {
        into_number(self.0)
    }
}

fn into_number(bits: &[Bit]) -> usize {
    let mut i = 0_usize;
    bits.iter().rfold(0, |mut acc, next| {
        let bit_value = match next {
            Bit::High => (1 << i),
            Bit::Low => 0,
        };

        i += 1;

        acc + bit_value
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        assert_eq!(0, 40);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day16.txt");

        assert_eq!(0, 755);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day16.txt");

        assert_eq!(0, 315);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day16.txt");

        assert_eq!(0, 3016);
    }
}
