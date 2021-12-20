use std::cmp::Ordering;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day17.txt").trim();

    let area = parse(input);

    let option = generate_simulations(&area);
    dbg!(option);

    println!("(day 17) part 1: {}", part1(&area));

    println!("(day 17) part 2: {}", -1);

    Ok(())
}

fn part1(area: &Area) -> isize {
    let yy = (area.min_y).abs() - 1;

    (yy * (yy + 1)) / 2
}

// How to do this one more cleverly than simulating each 'ray'
// At least we could try inversing the rays; but there's probably a way to calculate it?
fn part2(area: &Area) -> isize {
    // TODO: wip
    generate_simulations(area).unwrap()
}

fn generate_simulations(area: &Area) -> Option<isize> {
    for vy in (0_isize..10300).rev() {
        for vx in (-100_isize..100).rev() {
            let mut sim = Simulation::new(Step::new(Velocity::new(vx, vy)));
            if let Some(v) = sim.simulate(area, 200) {
                return Some(v);
            }
        }
    }
    None
}

struct Simulation {
    step: Step,
}

impl Simulation {
    fn new(initial_step: Step) -> Self {
        Self { step: initial_step }
    }

    fn simulate(&mut self, target: &Area, max_steps: usize) -> Option<isize> {
        for _ in 0..max_steps {
            if target.is_inside(self.step.position) {
                return Some(self.step.position.y);
            }

            self.step = self.step.next();
        }

        None
    }
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: isize,
    y: isize,
}

impl Velocity {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
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

    fn next(&self) -> Self {
        let Position { x: px, y: py } = self.position;
        let Velocity { x: vx, y: vy } = self.velocity;

        let new_vx = match vx.cmp(&0) {
            Ordering::Less => vx + 1,
            Ordering::Greater => vx - 1,
            Ordering::Equal => 0,
        };

        Self {
            position: Position {
                x: px + vx,
                y: py + vy,
            },
            velocity: Velocity {
                x: new_vx,
                y: vy - 1,
            },
        }
    }
}

#[derive(Debug)]
struct Area {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Area {
    fn new(x: (isize, isize), y: (isize, isize)) -> Self {
        Self {
            min_x: x.0.min(x.1),
            max_x: x.0.max(x.1),
            min_y: y.0.min(y.1),
            max_y: y.0.max(y.1),
        }
    }

    fn is_inside(&self, position: Position) -> bool {
        (self.min_x <= position.x && position.x <= self.max_x)
            && (self.max_y <= position.y && position.y <= self.max_y)
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
    use crate::{parse, part1, Area};

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

        assert_eq!(input.len(), 0);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day17.txt");

        assert_eq!(input.len(), 0);
    }
}
