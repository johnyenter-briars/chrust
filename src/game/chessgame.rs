use crate::board::cell::chesspiece::ChessPiece;
use crate::board::chessboard::Board;
use crate::board::coordinate::Coordinate;
use crate::chessmove::piecemove::Move;
use crate::chessmove::piecemove::PieceMove;
use crate::player::aiplayer::*;
use crate::player::humanplayer::{self, *};

use crate::chessmove::action::{self, Action, HumanAction};

use std::borrow::Borrow;
use std::error::Error;

pub struct ChessGame<'a> {
    pub human_player: HumanPlayer,
    pub ai_player: AIPlayer,
    pub board: Board,
    pub history: Vec<&'a dyn Action<'a>>,
}

impl<'a> ChessGame<'a> {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame {
            human_player,
            ai_player,
            board,
            history: Vec::new(),
        }
    }

    pub fn start_game(&mut self) -> Result<&str, Box<dyn Error>> {
        self.board.print_to_screen();

        let mut turn_num = 1;

        loop {
            //Human moves
            println!("Select your piece!");

            println!("x:");
            let mut line = String::new();
            let size = std::io::stdin().read_line(&mut line).unwrap();
            let x: char = line.trim().parse()?;

            println!("y:");
            let mut line = String::new();
            let size = std::io::stdin().read_line(&mut line).unwrap();
            let y: i32 = line.trim().parse()?;

            println!("Your choice: x: {} y: {}", x, y);
            let current_position = Coordinate::new(x, y);

            let coord_choices = self.board.get_possible_moves(current_position, turn_num);

            println!("{:?}", coord_choices);
            for (index, c) in coord_choices.iter().enumerate() {
                println!("choice: {}, coord: {:?}", index, c);
            }

            //take in choice
            let mut line = String::new();
            let size = std::io::stdin().read_line(&mut line).unwrap();
            let choice: i32 = line.trim().parse()?;

            let coord_choice = coord_choices[choice as usize];

            self.board.move_piece(Coordinate { x, y }, coord_choice);

            self.board.print_to_screen();

            //AI moves

            turn_num += 1;
        }

        Ok(&self.human_player.name)
    }
}
