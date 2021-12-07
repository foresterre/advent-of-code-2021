use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day07.txt");
    let inputs = parse(contents)?;

    let part1 = find_alignment(&inputs, const_fuel)?;
    println!("(day 07) part 1: {}", part1);

    let part2 = find_alignment(&inputs, nth_triangle_fuel)?;
    println!("(day 07) part 2: {}", part2);

    Ok(())
}

fn const_fuel(alignment: i32, from: i32) -> i32 {
    (alignment - from).abs()
}

fn nth_triangle_fuel(alignment: i32, from: i32) -> i32 {
    let dist = (alignment - from).abs();
    dist * (dist + 1) / 2
}

fn parse(input: &str) -> anyhow::Result<Vec<i32>> {
    input
        .trim()
        .split(',')
        .map(|n| {
            n.parse()
                .with_context(|| anyhow!("Unable to parse value: {}", n))
        })
        .collect::<anyhow::Result<Vec<i32>>>()
}

fn find_alignment(crabs: &[i32], fuel: impl Fn(i32, i32) -> i32) -> anyhow::Result<i32> {
    let max = *crabs
        .iter()
        .max()
        .with_context(|| anyhow!("Unable to find alignment: no input values"))?;

    (0..=max)
        .map(|alignment| crabs.iter().map(|from| fuel(alignment, *from)).sum())
        .min()
        .with_context(|| anyhow!("Unable to find viable alignment: no input values"))
}

#[cfg(test)]
mod tests {
    use crate::{const_fuel, find_alignment, nth_triangle_fuel, parse};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day07.txt");
        let inputs = parse(input).unwrap();

        let solution = find_alignment(&inputs, const_fuel).unwrap();
        assert_eq!(solution, 37);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day07.txt");
        let inputs = parse(input).unwrap();

        let solution = find_alignment(&inputs, nth_triangle_fuel).unwrap();
        assert_eq!(solution, 168);
    }
}
