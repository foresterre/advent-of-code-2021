fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("inputs/day01.txt")?;
    let inputs = parse(&contents);

    let part1 = count_increasing_measurements(&inputs);
    println!("(day 01) part 1: {}", part1);

    let measurement_windows = create_measurement_windows(&inputs);
    let part2 = count_increasing_measurements(&measurement_windows);
    println!("(day 01) part 2: {}", part2);

    Ok(())
}

fn parse(input: &str) -> Vec<u16> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn count_increasing_measurements(inputs: &[u16]) -> usize {
    inputs
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn create_measurement_windows(inputs: &[u16]) -> Vec<u16> {
    inputs
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_increasing_measurements, create_measurement_windows, parse};

    #[test]
    fn part1_example() {
        let input = r#"199
200
208
210
200
207
240
269
260
263"#;
        let inputs = parse(input);
        let solution = count_increasing_measurements(&inputs);

        assert_eq!(solution, 7);
    }

    #[test]
    fn part2_example() {
        let input = r#"199
200
208
210
200
207
240
269
260
263"#;
        let inputs = parse(input);

        let measurement_windows = create_measurement_windows(&inputs);
        let solution = count_increasing_measurements(&measurement_windows);

        assert_eq!(solution, 5);
    }
}
