mod data;

use std::fmt::{Display, write, format};

use crate::data::data::INPUT;

#[derive(Copy, Clone)]
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
            false => '⭕'
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

struct Board {
    matrix: [[Cell; 5]; 5]
}

impl Board {
    fn new() -> Board {
        Board {
            matrix: [[Cell::new(0); 5]; 5],
        }
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
    let random_numbers: Vec<&str> = random_numbers.split(',').collect();

    let boards: Vec<Board> = input.map(|board_str| {
        let rows = board_str.split('\n').into_iter();
        rows.map(|row| {
            let numbers = row.split_whitespace();
            let array = numbers
                .take(5)
                .map(|number| Cell::new(number.parse().unwrap()))
                .collect::<[Cell; 5]>();
                array
        })
        .enumerate()
        .fold(
            Board::new(),
            |mut board: Board, (index, row)| {
                board.matrix[index] = row;
                board
            },
        )
    })
    .collect();

    for b in boards {
        println!("{}", b);
    }
}
