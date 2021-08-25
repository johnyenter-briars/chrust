use crate::ai::minimax::boardstate::BoardState;
use crate::ai::minimax::funcs::max_decision;
use crate::board;
use crate::board::cell::chesspiece::ChessPiece;
use crate::board::chessboard::Board;
use crate::board::coordinate::Coordinate;
// use crate::chessmove::piecemove::Move;
use crate::chessmove::piecemove::PieceMove;
use crate::player::aiplayer::*;
use crate::player::humanplayer::{self, *};

use crate::chessmove::action::{self, Action, HumanAction};

extern crate rand;

// use rand::thread_rng;
use rand::prelude::*;
use rand::seq::SliceRandom;

use std::borrow::Borrow;
use std::error::Error;
use std::{result, thread};
use std::str::FromStr;
use std::thread::current;
use std::time::Duration;

pub struct ChessGame<'a> {
    pub human_player: HumanPlayer,
    pub ai_player: AIPlayer,
    pub board: Board,
    pub history: Vec<&'a dyn Action<'a>>,
    human_plays: bool,
    tick_speed: u64, //milli
    // pub decision_maker: DecisionMaker,
}

impl<'a> ChessGame<'a> {
    pub fn new(
        human_player: HumanPlayer,
        ai_player: AIPlayer,
        board: Board,
        human_plays: bool,
        tick_speed: u64,
    ) -> Self {
        ChessGame {
            human_player,
            ai_player,
            board,
            history: Vec::new(),
            human_plays,
            tick_speed,
            // DecisionMaker{}
        }
    }

    pub fn start_game(&mut self) -> Result<&str, Box<dyn Error>> {
        self.board.print_to_screen("Initial".to_string());

        let mut turn_num = 1;

        loop {
            //Human moves
            if !self.human_moves(turn_num)? {
                continue;
            }

            self.board
                .print_to_screen(format!("after human turn {}", turn_num));

            thread::sleep(Duration::from_millis(self.tick_speed));

            //AI moves
            if !self.ai_moves(turn_num)? {
                continue;
            }

            self.board
                .print_to_screen(format!("after ai turn {}", turn_num));

            turn_num += 1;

            thread::sleep(Duration::from_millis(self.tick_speed));

            // if turn_num == 10 {
            //     break;
            // }
        }

        Ok(&self.human_player.name)
    }

    pub fn ai_moves(&mut self, turn_num: i32) -> Result<bool, Box<dyn Error>> {
        println!("ai moving");

        let board_state = BoardState {
            board: self.board.clone(),
        };

        let ai_move = max_decision(&board_state, self.ai_player.color, 2);

        if ai_move.from.x == ai_move.to.x && ai_move.from.y == ai_move.to.y {
            let idk = "im sad";
        }
        self.board.move_piece(ai_move.from, ai_move.to);

        Ok(true)
    }

    pub fn human_moves(&mut self, turn_num: i32) -> Result<bool, Box<dyn Error>> {
        println!("Human moving!");

        let (from, to) = if self.human_plays {
            println!("Select your piece!");

            println!("x:");
            let x: char = get_input::<char>()?;

            println!("y:");
            let y: i32 = get_input::<i32>()?;

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

            println!("Options: ");
            for (index, c) in coord_choices.iter().enumerate() {
                println!("choice: {}, coord: {:?}", index, c);
            }

            let choice: i32 = get_input::<i32>()?;

            let coord_choice = coord_choices[choice as usize];
            println!("Choice: {:?}", coord_choice);

            (current_position, coord_choice)
        } else {
            let cells = self
                .board
                .get_cells_with_pieces_with_color(self.human_player.color);

            let (from, to) = loop {
                let mut random_white_square = cells.choose(&mut rand::thread_rng());

                let cell = random_white_square
                    .ok_or("There was an error while trying to get the human's player's squares")?;

                let from = Coordinate::new(cell.x, cell.y);

                let coord_choices =
                    self.board
                        .get_possible_moves_human(from, turn_num, &self.human_player)?;

                if coord_choices.len() < 1 {
                    //that piece can't go anywhere - try to get another one
                    continue;
                }

                let to = *coord_choices.choose(&mut rand::thread_rng()).ok_or("There was an error while trying to get a choice randomly")?;

                break (from.clone(), to.clone());
            };

            (from, to)
        };

        self.board.move_piece(from, to);

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
