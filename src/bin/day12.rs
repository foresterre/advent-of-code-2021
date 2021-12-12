use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day12.txt");
    let graph = Graph::from_str(contents);

    let part1 = part1(&graph);

    println!("(day 12) part 1: {}", part1);

    let part2 = part2(&graph);
    println!("(day 12) part 2: {}", part2);

    Ok(())
}

fn part1(graph: &Graph) -> usize {
    let cap = graph.vertices.len() * 2;

    graph.dfs(Vec::with_capacity(cap), "start", true)
}

fn part2(graph: &Graph) -> usize {
    let cap = graph.vertices.len() * 2;

    graph.dfs(Vec::with_capacity(cap), "start", false)
}

type Map<'s> = HashMap<&'s str, Vec<&'s str>>;

struct Graph<'s> {
    vertices: Map<'s>,
}

impl<'s> Graph<'s> {
    fn from_str(input: &'s str) -> Self {
        let mut map = Map::new();

        input.lines().for_each(|c| {
            let (s, t) = c.split_once('-').unwrap();
            map.entry(s).or_default().push(t);
            map.entry(t).or_default().push(s);
        });

        Self { vertices: map }
    }

    fn dfs(&self, mut visited: Vec<&'s str>, current: &'s str, mut visited_twice: bool) -> usize {
        if current == "end" {
            return 1;
        }

        if visited.contains(&current) {
            if current == "start" {
                return 0;
            }

            match current.is_small() {
                true if visited_twice => return 0,
                true => visited_twice = true,
                false => {}
            }
        }

        visited.push(current);

        self.neighbours(current)
            .iter()
            .map(|cave| self.dfs(visited.clone(), cave, visited_twice))
            .sum()
    }

    fn neighbours(&self, node: &'s str) -> &Vec<&'s str> {
        self.vertices.get(node).unwrap()
    }
}

trait Cave {
    fn is_small(&self) -> bool;
}

impl<'s> Cave for &'s str {
    fn is_small(&self) -> bool {
        self.chars().next().unwrap().is_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Graph};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day12.txt");
        let input = Graph::from_str(input);

        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day12.txt");
        let input = Graph::from_str(input);

        assert_eq!(part1(&input), 3761);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day12.txt");
        let input = Graph::from_str(input);

        assert_eq!(part2(&input), 36);
    }
    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day12.txt");
        let input = Graph::from_str(input);

        assert_eq!(part2(&input), 99138);
    }
}
