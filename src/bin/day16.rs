use std::fmt::{Display, Formatter};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day16.txt").trim();
    println!("(day 16) part 1: {}", part1(input));

    println!("(day 16) part 2: {}", part2(input));

    Ok(())
}

fn part1(input: &str) -> usize {
    let mut p = Parser::new(input);
    let packet = p.read_packet().0;

    packet.count_versions()
}

fn part2(input: &str) -> usize {
    let mut p = Parser::new(input);
    let packet = p.read_packet().0;

    packet.eval()
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

    fn read_packet(&mut self) -> (Packet, usize) {
        let version = self.read_version();
        let type_id = self.read_type_id();

        let packet = match type_id {
            TypeId::Literal => Packet::Literal(Literal {
                version,
                type_id,
                num: self.read_literal(),
            }),
            TypeId::Operator(_) => Packet::Operator(Operator {
                version,
                type_id,
                operands: self.read_operator(),
            }),
        };

        (packet, self.current)
    }

    fn read_version(&mut self) -> Version {
        let slice = &self.bits[self.current..self.current + Version::SIZE];
        self.current += Version::SIZE;

        Version(bits_to_number(slice) as u8)
    }

    fn read_type_id(&mut self) -> TypeId {
        let slice = &self.bits[self.current..self.current + TypeId::SIZE];
        self.current += TypeId::SIZE;

        TypeId::from(bits_to_number(slice))
    }

    fn read_literal(&mut self) -> usize {
        let mut literal = self.bits[self.current..]
            .chunks_exact(5)
            .enumerate()
            .take_while(|(_nth, chunk)| chunk[0] == Bit::High)
            .fold(Vec::new(), |mut acc, (_n, bits)| {
                acc.extend_from_slice(&bits[1..]); // skip the stop marker
                self.current += Literal::SIZE;
                acc
            });

        // also extend the number with the last group of  bits
        literal.extend_from_slice(&self.bits[self.current + 1..self.current + Literal::SIZE]);
        self.current += Literal::SIZE;

        // // skip the remainder of the hex number.
        // let skip = self.current % 16;
        // self.current += skip;
        //
        // println!("lit: {}", Bits(&literal).to_string());

        bits_to_number(&literal)
    }

    fn read_operator(&mut self) -> Vec<Packet> {
        let variant = self.read_operator_variant();
        match variant {
            OperatorLength::LengthOfSubPackets => {
                let length = self.read_sub_packet_length();
                self.read_sub_packets_by_length(length)
            }
            OperatorLength::NumberOfSubPackets => {
                let count = self.read_sub_packet_count();
                self.read_sub_packets_by_count(count)
            }
        }
    }

    fn read_operator_variant(&mut self) -> OperatorLength {
        let bits = &self.bits[self.current..self.current + OperatorLength::SIZE];
        self.current += OperatorLength::SIZE;

        match bits[0] {
            Bit::Low => OperatorLength::LengthOfSubPackets,
            Bit::High => OperatorLength::NumberOfSubPackets,
        }
    }

    fn read_sub_packet_length(&mut self) -> usize {
        let bits = &self.bits[self.current..self.current + 15];
        self.current += 15;
        bits_to_number(bits)
    }

    fn read_sub_packets_by_length(&mut self, length: usize) -> Vec<Packet> {
        let end = self.current + length;

        let mut vec = Vec::new();
        while self.current < end {
            vec.push(self.read_packet().0);
        }

        vec

        // std::iter::repeat_with(|| self.read_packet())
        //     .take_while(|(_packet, len)| *len < end)
        //     .map(|(packet, _len)| packet)
        //     .collect()
    }

    fn read_sub_packet_count(&mut self) -> usize {
        let bits = &self.bits[self.current..self.current + 11];
        self.current += 11;

        bits_to_number(bits)
    }

    fn read_sub_packets_by_count(&mut self, count: usize) -> Vec<Packet> {
        std::iter::repeat_with(|| self.read_packet())
            .take(count)
            .map(|(packet, _len)| packet)
            .collect()
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bit_string = self
            .bits
            .iter()
            .map(|&bit| if bit == Bit::Low { '0' } else { '1' })
            .collect::<String>();

        f.write_str(&bit_string)?;
        f.write_fmt(format_args!(
            "\n{}^{}",
            " ".repeat(self.current),
            self.current
        ))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

impl Packet {
    fn count_versions(&self) -> usize {
        match self {
            Self::Literal(lit) => usize::from(lit.version.0),
            Self::Operator(ops) => {
                ops.operands
                    .iter()
                    .map(|packet| packet.count_versions())
                    .sum::<usize>()
                    + usize::from(ops.version.0)
            }
        }
    }

    fn eval(&self) -> usize {
        match &self {
            Packet::Literal(lit) => lit.num,
            Packet::Operator(op) => op.eval(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Literal {
    version: Version,
    type_id: TypeId,
    num: usize,
}

impl Literal {
    const SIZE: usize = 5;
}

#[derive(Debug, Eq, PartialEq)]
struct Operator {
    version: Version,
    type_id: TypeId,
    operands: Vec<Packet>,
}

impl Operator {
    fn eval(&self) -> usize {
        match self.type_id {
            TypeId::Operator(OpType::Sum) => {
                self.operands.iter().fold(0, |acc, next| acc + next.eval())
            }
            TypeId::Operator(OpType::Product) => {
                self.operands.iter().fold(1, |acc, next| acc * next.eval())
            }
            TypeId::Operator(OpType::Maximum) => self
                .operands
                .iter()
                .fold(0, |acc, next| acc.max(next.eval())),
            TypeId::Operator(OpType::Minimum) => self
                .operands
                .iter()
                .fold(usize::MAX, |acc, next| acc.min(next.eval())),
            TypeId::Operator(OpType::LessThan) => {
                if self.operands[0].eval() < self.operands[1].eval() {
                    1
                } else {
                    0
                }
            }
            TypeId::Operator(OpType::GreaterThan) => {
                if self.operands[0].eval() > self.operands[1].eval() {
                    1
                } else {
                    0
                }
            }
            TypeId::Operator(OpType::EqualTo) => {
                if self.operands[0].eval() == self.operands[1].eval() {
                    1
                } else {
                    0
                }
            }
            TypeId::Literal => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Version(u8);

impl Version {
    const SIZE: usize = 3;
}

#[derive(Debug, Eq, PartialEq)]
enum TypeId {
    Literal,
    Operator(OpType),
}

#[derive(Debug, Eq, PartialEq)]
enum OpType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<usize> for TypeId {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::Operator(OpType::Sum),
            1 => Self::Operator(OpType::Product),
            2 => Self::Operator(OpType::Minimum),
            3 => Self::Operator(OpType::Maximum),
            4 => Self::Literal,
            5 => Self::Operator(OpType::GreaterThan),
            6 => Self::Operator(OpType::LessThan),
            7 => Self::Operator(OpType::EqualTo),
            n => unimplemented!("{}", n),
        }
    }
}

impl TypeId {
    const SIZE: usize = 3;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OperatorLength {
    LengthOfSubPackets,
    NumberOfSubPackets,
}

impl OperatorLength {
    const SIZE: usize = 1;
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

fn bits_to_number(bits: &[Bit]) -> usize {
    // let mut i = 0_usize;
    bits.iter().rev().enumerate().fold(0, |acc, (i, next)| {
        let bit_value = match next {
            Bit::High => (1 << i),
            Bit::Low => 0,
        };

        // i += 1;

        acc + bit_value
    })
}

#[cfg(test)]
mod tests {
    use crate::{part1, Literal, OpType, Operator, Packet, Parser, TypeId, Version};

    #[test]
    fn literal() {
        let mut p = Parser::new("D2FE28");
        let version = p.read_version();
        assert_eq!(version, Version(6));
        let type_id = p.read_type_id();
        assert_eq!(type_id, TypeId::Literal);
        let lit = p.read_literal();
        assert_eq!(lit, 2021);
    }

    #[test]
    fn operator_with_len_of_subpackets() {
        let mut p = Parser::new("38006F45291200");
        let packets = p.read_packet().0;

        let expected = Packet::Operator(Operator {
            version: Version(1),
            type_id: TypeId::Operator(OpType::LessThan),
            operands: vec![
                Packet::Literal(Literal {
                    version: Version(6),
                    type_id: TypeId::Literal,
                    num: 10,
                }),
                Packet::Literal(Literal {
                    version: Version(2),
                    type_id: TypeId::Literal,
                    num: 20,
                }),
            ],
        });

        assert_eq!(expected, packets)
    }

    #[test]
    fn operator_with_count_of_subpackets() {
        let mut p = Parser::new("EE00D40C823060");
        let packets = p.read_packet().0;

        let expected = Packet::Operator(Operator {
            version: Version(7),
            type_id: TypeId::Operator(OpType::Maximum),
            operands: vec![
                Packet::Literal(Literal {
                    version: Version(2),
                    type_id: TypeId::Literal,
                    num: 1,
                }),
                Packet::Literal(Literal {
                    version: Version(4),
                    type_id: TypeId::Literal,
                    num: 2,
                }),
                Packet::Literal(Literal {
                    version: Version(1),
                    type_id: TypeId::Literal,
                    num: 3,
                }),
            ],
        });

        assert_eq!(expected, packets)
    }

    #[test]
    fn part1_tests_1() {
        assert_eq!(16, part1("8A004A801A8002F478"));
    }
    #[test]
    fn part1_tests_2() {
        assert_eq!(12, part1("620080001611562C8802118E34"));
    }
    #[test]
    fn part1_tests_3() {
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
    }
    #[test]
    fn part1_tests_4() {
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day16.txt");
        let actual = part1(input.trim());

        assert_eq!(actual, 6);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day16.txt");
        let actual = part1(input.trim());

        assert_eq!(actual, 866);
    }

    #[test]
    fn part2_example() {
        let input = "9C0141080250320F1802104A08";
        let actual = Parser::new(input.trim()).read_packet().0;

        assert_eq!(actual.eval(), 1);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day16.txt");
        let actual = Parser::new(input.trim()).read_packet().0;

        assert_eq!(actual.eval(), 1392637195518);
    }
}
