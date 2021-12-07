use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day03.txt");
    let map = make_column_major_map(contents);

    let power_consumption = compute_power_consumption(&map)?;
    println!("(day 03) part 1: {}", power_consumption);

    let life_support_rating = compute_life_support_rating(contents);
    println!("(day 03) part 2: {}", life_support_rating);

    Ok(())
}

// FIXME: no unwraps :)
fn compute_life_support_rating(contents: &str) -> usize {
    let mut lines = contents.lines().collect::<Vec<&str>>();
    lines.sort_unstable();

    let oxygen = binary_search(lines.as_slice(), 0, Commonality::Most);
    let oxygen_rate = usize::from_str_radix(&oxygen, 2).unwrap();

    let co2 = binary_search(lines.as_slice(), 0, Commonality::Least);
    let co2_rate = usize::from_str_radix(&co2, 2).unwrap();

    oxygen_rate * co2_rate
}

#[derive(Debug, Copy, Clone)]
enum Commonality {
    Most,
    Least,
}

fn binary_search(lines: &[impl AsRef<str>], column: usize, commonality: Commonality) -> String {
    if lines.len() <= 1 {
        return lines[0].as_ref().to_string();
    }

    let half = (lines.len() - 1) / 2;
    let is_even = lines.len() % 2 == 0;

    let most_common = if is_even {
        select_even_half(lines, column, half)
    } else {
        select_uneven_half(lines, column, half)
    };

    // mirror, mirror, on the wall
    let most_common = if let Commonality::Least = commonality {
        if most_common == '0' {
            '1'
        } else {
            '0'
        }
    } else {
        most_common
    };

    let acceptable = lines
        .iter()
        .filter(|line| line.as_ref().chars().nth(column).unwrap() == most_common)
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<_>>();

    binary_search(&acceptable, column + 1, commonality)
}

fn select_even_half(lines: &[impl AsRef<str>], column: usize, half: usize) -> char {
    let lower = lines[half].as_ref().chars().nth(column).unwrap();
    let higher = lines[half + 1].as_ref().chars().nth(column).unwrap();

    if lower == higher {
        lower
    } else {
        '1'
    }
}

fn select_uneven_half(lines: &[impl AsRef<str>], column: usize, half: usize) -> char {
    lines[half].as_ref().chars().nth(column).unwrap()
}

fn compute_power_consumption(map: &ColumnMajorMap) -> anyhow::Result<usize> {
    let gamma = compute_gamma_rate(map)?;
    let epsilon = compute_epsilon_rate(map)?;

    Ok(gamma * epsilon)
}

fn compute_gamma_rate(map: &ColumnMajorMap) -> anyhow::Result<usize> {
    compute_rate(map, std::cmp::max)
}

fn compute_epsilon_rate(map: &ColumnMajorMap) -> anyhow::Result<usize> {
    compute_rate(map, std::cmp::min)
}

fn compute_rate(
    map: &ColumnMajorMap,
    comparator: fn(usize, usize) -> usize,
) -> anyhow::Result<usize> {
    let rate = map.iter().fold(String::new(), |mut acc, (_k, v)| {
        let most_common = commonality(v.as_str(), comparator);
        acc.push(most_common);
        acc
    });

    let decimal = usize::from_str_radix(&rate, 2)?;

    Ok(decimal)
}

type ColumnMajorMap<'s> = BTreeMap<usize, String>;

fn make_column_major_map(contents: &str) -> ColumnMajorMap {
    contents.lines().fold(ColumnMajorMap::new(), |outer, line| {
        line.chars()
            .enumerate()
            .fold(outer, |mut inner, (nth_char, c)| {
                inner.entry(nth_char).or_default().push(c);
                inner
            })
    })
}

fn commonality<F: Fn(usize, usize) -> usize>(slice: &str, f: F) -> char {
    let ones = slice.chars().filter(|c| *c == '1').count();
    let zeros = slice.len() - ones;

    let max = f(zeros, ones);

    if max == zeros {
        '0'
    } else {
        '1'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamma_rate() {
        let input = include_str!("../../inputs/example/day03.txt");
        let map = make_column_major_map(input);
        let actual = compute_gamma_rate(&map);

        assert_eq!(actual.unwrap(), 22);
    }

    #[test]
    fn epsilon_rate() {
        let input = include_str!("../../inputs/example/day03.txt");
        let map = make_column_major_map(input);
        let actual = compute_epsilon_rate(&map);

        assert_eq!(actual.unwrap(), 9);
    }

    #[yare::parameterized(
        max = { std::cmp::max, ['0', '0', '0', '1', '1'] },
        min = { std::cmp::min, ['1', '1', '1', '0', '0'] }
    )]
    fn commonality_test_max(comparator: fn(usize, usize) -> usize, expected: [char; 5]) {
        let a = commonality("00000", comparator);
        let b = commonality("00001", comparator);
        let c = commonality("01010", comparator);
        let d = commonality("10101", comparator);
        let e = commonality("11111", comparator);

        assert_eq!(a, expected[0]);
        assert_eq!(b, expected[1]);
        assert_eq!(c, expected[2]);
        assert_eq!(d, expected[3]);
        assert_eq!(e, expected[4]);
    }

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day03.txt");
        let map = make_column_major_map(input);
        let power = compute_power_consumption(&map);

        assert_eq!(power.unwrap(), 198);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day03.txt");
        let rate = compute_life_support_rating(input);

        assert_eq!(rate, 230);
    }

    #[test]
    fn part1_result() {
        let contents = std::fs::read_to_string("inputs/day03.txt").unwrap();
        let map = make_column_major_map(&contents);
        let power_consumption = compute_power_consumption(&map).unwrap();

        assert_eq!(power_consumption, 3009600);
    }

    #[test]
    fn part2_result() {
        let contents = std::fs::read_to_string("inputs/day03.txt").unwrap();
        let life_support_rating = compute_life_support_rating(&contents);

        assert_eq!(life_support_rating, 6940518);
    }
}
