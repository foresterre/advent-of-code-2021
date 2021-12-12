use comfy_table::{Cell, Row, Table};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day11.txt");
    let part1 = part1(contents);

    println!("(day 11) part 1: {}", part1);

    let part2 = part2(contents);
    println!("(day 11) part 2: {}", part2);

    Ok(())
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input);

    (0..100)
        .map(|_| {
            grid.step();
            grid.count_flashes()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_str(input);

    std::iter::repeat(())
        .enumerate()
        .find(|_| {
            grid.step();
            grid.flashed_simultaneously()
        })
        .map(|(i, _)| i + 1)
        .unwrap()
}

struct Grid {
    map: HashMap<(usize, usize), Octopus>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(j, b)| ((i, j), Octopus::new(*b - b'0')))
                    .collect::<Vec<((usize, usize), Octopus)>>()
            })
            .collect::<HashMap<(usize, usize), Octopus>>();

        Self { map }
    }

    fn step(&mut self) {
        // First, the energy level of each octopus increases by 1.
        self.map
            .values_mut()
            .for_each(|octopus| octopus.raise_energy_level());

        // Then, any octopus with an energy level greater than 9 flashes
        while let Some((&center, octopus)) = self.needs_update() {
            // This increases the energy level of all adjacent octopuses by 1,
            octopus.flash();

            // If this causes an octopus to have an energy level greater than 9, it also flashes
            self.increase_neighbour_energy_levels(center);
        }

        self.reset();
    }

    fn needs_update(&mut self) -> Option<(&(usize, usize), &mut Octopus)> {
        self.map.iter_mut().find(|(_k, v)| v.should_flash())
    }

    fn increase_neighbour_energy_levels(&mut self, center: (usize, usize)) {
        for i in -1..=1 {
            for j in -1..=1 {
                let xx = ((center.0 as isize) + i) as usize;
                let yy = ((center.1 as isize) + j) as usize;

                if let Some(neighbour) = self.map.get_mut(&(xx, yy)) {
                    neighbour.raise_energy_level();
                }
            }
        }
    }

    fn reset(&mut self) {
        self.map
            .iter_mut()
            .for_each(|(_, octopus)| octopus.reset_did_flash());
    }

    /// Requires generation to be complete
    fn count_flashes(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, octopus)| octopus.energy_level == 0)
            .count()
    }

    fn flashed_simultaneously(&self) -> bool {
        self.map.len() == self.count_flashes()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ii = self.map.keys().map(|(l, _r)| *l).max().unwrap_or(0);

        let jj = self.map.keys().map(|(_l, r)| *r).max().unwrap_or(0);

        let mut table = Table::new();

        for i in 0_usize..=ii {
            let mut row = Row::new();

            for j in 0_usize..=jj {
                row.add_cell(Cell::new(format!("{}", self.map.get(&(i, j)).unwrap())));
            }
            table.add_row(row);
        }

        std::fmt::Display::fmt(&table, f)
    }
}

#[derive(Debug)]
struct Octopus {
    energy_level: u8,
    did_flash: bool,
}

impl Octopus {
    fn new(energy_level: u8) -> Self {
        Self {
            energy_level,
            did_flash: false,
        }
    }

    /// Raises the energy level by one
    fn raise_energy_level(&mut self) {
        if !self.did_flash {
            self.energy_level += 1;
        }
    }

    fn should_flash(&self) -> bool {
        self.energy_level > 9 && !self.did_flash
    }

    fn flash(&mut self) {
        self.energy_level = 0;
        self.did_flash = true;
    }

    fn reset_did_flash(&mut self) {
        self.did_flash = false;
    }
}

impl std::fmt::Display for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.energy_level))
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day11.txt");

        assert_eq!(part1(input), 1656);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day11.txt");

        assert_eq!(part1(input), 1735);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day11.txt");

        assert_eq!(part2(input), 195);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day11.txt");

        assert_eq!(part2(input), 400);
    }
}
