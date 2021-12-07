use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day06.txt");
    let inputs = parse(contents)?;

    let part1 = solve(inputs.iter(), 80);
    println!("(day 06) part 1: {}", part1);

    let part2 = solve(inputs.iter(), 256);
    println!("(day 06) part 2: {}", part2);

    Ok(())
}

fn parse(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .trim()
        .split(',')
        .map(|n| {
            n.parse()
                .with_context(|| anyhow!("Unable to parse fish timer value: {}", n))
        })
        .collect::<anyhow::Result<Vec<u32>>>()
}

fn solve<'f>(seedlings: impl Iterator<Item = &'f u32>, days: u32) -> usize {
    let mut fish = [0usize; 9];

    // initial fish
    for f in seedlings {
        fish[*f as usize] += 1;
    }

    for _day in 0..days {
        let new = fish[0];

        // no need to loop for 9 updates <3
        fish[0] = fish[1];
        fish[1] = fish[2];
        fish[2] = fish[3];
        fish[3] = fish[4];
        fish[4] = fish[5];
        fish[5] = fish[6];
        fish[6] = fish[7] + new;
        fish[7] = fish[8];
        fish[8] = new;
    }

    fish.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::{parse, solve};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day06.txt");
        let inputs = parse(input).unwrap();

        let solution = solve(inputs.iter(), 80);
        assert_eq!(solution, 5934);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day06.txt");
        let inputs = parse(input).unwrap();

        let solution = solve(inputs.iter(), 256);
        assert_eq!(solution, 26984457539);
    }
}
