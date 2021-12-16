use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use std::usize;

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day15.txt");
    let graph = Graph::from_str(contents.trim())?;
    let Size { width, height } = graph.size;

    println!(
        "(day 15) part 1: {}",
        graph
            .shortest_path((0, 0), (width - 1, height - 1))
            .unwrap()
    );

    let graph = Graph::from_str_tiled(contents.trim(), 5)?;
    let Size { width, height } = graph.size;

    println!(
        "(day 15) part 2: {}",
        graph
            .shortest_path((0, 0), (width - 1, height - 1))
            .unwrap()
    );

    Ok(())
}

type RiskLevel = u8;

#[derive(Debug)]
struct Graph {
    vertices: Vec<RiskLevel>,
    size: Size,
}

impl Graph {
    fn new(vertices: Vec<RiskLevel>, size: Size) -> Self {
        Self { vertices, size }
    }

    fn risk_level(&self, index: usize) -> RiskLevel {
        self.vertices[index]
    }

    // Scales the map as received in the input by `scale_by` in both
    // the x and y axis. In addition, the risk level for each right or downwards
    // tile is incremented (wrapped after 9, back to 1) for each position on each
    // successive tile.
    fn from_str_tiled(input: &str, scale_by: u8) -> anyhow::Result<Self> {
        let input_width = input_width(input);
        let input_height = input_height(input);

        let width = input_width * usize::from(scale_by);
        let height = input_height * usize::from(scale_by);

        let size = Size { width, height };

        let vertices =
            input
                .lines()
                .enumerate()
                .fold(vec![u8::MAX; width * height], |mut map, (y, line)| {
                    line.bytes().enumerate().for_each(|(x, risk)| {
                        let risk_level = risk - b'0';

                        for factor_x in 0_u8..scale_by {
                            for factor_y in 0_u8..scale_by {
                                let new_risk = {
                                    let risk = risk_level + factor_x + factor_y;
                                    if risk < 9 {
                                        risk
                                    } else {
                                        (risk - 1) % 9 + 1
                                    }
                                };

                                let scaled_x = (usize::from(factor_x) * input_width) + x;
                                let scaled_y = (usize::from(factor_y) * input_height) + y;
                                let index = size.index(scaled_x, scaled_y);

                                map[index] = new_risk;
                            }
                        }
                    });

                    map
                });

        Ok(Self::new(vertices, size))
    }
}

#[derive(Debug)]
struct Size {
    width: usize,
    height: usize,
}

impl Size {
    // Compute the index of a given xy-coordinate.
    fn index(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let width = input_width(contents);
        let height = input_height(contents);
        let size = Size { width, height };

        let vertices = contents.lines().enumerate().fold(
            vec![u8::MAX; width * height],
            |mut map, (y, line)| {
                line.bytes().enumerate().for_each(|(x, risk)| {
                    map[size.index(x, y)] = risk - b'0';
                });

                map
            },
        );

        Ok(Self::new(vertices, size))
    }
}

impl Graph {
    fn shortest_path(&self, s: (usize, usize), t: (usize, usize)) -> Option<u32> {
        let mut pq = BinaryHeap::new();
        let mut visited = vec![false; self.vertices.len()];

        pq.push(Node {
            position: s,
            distance: 0,
        });

        visited[self.size.index(s.0, s.1)] = true;

        while let Some(Node { position, distance }) = pq.pop() {
            if position == t {
                return Some(distance);
            }

            for (x, y) in neighbours(position, &self.size) {
                let index = self.size.index(x, y);

                if !visited[index] {
                    visited[index] = true;

                    let risk = self.risk_level(index);
                    let travelled = distance + u32::from(risk);

                    // println!(
                    //     "visited: {},{} ({}) = {}, travelled = {}",
                    //     x, y, index, risk, travelled
                    // );

                    pq.push(Node {
                        position: (x, y),
                        distance: travelled,
                    })
                }
            }
        }

        None
    }
}

// Need to wrap the traveled distance so we can custom balance the binary heap, while
// still referring back to it's coordinates.
#[derive(Debug)]
struct Node {
    position: (usize, usize),
    distance: u32,
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        other.distance.eq(&self.distance)
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn neighbours(middle: (usize, usize), size: &Size) -> Vec<(usize, usize)> {
    let width = size.width as isize;
    let height = size.height as isize;

    let relative = |position: (usize, usize), offset: (isize, isize)| -> Option<(usize, usize)> {
        let within_width = |x: isize| (0_isize..width).contains(&x);
        let within_height = |y: isize| (0_isize..height).contains(&y);

        let x = (position.0 as isize)
            .checked_add(offset.0)
            .filter(|&x| within_width(x));
        let y = (position.1 as isize)
            .checked_add(offset.1)
            .filter(|&y| within_height(y));

        x.zip(y).map(|(x, y)| (x as usize, y as usize))
    };

    [(1, 0), (0, 1), (0, -1), (-1, 0)]
        .iter()
        .filter_map(|&offset| relative(middle, offset))
        .collect()
}

fn input_width(input: &str) -> usize {
    input
        .trim()
        .lines()
        .next()
        .map(|c| c.bytes().count())
        .unwrap_or(0)
}

fn input_height(input: &str) -> usize {
    input.trim().lines().count()
}

#[cfg(test)]
mod tests {
    use crate::{Graph, Size};
    use std::str::FromStr;

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day15.txt");
        let graph = Graph::from_str(input.trim()).unwrap();
        let Size { width, height } = graph.size;
        let s = (0, 0);
        let t = (width - 1, height - 1);

        assert_eq!(graph.shortest_path(s, t).unwrap(), 40);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day15.txt");
        let graph = Graph::from_str(input.trim()).unwrap();
        let Size { width, height } = graph.size;
        let s = (0, 0);
        let t = (width - 1, height - 1);

        assert_eq!(graph.shortest_path(s, t).unwrap(), 755);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day15.txt");
        let graph = Graph::from_str_tiled(input.trim(), 5).unwrap();
        let Size { width, height } = graph.size;
        let s = (0, 0);
        let t = (width - 1, height - 1);

        assert_eq!(graph.shortest_path(s, t).unwrap(), 315);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day15.txt");
        let graph = Graph::from_str_tiled(input.trim(), 5).unwrap();
        let Size { width, height } = graph.size;
        let s = (0, 0);
        let t = (width - 1, height - 1);

        assert_eq!(graph.shortest_path(s, t).unwrap(), 3016);
    }
}
