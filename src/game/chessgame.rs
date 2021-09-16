use crate::ai::minimax::boardstate::BoardState;
use crate::ai::minimax::funcs::max_decision;
use crate::board;
use crate::board::cell::chesspiece::ChessPiece;
use crate::board::cell::color::Color;
use crate::board::cell::piecetype::PieceType;
use crate::board::cell::Cell;
use crate::board::chessboard::Board;
use crate::board::coordinate::Coordinate;
// use crate::chessmove::piecemove::Move;
use crate::chessmove::piecemove::PieceMove;
use crate::player::aiplayer::*;
use crate::player::chessplayer::ChessPlayer;
use crate::player::humanplayer::{self, *};

use crate::chessmove::action::{self, Action, HumanAction};

extern crate rand;

// use rand::thread_rng;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rocket::request::FromRequest;

use std::borrow::Borrow;
use std::error::Error;
use std::str::FromStr;
use std::thread::current;
use std::time::Duration;
use std::{result, thread};

use crate::ext::stringext::ToCoord;

pub struct ChessGame {
    pub human_player: HumanPlayer,
    pub ai_player: AIPlayer,
    pub board: Board,
    // pub history: Vec<&'a (dyn Action<'a> + Sync)>,
    human_plays: bool,
    tick_speed: u64, //milli

    side_to_move: char,
    castling_ability: String,
    en_passant_target_square: String,
    halfmove_clock: u32,
    fullmove_counter: u32,
}

impl ChessGame {
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
            // history: Vec::new(),
            human_plays,
            tick_speed,
            side_to_move: 'w',
            castling_ability: "KQkq".to_string(),
            en_passant_target_square: "h3".to_string(),
            halfmove_clock: 0,
            fullmove_counter: 1,
            // DecisionMaker{}
        }
    }

    pub fn check_for_winner(&self) -> Option<&str> {
        let white_king = self.board.piece_specific(Color::White, PieceType::King);
        let black_king = self.board.piece_specific(Color::Black, PieceType::King);

        match (white_king.is_some(), black_king.is_some()) {
            (true, true) => None,
            (true, false) => Some(self.user_of_color(Color::White).name()),
            (false, true) => Some(self.user_of_color(Color::Black).name()),
            (false, false) => panic!("Something went horribly wrong. Both kings are gone?"),
        }
    }

    pub fn start_game(&mut self) -> Result<&str, Box<dyn Error>> {
        self.print_to_screen("Initial".to_string());

        let mut turn_num: u32 = 1;

        loop {
            //Human moves
            if !self.human_moves(turn_num)? {
                continue;
            }

            self.side_to_move = self.ai_player.color_abbr();

            if self.human_plays {
                self.print_to_screen(format!("after ai turn {}", turn_num));
            } else {
                thread::sleep(Duration::from_millis(self.tick_speed));
            }

            //AI moves
            if !self.ai_moves(turn_num)? {
                continue;
            }

            self.side_to_move = self.human_player.color_abbr();

            self.print_to_screen(format!("after ai turn {}", turn_num));

            turn_num += 1;

            if let Some(_) = self.check_for_winner() {
                //would love to return the value itself here - but the borrow checker complains
                //the wize wizards on the rust discord say its a bug in the borrow checker - so maybe it gets fixed later
                break;
            }
        }

        Ok(self.check_for_winner().unwrap())
    }

    pub fn ai_moves(&mut self, turn_num: u32) -> Result<bool, Box<dyn Error>> {
        println!("ai moving");

        let board_state = BoardState {
            board: self.board.clone(),
        };

        let ai_move = max_decision(&board_state, self.ai_player.color, 2)?;

        if ai_move.from.x == ai_move.to.x && ai_move.from.y == ai_move.to.y {
            let idk = "im sad";
        }
        self.board.move_piece(ai_move.from, ai_move.to);

        self.fullmove_counter += 1;
        self.halfmove_clock += 1;

        Ok(true)
    }

    pub fn human_moves(&mut self, turn_num: u32) -> Result<bool, Box<dyn Error>> {
        println!("Human moving!");

        let (from, to) = if self.human_plays {
            println!("Select your piece!");

            println!("x:");
            let x: char = get_input::<char>()?;

            println!("y:");
            let y: i32 = get_input::<i32>()?;

            let current_position = Coordinate::new(x, y);

            //this is syntax is really cool
            let coord_choices = match self.board.possible_moves_human(
                current_position,
                turn_num as i32,
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
                .cells_with_pieces_with_color(self.human_player.color);

            let (from, to) = loop {
                let mut random_white_square = cells.choose(&mut rand::thread_rng());

                let cell = random_white_square
                    .ok_or("There was an error while trying to get the human's player's squares")?;

                let from = Coordinate::new(cell.x, cell.y);

                let coord_choices =
                    self.board
                        .possible_moves_human(from, turn_num as i32, &self.human_player)?;

                if coord_choices.len() < 1 {
                    //that piece can't go anywhere - try to get another one
                    continue;
                }

                let to = coord_choices
                    .choose(&mut rand::thread_rng())
                    .ok_or("There was an error while trying to get a choice randomly")?;

                break (from.clone(), to.clone());
            };

            (from, to)
        };

        self.board.move_piece(from, to);

        Ok(true)
    }

    fn user_of_color(&self, color: Color) -> Box<&dyn ChessPlayer> {
        match (
            self.human_player.color == color,
            self.ai_player.color == color,
        ) {
            (true, true) => panic!("Players with the same color?"),
            (true, false) => Box::new(&self.human_player),
            (false, true) => Box::new(&self.ai_player),
            (false, false) => panic!("Players with the same color?"),
        }
    }

    pub fn process_fen(&mut self, fen: String) -> String {
        let board_section = fen.split(' ').next().expect("Fen is improperly formatted!");

        //old board object is hopefully destructed from memory right?
        self.board =
            Board::load_from_fen(board_section.to_string()).expect("Unable to load board from ");

        self.ai_moves(self.fullmove_counter);

        self.board.print_to_screen("test".to_string());

        self.fen()
    }

    pub fn is_valid(
        &self,
        current_fen: String,
        current_location: String,
        possible_location: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let board_section = current_fen
            .split(' ')
            .next()
            .expect("Fen is improperly formatted!");

        let board =
            Board::load_from_fen(board_section.to_string()).expect("Unable to load board from ");

        let current_position = current_location.to_coord();

        let possible_moves = board.possible_moves(current_position, 1, Color::White)?;

        let possible_position = possible_location.to_coord();

        let valid_move = possible_moves
            .iter()
            .any(|coord| coord.x == possible_position.x && coord.y == possible_position.y);

        Ok(valid_move)
    }

    pub fn valid_moves(
        &self,
        fen: String,
        location: String,
    ) -> Result<Vec<Coordinate>, Box<dyn std::error::Error>> {
        let board_section = fen.split(' ').next().expect("Fen is improperly formatted!");
        
        let board =
            Board::load_from_fen(board_section.to_string()).expect("Unable to load board from ");

        if location.as_str().len() != 2 {
            return Err(Box::from("Invalid format for location string"));
        }

        let current_position = location.to_coord();

        if board.piece(current_position.x, current_position.y)?.color == Color::Black {
            return Ok(vec![]);
        }

        let coords = board
            .possible_moves(current_position, 1, Color::White)
            .expect("Unable to find the valid moves!");

        Ok(coords)
    }

    //for an explanation of this crazy algo check out: https://www.chessprogramming.org/Forsyth-Edwards_Notation
    fn fen(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.board.board_fen_section(), //board formatted
            self.side_to_move,
            self.castling_ability,
            self.en_passant_target_square,
            self.halfmove_clock,
            self.fullmove_counter
        )
    }

    fn print_to_screen(&mut self, configuration_name: String) {
        self.board.print_to_screen(configuration_name);
        println!("{}", self.fen());
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
