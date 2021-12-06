use anyhow::{anyhow, Context};
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("inputs/day06.txt")?;
    let inputs = contents
        .trim()
        .split(',')
        .map(|n| {
            n.parse()
                .with_context(|| anyhow!("Unable to parse fish timer value: {}", n))
        })
        .collect::<anyhow::Result<Vec<u32>>>()?;

    let part1 = solve(inputs.iter(), 80);
    println!("(day 06) part 1: {}", part1);

    let part2 = solve(inputs.iter(), 256);
    println!("(day 06) part 2: {}", part2);

    Ok(())
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
