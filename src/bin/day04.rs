use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

// close your eyes, nothing to see here <3
fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("inputs/day04.txt")?;
    let (instructions, mut boards) = parse(&contents)?;

    let score =
        part1(&instructions.0, &mut boards).with_context(|| anyhow!("No winning board found!"))?;
    println!("(day 04) part 1: {}", score);

    boards.iter_mut().for_each(|board| board.reset());
    let score =
        part2(&instructions.0, &mut boards).with_context(|| anyhow!("No winning board found!"))?;
    println!("(day 04) part 2: {}", score);

    Ok(())
}

const CHUNK_SEPARATOR: &'static str = "\n\n";
const LEN: usize = 5;

fn parse(input: &str) -> Result<(Instructions, Vec<Board>)> {
    let (instructions, board) = input
        .split_once(CHUNK_SEPARATOR)
        .with_context(|| anyhow!("No instructions found"))?;

    let instructions = Instructions::from_str(instructions)?;

    let board = board
        .split(CHUNK_SEPARATOR)
        .map(Board::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok((instructions, board))
}

fn part1(instructions: &[u16], boards: &mut [Board]) -> Option<u16> {
    for instr in instructions {
        for board in boards.iter_mut() {
            board.mark(*instr);

            if board.is_winner() {
                println!("first winner\n{}", &board);

                return Some(board.unmarked_score() * instr);
            }
        }
    }

    None
}

fn part2(instructions: &[u16], boards: &mut [Board]) -> Option<u16> {
    let mut queue = (0..boards.len()).collect::<HashSet<_>>();

    for instr in instructions {
        for (n, board) in boards.iter_mut().enumerate() {
            board.mark(*instr);

            if board.is_winner() {
                queue.remove(&n);

                if queue.is_empty() {
                    println!("last winner\n{}", &board);

                    return Some(board.unmarked_score() * instr);
                }
            }
        }
    }

    None
}

#[derive(Debug)]
struct Instructions(Vec<u16>);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let result = line
            .split(',')
            .map(|item| {
                item.parse()
                    .with_context(|| anyhow!("Unable to parse an instruction"))
            })
            .collect::<Result<_>>()?;

        Ok(Instructions(result))
    }
}

#[derive(Debug)]
struct Board {
    // 5x5 board, row-major
    cells: [Cell; LEN * LEN],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [Cell::default(); 25],
        }
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(items: &str) -> Result<Self> {
        let items = items
            .split_ascii_whitespace()
            .filter_map(|value| value.parse::<u16>().ok());

        let mut board = [Cell::default(); 25];

        for (n, value) in items.enumerate() {
            board[n].value = value;
        }

        Ok(Self { cells: board })
    }
}

impl Board {
    pub fn mark(&mut self, value: u16) {
        self.cells
            .iter_mut()
            .filter(|cell| cell.value == value)
            .for_each(|cell| cell.marked = true);

        for cell in self.cells.iter_mut() {
            if cell.value == value {
                cell.marked = true;
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        self.has_winning_row() || self.has_winning_column()
    }

    pub fn unmarked_score(&self) -> u16 {
        self.cells
            .iter()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.value)
            .sum()
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.marked = false;
        }
    }

    fn has_winning_row(&self) -> bool {
        let mut index = std::iter::successors(Some(0usize), |y| {
            let next = y + 5;
            if next >= LEN * LEN {
                return None;
            }

            Some(next)
        });

        // check whether a full row is marked, if it is, the board is winning
        index
            .find(|a| Self::is_winning_row(&self.cells, *a))
            .is_some()
    }

    fn is_winning_row(cells: &[Cell; LEN * LEN], row: usize) -> bool {
        (row..row + 5).all(|v| cells[v].marked)
    }

    fn has_winning_column(&self) -> bool {
        let mut index = std::iter::successors(Some(0usize), |x| {
            if *x >= 4 {
                return None;
            }
            Some(x + 1)
        });

        // check whether a full column is marked, if it is, the board is winning
        index
            .find(|a| Self::is_winning_column(&self.cells, *a))
            .is_some()
    }

    fn is_winning_column(cells: &[Cell; LEN * LEN], col: usize) -> bool {
        // too many off by one errors, so hand generated instead <3
        [0, 5, 10, 15, 20].iter().all(|v| cells[*v + col].marked)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = comfy_table::Table::new();

        for row in (0..LEN * LEN).step_by(5) {
            table.add_row(vec![
                comfy_table::Cell::new(self.cells[row]),
                comfy_table::Cell::new(self.cells[row + 1]),
                comfy_table::Cell::new(self.cells[row + 2]),
                comfy_table::Cell::new(self.cells[row + 3]),
                comfy_table::Cell::new(self.cells[row + 4]),
            ]);
        }

        std::fmt::Display::fmt(&table, f)
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    value: u16,
    marked: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: u16::MAX,
            // fixme: shouldn't have kept those inside the cell, great for visualization, but
            //  less so for the algorithm, as this field is the reason we need our board to be mutable.
            //  It's also quite unnecessary; we can just keep a collection of seen values, and check
            //  for each column/row whether it's in there or not; or use a bitvec.
            marked: false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.value)?;

        if self.marked {
            write!(f, "X")
        } else {
            write!(f, "")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, Board};

    yare::ide!();

    #[yare::parameterized(
        first_row = {  [0, 1, 2, 3, 4] },
        second_row = { [5, 6, 7, 8, 9] },
        third_row = { [10, 11, 12, 13, 14] },
        fourth_row = { [15, 16, 17, 18, 19] },
        fifth_row = { [20, 21, 22, 23, 24] },
    )]
    fn winning_row(which: [usize; 5]) {
        let mut board = Board::default();

        board.cells[which[0]].marked = true;
        board.cells[which[1]].marked = true;
        board.cells[which[2]].marked = true;
        board.cells[which[3]].marked = true;
        board.cells[which[4]].marked = true;

        assert!(board.has_winning_row());
    }

    #[yare::parameterized(
        first_col = {  [0, 5, 10, 15, 20] },
        second_col = { [1, 6, 11, 16, 21] },
        third_col = { [2, 7, 12, 17, 22] },
        fourth_col = { [3, 8, 13, 18, 23] },
        fifth_col = { [4, 9, 14, 19, 24] },
    )]
    fn winning_column(which: [usize; 5]) {
        let mut board = Board::default();

        board.cells[which[0]].marked = true;
        board.cells[which[1]].marked = true;
        board.cells[which[2]].marked = true;
        board.cells[which[3]].marked = true;
        board.cells[which[4]].marked = true;

        assert!(board.has_winning_column());
    }

    #[test]
    fn is_not_winning_row() {
        let mut board = Board::default();
        board.cells[0].marked = true;
        let no = Board::is_winning_row(&board.cells, 0);

        assert!(!no)
    }

    #[test]
    fn is_winning_row() {
        let mut board = Board::default();
        board.cells[0].marked = true;
        board.cells[1].marked = true;
        board.cells[2].marked = true;
        board.cells[3].marked = true;
        board.cells[4].marked = true;
        let yes = Board::is_winning_row(&board.cells, 0);

        println!("{}", board);

        assert!(yes)
    }

    #[test]
    fn example_part1() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let (instructions, mut boards) = parse(input).unwrap();

        let score = part1(&instructions.0, &mut boards).unwrap();

        assert_eq!(score, 4512);
    }
}
