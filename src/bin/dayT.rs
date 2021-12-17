fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/dayT.txt").trim();

    println!("(day T) part 1: {}", -1);

    println!("(day T) part 2: {}", -1);

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/dayT.txt");

        assert_eq!(input.len(), 1);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/dayT.txt");

        assert_eq!(input.len(), 755);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/dayT.txt");

        assert_eq!(input.len(), 315);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/dayT.txt");

        assert_eq!(input.len(), 3016);
    }
}
