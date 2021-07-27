use std::borrow::BorrowMut;
use std::{io};


#[derive(Debug, Clone, Copy)]
enum ChessPiece {
    WhiteKing
}

#[derive(Debug, Clone, Copy)]
struct Cell{
    space: Option<ChessPiece>
}
#[derive(Debug)]
struct Board {
    // squares: [[Cell;8 ]; 8]
    squares: Vec<Vec<Cell>>,
    board_size: i32
}

impl Board {
    fn new(board_size: i32) -> Board {
        let mut squares: Vec<Vec<Cell>> = Vec::new();

        for _ in 0..board_size {
            let mut sub_squares: Vec<Cell> = Vec::new();
            for _ in 0..board_size {
                sub_squares.push(Cell{space: Option::None});
            }

            squares.push(sub_squares);
        }

        Board{squares, board_size}
    }

    fn set_board(&mut self)  -> io::Result<()>{
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                let cell: &mut Cell = self.squares[i as usize][j as usize].borrow_mut();

                cell.space = Option::from(ChessPiece::WhiteKing);
            }
        }

        Ok(())
    }
}

fn main() {
    let mut board = Board::new(2);
    let _ = board.set_board();
}
