use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day14.txt");
    let (polymer, rules) = parse(contents);

    let part1 = polymerize(&polymer, &rules, 10);
    println!("(day 14) part 1: {}", part1);

    let part2 = polymerize(&polymer, &rules, 40);
    println!("(day 14) part 2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Rule {
    window: (char, char),
    insertion: char,
}

fn parse(input: &str) -> (Vec<char>, Vec<Rule>) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let polymer = template.chars().collect();

    let rules = rules
        .lines()
        .map(|s| {
            let (window, insertion) = s.split_once(" -> ").unwrap();
            let mut chars = window.chars();
            let lhs = chars.next().unwrap();
            let rhs = chars.next().unwrap();
            let insertion = insertion.chars().next().unwrap();

            Rule {
                window: (lhs, rhs),
                insertion,
            }
        })
        .collect();

    (polymer, rules)
}

type PolymerPairCounter = HashMap<(char, char), usize>;

fn polymerize(polymer: &[char], rules: &[Rule], times: usize) -> usize {
    let init = polymer
        .windows(2)
        .fold(PolymerPairCounter::new(), |mut acc, slice| {
            *acc.entry((slice[0], slice[1])).or_default() += 1;
            acc
        });

    let map = (0..times).fold(init, |mut old_pairs, _| {
        // inner
        let new_pairs = rules
            .iter()
            .fold(PolymerPairCounter::new(), |mut acc, rule| {
                let (lhs, rhs) = rule.window;
                let middle = rule.insertion;
                let size = *old_pairs.get(&rule.window).unwrap_or(&0);
                *acc.entry((lhs, middle)).or_default() += size;
                *acc.entry((middle, rhs)).or_default() += size;
                old_pairs.insert((lhs, rhs), 0);

                acc
            });

        old_pairs
            .iter()
            .filter(|(_, &v)| v > 0)
            .fold(new_pairs, |mut acc, (&pos, &v)| {
                *acc.entry(pos).or_default() += v;
                acc
            })
    });

    let counter = map.iter().fold(
        HashMap::<char, usize>::new(),
        |mut acc, (&(l, r), &count)| {
            *acc.entry(l).or_default() += count;
            *acc.entry(r).or_default() += count;
            acc
        },
    );

    // We counted double because we used .windows(2) above
    // There are two edge cases: the first and last element, for which we have to add one, for being
    // only each in one 'window'.
    let halve = |item: char, count: usize| {
        if polymer[0] == item || polymer[polymer.len() - 1] == item {
            count / 2 + 1
        } else {
            count / 2
        }
    };

    let (max, min) = counter
        .iter()
        .map(|(&c, &n)| halve(c, n))
        .fold((usize::MIN, usize::MAX), |(max, min), count| {
            (max.max(count), min.min(count))
        });

    max - min
}

#[cfg(test)]
mod tests {
    use crate::{parse, polymerize};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day14.txt");
        let inputs = parse(input);
        let solution = polymerize(&inputs.0, &inputs.1, 10);

        assert_eq!(solution, 1588);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day14.txt");
        let inputs = parse(input);
        let solution = polymerize(&inputs.0, &inputs.1, 10);

        assert_eq!(solution, 3406);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day14.txt");
        let inputs = parse(input);
        let solution = polymerize(&inputs.0, &inputs.1, 40);

        assert_eq!(solution, 2188189693529);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day14.txt");
        let inputs = parse(input);
        let solution = polymerize(&inputs.0, &inputs.1, 40);

        assert_eq!(solution, 3941782230241);
    }
}
