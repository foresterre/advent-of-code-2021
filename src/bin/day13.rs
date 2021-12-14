use anyhow::{anyhow, bail, Context};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day13.txt");

    let (dots, folds) = parse(contents)?;
    let part1 = part1(&dots, &folds[0]);
    println!("(day 13) part 1: {}", part1);

    println!("(day 13) part 2:");
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    part2(dots, folds.iter(), &mut lock)?;

    Ok(())
}

// parse to coordinates (dots) and instructions (folds)
fn parse(input: &str) -> anyhow::Result<(HashSet<Dot>, Vec<Fold>)> {
    let (dots, folds) = input.split_once("\n\n").unwrap();

    let dots = dots
        .lines()
        .map(|line| line.parse::<Dot>())
        .collect::<anyhow::Result<HashSet<_>>>()?;

    let folds = folds
        .lines()
        .map(|line| line.parse::<Fold>())
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok((dots, folds))
}

fn part1(dots: &HashSet<Dot>, fold: &Fold) -> usize {
    fold.fold(dots.iter()).len()
}

fn part2<'f, W: Write>(
    dots: HashSet<Dot>,
    folds: impl Iterator<Item = &'f Fold>,
    output: &mut W,
) -> anyhow::Result<()> {
    let code = folds.fold(dots, |acc, next| next.fold(acc.iter()));
    let code = PaperFmt::new(&code);

    writeln!(output, "{}", code).with_context(|| anyhow!("Unable to write code to output"))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Dot(i32, i32);

impl Fold {
    fn fold<'dot>(&self, dots: impl Iterator<Item = &'dot Dot>) -> HashSet<Dot> {
        #[inline]
        fn shift(pos: i32, by: i32) -> i32 {
            by - (by - pos).abs()
        }

        dots.into_iter().fold(HashSet::new(), |mut acc, dot| {
            let res = match *self {
                Fold::Up(by) => Dot(shift(dot.0, by), dot.1),
                Fold::Left(by) => Dot(dot.0, shift(dot.1, by)),
            };

            acc.insert(res);
            acc
        })
    }
}

impl FromStr for Dot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .with_context(|| anyhow!("Unable to parse dot coordinate from '{}'", s))?;

        let x = x
            .parse()
            .with_context(|| anyhow!("Unable to parse x coordinate from '{}'", x))?;
        let y = y
            .parse()
            .with_context(|| anyhow!("Unable to parse y coordinate from '{}'", x))?;

        Ok(Dot(x, y))
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    Up(i32),
    Left(i32),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s
            .split_once('=')
            .with_context(|| anyhow!("Unable to parse fold instruction from '{}'", s))?;

        let value = value
            .parse()
            .with_context(|| anyhow!("Unable to parse fold value from '{}'", value))?;

        Ok(match axis {
            "fold along y" => Fold::Left(value),
            "fold along x" => Fold::Up(value),
            _ => bail!("Invalid fold instruction '{}'", axis),
        })
    }
}

struct PaperFmt<'s> {
    dots: &'s HashSet<Dot>,
}

impl<'s> PaperFmt<'s> {
    fn new(dots: &'s HashSet<Dot>) -> Self {
        Self { dots }
    }
}

impl Display for PaperFmt<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let xx = self
            .dots
            .iter()
            .max_by_key(|dot| dot.0)
            .map(|dot| dot.0)
            .unwrap_or(0);
        let yy = self
            .dots
            .iter()
            .max_by_key(|dot| dot.1)
            .map(|dot| dot.1)
            .unwrap_or(0);

        for y in 0..=yy {
            for x in 0..=xx {
                if self.dots.contains(&Dot(x, y)) {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn part1_example() {
        let contents = include_str!("../../inputs/example/day13.txt");
        let (dots, folds) = parse(contents).unwrap();

        let part1 = part1(&dots, &folds[0]);
        assert_eq!(part1, 17);
    }

    #[test]
    fn part1_solution() {
        let contents = include_str!("../../inputs/day13.txt");
        let (dots, folds) = parse(contents).unwrap();

        let solution = part1(&dots, &folds[0]);

        assert_eq!(solution, 607);
    }

    #[test]
    fn part2_example() {
        let contents = include_str!("../../inputs/example/day13.txt");
        let (dots, folds) = parse(contents).unwrap();

        let mut buffer = Vec::new();
        part2(dots, folds.iter(), &mut buffer).unwrap();
        let code = String::from_utf8(buffer).unwrap();

        let expected = r#"
#####
#...#
#...#
#...#
#####"#;

        assert_eq!(code.trim(), expected.trim());
    }

    #[test]
    fn part2_solution() {
        let contents = include_str!("../../inputs/day13.txt");
        let (dots, folds) = parse(contents).unwrap();

        let mut buffer = Vec::new();
        part2(dots, folds.iter(), &mut buffer).unwrap();
        let code = String::from_utf8(buffer).unwrap();

        let expected = r#"
.##..###..####.#....###..####.####.#...
#..#.#..#....#.#....#..#.#.......#.#...
#....#..#...#..#....#..#.###....#..#...
#....###...#...#....###..#.....#...#...
#..#.#....#....#....#....#....#....#...
.##..#....####.####.#....#....####.####"#;

        assert_eq!(code.trim(), expected.trim());
    }
}
