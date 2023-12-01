use crate::common::Part;

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
#[derive(Clone)]
struct Board {
    size: usize,
    board: Vec<i32>,
    called: Vec<bool>,
    last_called: i32,
}

impl Board {
    fn new(board: Vec<i32>) -> Board {
        let size = 5usize;
        assert_eq!(board.len(), size*size);
        Board {
            size,
            called: vec![false; board.len()],
            board,
            last_called: -1,
        }
    }

    fn play(&mut self, number: i32) {
        self.last_called = number;
        for i in 0..self.board.len() {
            if self.board[i] == number {
                self.called[i] = true;
            }
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..self.size {
            if self.check_row(i) { return true; }
            if self.check_col(i) { return true; }
        }
        false
    }

    fn check_row(&self, i: usize) -> bool {
        (i*self.size..(i+1)*self.size).all(|i| self.called[i])
    }

    fn check_col(&self, i: usize) -> bool {
        (i..self.board.len()).step_by(self.size).all(|i| self.called[i])
    }

    fn score(&self) -> i32 {
        let sum : i32 = (0..self.board.len()).filter(|&i| !self.called[i]).map(|i| self.board[i]).sum();
        sum*self.last_called
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers : Vec<i32> = s.split_whitespace().map(|s| s.parse()).collect::<Result<Vec<i32>, Self::Err>>()?;
        Ok(Board::new(numbers))
    }
}

fn win_first(mut boards: Vec<Board>, numbers: impl IntoIterator<Item=i32>) -> Board {
    for n in numbers {
        for board in &mut boards {
            board.play(n);
            if board.has_won() {
                return board.clone();
            }
        }
    }
    panic!("Out of numbers and noone has won.")
}

fn win_last(mut boards: Vec<Board>, numbers: impl IntoIterator<Item=i32>) -> Board {
    let mut num_won = 0;
    let all_boards =  boards.len();
    for n in numbers {
        for board in &mut boards {
            if !board.has_won() {
                board.play(n);
                if board.has_won() {
                    num_won += 1;
                    if num_won == all_boards {
                        return board.clone();
                    }
                }
            }
        }
    }
    panic!("Out of numbers and not all boards won.")
}

pub fn solve(data : &Vec<String>, part : Part) {
    let numbers = data[0].split(',').map(|n| n.parse().expect("Int expected"));

    let boards : Vec<Board> = data[1..].join("\n").split("\n\n").map(|s| s.parse().expect("Board expected")).collect();

    match part {
        Part::First => {
            let winner = win_first(boards, numbers);
            println!("{}", winner.score());
        }
        Part::Second => {
            let winner = win_last(boards, numbers);
            println!("{}", winner.score());
        }
    }
}
