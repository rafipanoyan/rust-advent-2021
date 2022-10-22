mod data;

use std::fmt::Display;

use crate::data::data::INPUT;

#[derive(Copy, Clone, Debug)]
struct Cell {
    value: i32,
    marked: bool,
}

impl Cell {
    fn new(value: i32) -> Cell {
        Cell {
            value,
            marked: false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = match self.marked {
            true => '✅',
            false => '⭕',
        };
        write!(f, "{:02}:{} ", self.value, mark)
    }
}

impl FromIterator<Cell> for [Cell; 5] {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut row = [Cell::new(0); 5];

        iter.into_iter().take(5).enumerate().for_each(|(i, cell)| {
            row[i] = cell;
        });
        row
    }
}

#[derive(Copy, Clone)]
struct Board {
    matrix: [[Cell; 5]; 5],
    won: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            matrix: [[Cell::new(0); 5]; 5],
            won: false,
        }
    }

    fn try_mark(&mut self, number: &i32) {
        let mut dirty = false;
        self.matrix.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if cell.value == *number {
                    cell.marked = true;
                    dirty = true;
                }
            })
        });

        if dirty {
            print_board(self);
        }
    }

    fn check_win(&mut self) -> Option<Board> {
        let mut row_win: bool;
        let mut column_win: bool;
        for i in 0..5 {
            row_win = (0..5)
                .into_iter()
                .fold(true, |acc, j| acc && self.matrix[i][j].marked);

            column_win = (0..5)
                .into_iter()
                .fold(true, |acc, j| acc && self.matrix[j][i].marked);

            if column_win || row_win {
                self.won = true;
                return Some(*self);
            }
        }

        None
    }

    fn sum_unmarked(&self) -> i32 {
        self.matrix.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, cell| match cell.marked {
                true => acc,
                false => acc + cell.value,
            })
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        for row in self.matrix {
            for cell in row {
                let cell_str = format!("{cell}");
                display.push_str(&cell_str);
            }
            display.push('\n')
        }
        write!(f, "{}", display)
    }
}

fn main() {
    let mut input = INPUT.split("\n\n").into_iter();
    let random_numbers: &str = input.next().unwrap();
    let random_numbers = parse_numbers_str(random_numbers);

    let mut boards: Vec<Board> = input.map(parse_board_str).collect();

    let mut last_win: Option<(Board, i32)> = None;
    'loop_num: for num in &random_numbers {
        println!("Random number : {}", num);
        for board in &mut boards {
            if !board.won {
                board.try_mark(num);
                match board.check_win() {
                    Some(board) => {
                        println!("We have a winner");
                        let sum_unmarked = board.sum_unmarked();
                        println!(
                            "Sum of unmarked : {} | final score : {}",
                            sum_unmarked,
                            num * sum_unmarked
                        );

                        last_win = Some((board, *num));
                        // break 'loop_num;
                    }
                    None => {}
                }
            }
        }
    }

    match last_win {
        Some((b, num)) => {
            let last_board_sum_unmarked = b.sum_unmarked();
            println!(
                "Last Board\nSum of unmarked : {} | final score : {}",
                last_board_sum_unmarked,
                num * last_board_sum_unmarked
            );
        }
        None => {}
    }
}

fn parse_numbers_str(numbers: &str) -> Vec<i32> {
    numbers.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_row_str(row: &str) -> [Cell; 5] {
    row.split_whitespace()
        .take(5)
        .map(|number| Cell::new(number.parse().unwrap()))
        .collect::<[Cell; 5]>()
}

fn parse_board_str(boards: &str) -> Board {
    boards
        .split('\n')
        .into_iter()
        .map(parse_row_str)
        .enumerate()
        .fold(Board::new(), |mut board: Board, (index, row)| {
            board.matrix[index] = row;
            board
        })
}

fn print_board(board: &Board) {
    println!("{}", board);
}
