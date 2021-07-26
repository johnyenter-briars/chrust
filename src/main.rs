use std::borrow::BorrowMut;
use std::io;
use std::{borrow::Borrow, fmt::Result};

#[derive(Debug, Clone, Copy)]
enum ChessPiece {
    WhitePawn,
    WhiteRook,
    WhiteKing,

    BlackPawn,
    BlackRook,
    BlackKing
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
        let x: &Vec<Cell> = self.squares[0].borrow();
        println!("{:?}", x);

        for i in 0..self.board_size {
            for j in 0..self.board_size {
                // let idx:usize = usize::try_from(i);
                let mut x: Cell = self.squares[i as usize][j as usize];
                // let x: & Cell = self.squares[i as usize][j as usize].borrow();
                
                // let y = match x.space.borrow_mut() {
                //     Some(cp) => {cp}
                //     None => {panic!("idk really what to do rn")}
                // };

                if let Some(mut y) = x.space.as_mut() {
                    y = ChessPiece::BlackKing
                }

                // let z: &mut Option<ChessPiece> = x.space.borrow_mut();
                // *z = Option::from(ChessPiece::BlackKing);

            }
        }

        println!("==================================");
        println!("{:?}", x);

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let mut x = Board::new(8);
    x.set_board();
}
