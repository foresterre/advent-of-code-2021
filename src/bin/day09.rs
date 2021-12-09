use comfy_table::{Cell, Row, Table};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

fn main() -> anyhow::Result<()> {
    let contents = include_bytes!("../../inputs/day09.txt");
    let part1 = part1(contents);
    println!("(day 09) part 1: {}", part1);

    let part2 = part2(contents);
    println!("(day 09) part 2: {}", part2);

    Ok(())
}

fn width(input: &[u8]) -> usize {
    // let width = input.splitn(2, |b| *b == b'\n').next().unwrap().len();
    String::from_utf8_lossy(input)
        .as_ref()
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
}

fn parse(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            _ => None,
        })
        .collect::<Vec<u8>>()
}

fn part1(input: &[u8]) -> u32 {
    let width = width(input);
    let map = parse(input);

    map.iter()
        .enumerate()
        .filter(|&(index, _value)| {
            adjacent(&map, index, width, usize::checked_sub)
                && adjacent(&map, index, width, usize::checked_add)
                && (index % width == 0 || adjacent(&map, index, 1, usize::checked_sub))
                && (index % width == width - 1 || adjacent(&map, index, 1, usize::checked_add))
        })
        .map(|(_i, value)| u32::from(value + 1))
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let width = width(input);
    let map = parse(input);

    let low_points = map
        .iter()
        .enumerate()
        .filter(|&(index, _value)| is_low_point(width, &map, index))
        .map(|(i, _value)| i)
        .collect::<Vec<_>>();

    let mut set = low_points
        .iter()
        .map(|&low| {
            let mut seen = HashSet::new();
            check_neighbours(&map, low, width, &mut seen);
            seen.len()
        })
        .collect::<Vec<_>>();

    set.sort_unstable();

    set.iter().rev().take(3).product()
}

fn is_low_point(width: usize, map: &[u8], index: usize) -> bool {
    adjacent(&map, index, width, usize::checked_sub)
        && adjacent(&map, index, width, usize::checked_add)
        && (index % width == 0 || adjacent(&map, index, 1, usize::checked_sub))
        && (index % width == width - 1 || adjacent(&map, index, 1, usize::checked_add))
}

fn check_neighbours(map: &[u8], index: usize, width: usize, seen: &mut HashSet<usize>) {
    if seen.contains(&index) {
        return;
    }

    seen.insert(index);

    // up
    in_basin(map, index, width, usize::checked_sub)
        .map(|index| check_neighbours(map, index, width, seen));

    // down
    in_basin(&map, index, width, usize::checked_add)
        .map(|index| check_neighbours(map, index, width, seen));

    // left
    (index % width != 0).then(|| {
        in_basin(&map, index, 1, usize::checked_sub)
            .map(|index| check_neighbours(map, index, width, seen))
    });

    // right
    (index % width != width - 1).then(|| {
        in_basin(&map, index, 1, usize::checked_add)
            .map(|index| check_neighbours(map, index, width, seen))
    });
}

fn adjacent(
    map: &[u8],
    index: usize,
    width: usize,
    f: impl Fn(usize, usize) -> Option<usize>,
) -> bool {
    f(index, width)
        .and_then(|adjacent| map.get(adjacent))
        .map(|&val| map[index] < val)
        .unwrap_or(true)
}

fn in_basin(
    map: &[u8],
    index: usize,
    width: usize,
    f: impl Fn(usize, usize) -> Option<usize>,
) -> Option<usize> {
    let adjacent = f(index, width);

    adjacent
        .and_then(|adjacent| map.get(adjacent).map(|val| (val, adjacent)))
        .filter(|&(&val, _index)| val != 9)
        .map(|(_, index)| index)
}

#[derive(Debug)]
struct HeightMapFmt<'b> {
    map: &'b [u8],
    index: usize,
    width: usize,
}

impl<'b> HeightMapFmt<'b> {
    #[allow(unused)]
    fn new(map: &'b [u8], highlighted_index: usize, width: usize) -> Self {
        Self {
            map,
            index: highlighted_index,
            width,
        }
    }
}

impl<'b> Display for HeightMapFmt<'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        let mut i = 0_usize;
        self.map.chunks(self.width).for_each(|row_cells| {
            let mut row = Row::new();

            row_cells.iter().for_each(|value| {
                if i == self.index {
                    row.add_cell(Cell::new(format!("{} *", *value)));
                } else {
                    row.add_cell(Cell::new(format!("{}  ", *value)));
                }
                i += 1;
            });

            table.add_row(row);
        });

        f.write_str("\n\n")?;
        std::fmt::Display::fmt(&table, f)
    }
}

#[cfg(test)]
mod tests_part2 {
    use crate::part2;

    #[test]
    fn part2_example() {
        let input = include_bytes!("../../inputs/example/day09.txt");

        assert_eq!(part2(input), 1134);
    }
}

#[cfg(test)]
mod tests_part1 {
    use crate::part1;

    #[test]
    fn part1_example() {
        let input = include_bytes!("../../inputs/example/day09.txt");

        assert_eq!(part1(input), 15);
    }

    #[test]
    fn one_cell() {
        let input = b"1";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn two_corners() {
        let input = b"12\n31";

        assert_eq!(part1(input), 4);
    }

    #[test]
    fn none() {
        let input = b"11\n11";

        assert_eq!(part1(input), 0);
    }

    #[test]
    fn one_in_the_middle() {
        let input = b"111\n101\n111";

        assert_eq!(part1(input), 1);
    }

    #[test]
    fn one_in_the_middle_but_no_valley() {
        let input = b"111\n121\n111";

        assert_eq!(part1(input), 0);
    }
    #[test]
    fn star() {
        let input = b"010\n121\n010";

        assert_eq!(part1(input), 4);
    }
}
