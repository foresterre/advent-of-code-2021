use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day17.txt").trim();

    let target = parse(input);

    println!("(day 17) part 1: {}", part1(&target));

    println!("(day 17) part 2: {}", part2(&target));

    Ok(())
}

// Simply find maximum possible vertical velocity using the given velocity formula.
// Then calculate height ('distance') by taking the sum of increasing integers.
fn part1(area: &Area) -> i32 {
    let yy = (area.min_y).abs() - 1;

    (yy * (yy + 1)) / 2
}

// How to do this one more cleverly than simulating each 'ray'
// At least we could try inverting the rays; but there's probably a way to calculate it?
fn part2(area: &Area) -> usize {
    Simulator::from_tuples((area.min_x - 90, area.max_x + 0), (-150, 150)).run_simulations(area)
}

struct Simulator {
    xx: (i32, i32),
    yy: (i32, i32),
}

impl Simulator {
    fn from_tuples(xx: (i32, i32), yy: (i32, i32)) -> Self {
        Self { xx, yy }
    }

    fn run_simulations(&self, area: &Area) -> usize {
        (self.xx.0..=self.xx.1)
            .map(|vx| {
                (self.yy.0..=self.yy.1)
                    .filter(|&vy| Simulation::new(Step::new(Velocity::new(vx, vy))).simulate(area))
                    .count()
            })
            .sum()
    }
}

struct Simulation {
    step: Step,
}

impl Simulation {
    fn new(initial_step: Step) -> Self {
        Self { step: initial_step }
    }

    fn simulate(&self, target: &Area) -> bool {
        self.step
            .into_iter()
            .take_while(|step| {
                let restriction_x = step.position.x <= target.max_x;
                let restriction_y = step.position.y >= target.min_y;

                restriction_x && restriction_y
            })
            .any(|step| target.is_inside(step.position))
    }
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
struct Step {
    position: Position,
    velocity: Velocity,
}

impl Step {
    fn new(velocity: Velocity) -> Self {
        Self {
            velocity,
            position: Position::default(),
        }
    }
}

impl Iterator for Step {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let Position { x: px, y: py } = self.position;
        let Velocity { x: vx, y: vy } = self.velocity;

        let new_vx = match vx.cmp(&0) {
            Ordering::Less => vx + 1,
            Ordering::Greater => vx - 1,
            Ordering::Equal => 0,
        };

        self.position = Position {
            x: px + vx,
            y: py + vy,
        };

        self.velocity = Velocity {
            x: new_vx,
            y: vy - 1,
        };

        Some(*self)
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "p = ({}, {}), v = ({}, {})",
            self.position.x, self.position.y, self.velocity.x, self.velocity.y
        ))
    }
}

#[derive(Debug)]
struct Area {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Area {
    fn new(x: (i32, i32), y: (i32, i32)) -> Self {
        Self {
            min_x: x.0.min(x.1),
            max_x: x.0.max(x.1),
            min_y: y.0.min(y.1),
            max_y: y.0.max(y.1),
        }
    }

    fn is_inside(&self, position: Position) -> bool {
        (position.x >= self.min_x && position.x <= self.max_x)
            && (position.y >= self.min_y && position.y <= self.max_y)
    }
}

fn parse(input: &str) -> Area {
    let (x_side, y_side) = input.trim().split_once(", ").unwrap();
    let (xh, xr) = x_side.split_once("..").unwrap();
    let (yh, yr) = y_side.split_once("..").unwrap();

    let (_, xl) = xh.split_once('=').unwrap();
    let (_, yl) = yh.split_once('=').unwrap();

    let xl = xl.parse().unwrap();
    let yl = yl.parse().unwrap();
    let xr = xr.parse().unwrap();
    let yr = yr.parse().unwrap();

    Area::new((xl, xr), (yl, yr))
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day17.txt");
        let area = parse(input);

        assert_eq!(part1(&area), 45);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day17.txt");
        let area = parse(input);

        assert_eq!(part1(&area), 10296);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day17.txt");
        let area = parse(input);

        assert_eq!(part2(&area), 112);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day17.txt");
        let area = parse(input);

        assert_eq!(part2(&area), 2371);
    }
}
