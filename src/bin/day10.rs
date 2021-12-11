use std::fmt::Debug;

fn main() -> anyhow::Result<()> {
    let contents = include_str!("../../inputs/day10.txt");
    let part1 = part1(contents);

    println!("(day 10) part 1: {}", part1);

    let part2 = part2(contents);
    println!("(day 10) part 2: {}", part2);

    Ok(())
}

fn part2(input: &str) -> usize {
    let mut scores = input
        .trim()
        .lines()
        .map(|line| {
            let score = IncompleteSyntaxErrorGame::default().play::<Vec<Token>, _>(line.chars());
            score
        })
        .filter(|&s| s != 0)
        .collect::<Vec<_>>();

    scores.sort_unstable();

    *scores.get(scores.len() / 2).unwrap()
}

#[derive(Copy, Clone)]
enum Status {
    Good, // default
    Incomplete,
    Corrupt(Token),
}

impl Default for Status {
    fn default() -> Self {
        Self::Good
    }
}

trait SyntaxErrorGame {
    fn step<S>(&self, stack: &mut S, next: char) -> Status
    where
        S: Stack<Token> + Default,
    {
        match next {
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(token) if token != next.into() => Status::Corrupt(next.into()),
                Some(_) => Status::Good,
                None => Status::Incomplete,
            },
            open_char => {
                stack.push(open_char);
                Status::Good
            }
        }
    }
}

#[derive(Debug, Default)]
struct IncompleteSyntaxErrorGame;

impl SyntaxErrorGame for IncompleteSyntaxErrorGame {}

impl IncompleteSyntaxErrorGame {
    fn play<S, I>(&self, characters: I) -> usize
    where
        S: Stack<Token> + Default,
        I: Iterator<Item = char>,
    {
        let incomplete = characters.fold(Some(S::default()), |stack, next| match stack {
            Some(mut s) => match self.step(&mut s, next) {
                Status::Corrupt(_) => None,
                _ => Some(s),
            },
            None => None,
        });

        incomplete
            .map(|stack| {
                stack.to_vec().iter().rfold(0_usize, |score, token| {
                    score * 5 + token.incomplete_points()
                })
            })
            .unwrap_or(0)
    }
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| CorruptSyntaxErrorGame::default().play::<Vec<Token>, _>(line.chars()))
        .sum()
}

#[derive(Debug, Default)]
struct CorruptSyntaxErrorGame;

impl SyntaxErrorGame for CorruptSyntaxErrorGame {}

impl CorruptSyntaxErrorGame {
    fn play<S, I>(&self, characters: I) -> usize
    where
        S: Stack<Token> + Default,
        I: Iterator<Item = char>,
    {
        characters
            .scan(S::default(), |stack, next| {
                Some(match next {
                    ')' | ']' | '}' | '>' => self.points(stack.pop(), next),
                    open_char => {
                        stack.push(open_char);
                        0
                    }
                })
            })
            .sum()
    }

    // Pop returns error points for faulty unmatched tokens, 😀😀😀
    fn points<E>(&self, popped: Option<Token>, expected: E) -> usize
    where
        E: Into<Token> + PartialEq,
    {
        let expected = expected.into();
        match popped {
            Some(token) if token != expected => expected.corrupt_points(),
            _ => 0,
        }
    }
}

trait Stack<T: Debug>: IntoIterator<Item = T> + Debug {
    fn push(&mut self, element: impl Into<T>);
    fn pop(&mut self) -> Option<T>;
    fn to_vec(self) -> Vec<T>;
}

impl<T: Debug> Stack<T> for Vec<T> {
    fn push(&mut self, element: impl Into<T>) {
        Vec::push(self, element.into());
    }

    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }

    fn to_vec(self) -> Vec<T> {
        self
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Token {
    Paren,
    SquareBracket,
    CurlyBrace,
    TriangleBracket,
}

impl Token {
    fn corrupt_points(&self) -> usize {
        match self {
            Self::Paren => 3,
            Self::SquareBracket => 57,
            Self::CurlyBrace => 1197,
            Self::TriangleBracket => 25137,
        }
    }

    fn incomplete_points(&self) -> usize {
        match self {
            Self::Paren => 1,
            Self::SquareBracket => 2,
            Self::CurlyBrace => 3,
            Self::TriangleBracket => 4,
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' | ')' => Self::Paren,
            '[' | ']' => Self::SquareBracket,
            '{' | '}' => Self::CurlyBrace,
            '<' | '>' => Self::TriangleBracket,
            t => unreachable!(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/example/day10.txt");

        assert_eq!(part1(input), 26397);
    }

    #[test]
    fn part1_solution() {
        let input = include_str!("../../inputs/day10.txt");

        assert_eq!(part1(input), 392421);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/example/day10.txt");

        assert_eq!(part2(input), 288957);
    }

    #[yare::parameterized(
        t1 = { "[({(<(())[]>[[{[]{<()<>>", 288957  },
        t2 = { "[(()[<>])]({[<{<<[]>>(", 5566 },
        t3 = { "(((({<>}<{<{<>}{[]{[]{}", 1480781 },
        t4 = { "{<[[]]>}<{[{[{[]{()[[[]", 995444  },
        t5 = { "<{([{{}}[<[[[<>{}]]]>[]]", 294  }
    )]
    fn part2_lines(input: &str, score: usize) {
        assert_eq!(part2(input), score);
    }

    #[test]
    fn part2_solution() {
        let input = include_str!("../../inputs/day10.txt");

        assert_eq!(part2(input), 2769449099);
    }

    // #[test]
    // fn part2_solution() {
    //     let input = include_str!("../../inputs/day10.txt");
    //
    //     assert_eq!(part2(input), 392421);
    // }
}
