use crate::board;
use crate::board::cell::chesspiece::ChessPiece;
use crate::board::chessboard::Board;
use crate::board::coordinate::Coordinate;
// use crate::chessmove::piecemove::Move;
use crate::chessmove::piecemove::PieceMove;
use crate::player::aiplayer::*;
use crate::player::humanplayer::{self, *};

use crate::ai::decisionmaker::DecisionMaker;

use crate::chessmove::action::{self, Action, HumanAction};

extern crate rand;

// use rand::thread_rng;
use rand::prelude::*;
use rand::seq::SliceRandom;

use std::borrow::Borrow;
use std::error::Error;
use std::str::FromStr;
use std::thread::current;

pub struct ChessGame<'a> {
    pub human_player: HumanPlayer,
    pub ai_player: AIPlayer,
    pub board: Board,
    pub history: Vec<&'a dyn Action<'a>>,
    // pub decision_maker: DecisionMaker,
}

impl<'a> ChessGame<'a> {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame {
            human_player,
            ai_player,
            board,
            history: Vec::new(),
            // DecisionMaker{}
        }
    }

    pub fn start_game(&mut self) -> Result<&str, Box<dyn Error>> {
        self.board.print_to_screen();

        let mut turn_num = 1;

        loop {
            //Human moves
            if !self.human_moves(turn_num)? {continue;}

            self.board.print_to_screen();

            //AI moves
            if !self.ai_moves(turn_num)? {
                continue;
            }

            self.board.print_to_screen();

            turn_num += 1;

            if turn_num == 10 {
                break;
            }
        }

        Ok(&self.human_player.name)
    }

    pub fn ai_moves(&mut self, turn_num: i32) -> Result<bool, Box<dyn Error>> {
        let possible_cells = self
            .board
            .get_cells_with_pieces_with_color(self.ai_player.color);

        loop {


            // let choice = possible_cells.choose(&mut rand::thread_rng()).unwrap();

            // let current_position = Coordinate::new(choice.x, choice.y);

            // let coord_choices =
            //     self.board
            //         .get_possible_moves_ai(current_position, turn_num, &self.ai_player)?;

            // if coord_choices.len() < 1 {
            //     continue;
            // }

            // let coord_to_move_to = coord_choices.choose(&mut rand::thread_rng()).unwrap();

            // self.board.move_piece(current_position, coord_to_move_to.clone());

            break;
        }

        Ok(true)
    }

    pub fn human_moves(&mut self, turn_num: i32) -> Result<bool, Box<dyn Error>> {
        println!("Select your piece!");

        println!("x:");
        let x: char = get_input::<char>()?;

        println!("y:");
        let y: i32 = get_input::<i32>()?;

        println!("Your choice: x: {} y: {}", x, y);
        let current_position = Coordinate::new(x, y);

        //this is syntax is really cool
        let coord_choices = match self.board.get_possible_moves_human(
            current_position,
            turn_num,
            &self.human_player,
        ) {
            Ok(choices) => {
                if choices.len() < 1 {
                    println!("That piece can't go anywhere!");
                    return Ok(false);
                }
                choices
            }
            Err(reason) => {
                println!("Not valid move : {:?}", reason);
                return Ok(false);
            }
        };

        for (index, c) in coord_choices.iter().enumerate() {
            println!("choice: {}, coord: {:?}", index, c);
        }

        //take in choice
        let choice: i32 = get_input::<i32>()?;

        let coord_choice = coord_choices[choice as usize];

        self.board.move_piece(Coordinate { x, y }, coord_choice);

        Ok(true)
    }
}

//this is a mess and i dont fully understand it - i just did what the compiler told me to
fn get_input<I: std::str::FromStr>() -> Result<I, Box<dyn Error>>
where
    <I as FromStr>::Err: 'static + Error,
{
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let y: I = line.trim().parse()?;
    Ok(y)
}
