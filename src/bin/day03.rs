use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("inputs/day03.txt")?;

    let map = make_column_major_map(&contents);
    let power_consumption = compute_power_consumption(&map)?;

    println!("(day 03) part 1: {}", power_consumption);
    println!("(day 03) part 2: {}", 2);

    Ok(())
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
        commonality(v.as_str(), comparator, &mut acc);
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

fn commonality<F: Fn(usize, usize) -> usize>(slice: &str, f: F, output: &mut String) {
    let ones = slice.chars().filter(|c| *c == '1').count();
    let zeros = slice.len() - ones;

    let max = f(zeros, ones);

    if max == zeros {
        output.push('0');
    } else {
        output.push('1');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn gamma_rate() {
        let map = make_column_major_map(EXAMPLE_INPUT);
        let actual = compute_gamma_rate(&map);

        assert_eq!(actual.unwrap(), 22);
    }

    #[test]
    fn epsilon_rate() {
        let map = make_column_major_map(EXAMPLE_INPUT);
        let actual = compute_epsilon_rate(&map);

        assert_eq!(actual.unwrap(), 9);
    }

    #[yare::parameterized(
        max = { std::cmp::max, ['0', '0', '0', '1', '1'] },
        min = { std::cmp::min, ['1', '1', '1', '0', '0'] }
    )]
    fn commonality_test_max(comparator: fn(usize, usize) -> usize, expected: [char; 5]) {
        let mut output = String::new();
        commonality("00000", comparator, &mut output);
        commonality("00001", comparator, &mut output);
        commonality("01010", comparator, &mut output);
        commonality("10101", comparator, &mut output);
        commonality("11111", comparator, &mut output);

        let chars = output.chars().collect::<Vec<_>>();

        assert_eq!(chars[0], expected[0]);
        assert_eq!(chars[1], expected[1]);
        assert_eq!(chars[2], expected[2]);
        assert_eq!(chars[3], expected[3]);
        assert_eq!(chars[4], expected[4]);
    }

    #[test]
    fn part1_example() {
        let map = make_column_major_map(EXAMPLE_INPUT);
        let power = compute_power_consumption(&map);

        assert_eq!(power.unwrap(), 198);
    }
}
