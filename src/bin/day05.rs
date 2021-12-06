use anyhow::{anyhow, Context};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("inputs/day05.txt")?;
    let inputs = parse(&contents)?;

    let part1 = part1(inputs.iter());
    println!("(day 05) part 1: {}", part1);

    let part2 = part2(inputs.iter());
    println!("(day 05) part 2: {}", part2);

    Ok(())
}

fn part1<'vent>(vents: impl Iterator<Item = &'vent HydrothermalVent>) -> usize {
    // Solution idea:
    // - Create an empty map, with on each coordinate a natural number, representing how often a
    //   a line covers that point.
    // - For each vent, 'draw' it onto the map, incrementing each cell which the vent covers.
    // - To find the number of points where to cells overlap, we iterate the map, filter
    //   where the cell value is >= 2, and count the residual.

    // First, create an empty map, I'll use an HashMap for now, as we don't know what the largest
    // coordinate components are. In addition, using an hashmap allows us to save storage where there
    // are no lines, assuming the map will be sparse. We may leave some performance on the table versus
    // a map which uses a contiguous array.
    let mut map = VentMap::new();

    // Now we draw the vents on the map
    vents.for_each(|vent| vent.draw_orthogonal_line(&mut map));

    // println!("{}", VentMapFmt(&map));

    // All left to do is find the cells which overlap, i.e. whose value is at least 2
    map.iter().filter(|(_, value)| **value >= 2).count()
}

fn part2<'vent>(vents: impl Iterator<Item = &'vent HydrothermalVent>) -> usize {
    // same for part 2 really, except we also need to draw diagonal lines
    let mut map = VentMap::new();

    vents.for_each(|vent| vent.draw_line(&mut map));

    // println!("{}", VentMapFmt(&map));

    map.iter().filter(|(_, value)| **value >= 2).count()
}

type VentMap = HashMap<Coord, i32>;

type HydrothermalVents = Vec<HydrothermalVent>;

fn parse(input: &str) -> anyhow::Result<HydrothermalVents> {
    input
        .lines()
        .map(HydrothermalVent::from_str)
        .collect::<anyhow::Result<_>>()
}

#[derive(Debug)]
struct HydrothermalVent {
    from: Coord,
    to: Coord,
}

impl HydrothermalVent {
    // Draw only horizontal or vertical lines
    fn draw_orthogonal_line(&self, map: &mut VentMap) {
        if self.from.x == self.to.x || self.from.y == self.to.y {
            self.draw_line(map)
        }
    }

    // Computes the coordinates which are between `from` and `to`, including `from` and `to`,
    // and draws them on the map; assumes horizontal, vertical or diagonal lines.
    fn draw_line(&self, map: &mut VentMap) {
        // slope
        let dx = (self.to.x - self.from.x).signum();
        let dy = (self.to.y - self.from.y).signum();

        let len = std::cmp::max(
            (self.to.x - self.from.x).abs(),
            (self.to.y - self.from.y).abs(),
        );

        (0..=len).for_each(|n| {
            *map.entry(Coord {
                x: self.from.x + n * dx,
                y: self.from.y + n * dy,
            })
            .or_default() += 1;
        });
    }
}

impl FromStr for HydrothermalVent {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (from, to) = line
            .split_once(" -> ")
            .with_context(|| anyhow!("Unable to split line into coordinates"))?;

        let from = from.parse()?;
        let to = to.parse()?;

        Ok(Self { from, to })
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(coord: &str) -> Result<Self, Self::Err> {
        let (x, y) = coord
            .split_once(',')
            .with_context(|| anyhow!("Unable to find two components for coordinate"))?;

        let x = x
            .parse()
            .with_context(|| anyhow!("Coordinate value (x) could not be parsed as a number"))?;

        let y = y
            .parse()
            .with_context(|| anyhow!("Coordinate value (x) could not be parsed as a number"))?;

        Ok(Self { x, y })
    }
}

// Not necessary for the solution, just to print the map :)
struct VentMapFmt<'map>(&'map VentMap);

impl std::fmt::Display for VentMapFmt<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.0.iter().map(|(coord, _)| coord.x).max().unwrap_or(0);

        let max_y = self.0.iter().map(|(coord, _)| coord.y).max().unwrap_or(0);

        // Can't collect directly into a table sadly :(
        let mut table = comfy_table::Table::new();

        let rows = (0..=max_y)
            .map(|y| {
                // Can't collect directly into a row sadly :(
                let cells = (0..=max_x)
                    .map(|x| {
                        if let Some(val) = self.0.get(&Coord { x, y }) {
                            comfy_table::Cell::new(val)
                        } else {
                            comfy_table::Cell::new("")
                        }
                    })
                    .collect::<Vec<comfy_table::Cell>>();

                let mut row = comfy_table::Row::new();

                for cell in cells {
                    row.add_cell(cell);
                }

                row
            })
            .collect::<Vec<comfy_table::Row>>();

        for row in rows {
            table.add_row(row);
        }

        std::fmt::Display::fmt(&table, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn part1_example() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        let inputs = parse(input).unwrap();
        let part1 = part1(inputs.iter());

        assert_eq!(part1, 5);
    }

    #[test]
    fn part2_example() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        let inputs = parse(input).unwrap();
        let part2 = part2(inputs.iter());

        assert_eq!(part2, 12);
    }
}
