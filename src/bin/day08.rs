use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day08.txt");
    let part1 = part1(contents);

    // let part1 = find_alignment(&inputs, const_fuel)?;
    println!("(day 08) part 1: {}", part1);

    // let part2 = ...
    // println!("(day 08) part 2: {}", part2);

    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .map(|(_, r)| r)
        .map(|segments| -> usize {
            segments
                .split_ascii_whitespace()
                .map(|segment| segment.len())
                .filter(|len|
                    // digit 1 = 2 segments
                    // digit 7 = 3 segments
                    // digit 4 = 4 segments
                    // digit 8 = 7 segments
                    matches!(*len, 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day08.txt");

        assert_eq!(part1(input), 26);
    }
    //
    // #[test]
    // fn part2_example() {
    //     let input = include_str!("../../inputs/example/day07.txt");
    //     let inputs = parse(input).unwrap();
    //
    //     let solution = find_alignment(&inputs, nth_triangle_fuel).unwrap();
    //     assert_eq!(solution, 168);
    // }
}
