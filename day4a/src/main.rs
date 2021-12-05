use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    str::FromStr,
};

fn main() {
    let raw_input = fs::read_to_string("day4a.txt").unwrap();
    let drawn_numbers = DrawnNumbers::from_str(&raw_input).unwrap();
    let mut boards = Boards::from_str(&raw_input).unwrap();

    let mut last_bingo = None;
    let mut winners = Winners::new();
    for nb in drawn_numbers.0 {
        for (board_idx, board) in boards.inner.iter_mut().enumerate() {
            let bingo = board.scan(nb);
            if let Some(rep) = bingo {
                if winners.last(board_idx).is_some() {
                    last_bingo = Some(rep);
                }
            }
        }
    }

    println!("Answer: {}", last_bingo.unwrap());
}

#[derive(Debug)]
struct DrawnNumbers(Vec<i32>);

impl FromStr for DrawnNumbers {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let drawn_numbers = input.lines().next().unwrap();
        let drawn_numbers: Vec<&str> = drawn_numbers.split(',').collect();
        let drawn_numbers: Vec<i32> = drawn_numbers
            .iter()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();
        Ok(Self(drawn_numbers))
    }
}

#[derive(Debug, Clone)]
enum Mark {
    Marked(i32),
    Unmarked(i32),
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Mark>>,
    row_marks: HashMap<usize, Vec<i32>>,
    col_marks: HashMap<usize, Vec<i32>>,
}

impl Board {
    fn new(board: Vec<Vec<i32>>) -> Self {
        let board = board
            .iter()
            .map(|row| row.iter().map(|&nb| Mark::Unmarked(nb)).collect())
            .collect();

        Self {
            board,
            row_marks: HashMap::default(),
            col_marks: HashMap::default(),
        }
    }

    fn scan(&mut self, drawn_nb: i32) -> Option<i32> {
        for (r, row) in self.board.iter_mut().enumerate() {
            let col_idx = row.iter().position(|e| match e {
                Mark::Marked(nb) => nb == &drawn_nb,
                Mark::Unmarked(nb) => nb == &drawn_nb,
            });

            if let Some(c) = col_idx {
                // Marked the item
                row[c] = Mark::Marked(drawn_nb);

                // add row index to row mark list
                let v = self.row_marks.entry(r).or_insert_with(Vec::new);
                v.push(drawn_nb);
                if v.len() == 5 {
                    let mut unmarkeds = 0;
                    for row in &self.board {
                        for col in row {
                            match &col {
                                Mark::Unmarked(c) => unmarkeds += c,
                                Mark::Marked(_) => (),
                            }
                        }
                    }
                    return Some(unmarkeds * drawn_nb);
                }

                // add col index to col mark list
                let v = self.col_marks.entry(c).or_insert_with(Vec::new);
                v.push(drawn_nb);
                if v.len() == 5 {
                    let mut unmarkeds = 0;
                    for row in &self.board {
                        for col in row {
                            match &col {
                                Mark::Unmarked(c) => unmarkeds += c,
                                Mark::Marked(_) => (),
                            }
                        }
                    }
                    return Some(unmarkeds * drawn_nb);
                }
            }
        }
        None
    }
}

struct Winners(HashSet<usize>);

impl Winners {
    fn new() -> Self {
        Self(HashSet::default())
    }
    fn last(&mut self, idx: usize) -> Option<usize> {
        let prev_len = self.0.len();
        self.0.insert(idx);
        if self.0.len() != prev_len {
            Some(idx)
        } else {
            None
        }
    }
}

struct Boards {
    inner: Vec<Board>,
}

impl FromStr for Boards {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let boards_input = input.lines().skip(2);
        let mut boards = Vec::new();
        let mut raw_board = Vec::new();

        for line in boards_input {
            if !line.is_empty() {
                let row: Vec<i32> = line
                    .split_terminator(' ')
                    .filter_map(|nb| nb.parse::<i32>().ok())
                    .collect();
                raw_board.push(row);
            } else {
                let board = Board::new(raw_board.clone());
                boards.push(board);
                raw_board.clear();
            }
        }

        Ok(Self { inner: boards })
    }
}
