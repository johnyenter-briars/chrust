use crate::ai::minimax::funcs::max_decision;
use crate::board::chessboard::ChessBoard;
use crate::board::color::Color;
use crate::board::coordinate::Coordinate;
use crate::board::types::PieceType;
use crate::chessmove::piecemove::PieceMove;
use crate::player::aiplayer::*;
use crate::player::chessplayer::ChessPlayer;
use crate::player::humanplayer::*;
use crate::ChrustResult;

extern crate rand;

use itertools::Itertools;
use rand::seq::SliceRandom;

use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::ext::stringext::ToCoord;

pub struct ChessGame {
    pub human_player: HumanPlayer,
    pub ai_player: AIPlayer,
    pub board: ChessBoard,
    human_plays: bool,
    tick_speed: u64, //milli
}

impl ChessGame {
    pub fn new(
        human_player: HumanPlayer,
        ai_player: AIPlayer,
        board: ChessBoard,
        human_plays: bool,
        tick_speed: u64,
    ) -> Self {
        ChessGame {
            human_player,
            ai_player,
            board,
            human_plays,
            tick_speed,
        }
    }

    pub fn check_for_winner(&self) -> Option<String> {
        let white_king = self.board.piece_specific(Color::White, PieceType::King);
        let black_king = self.board.piece_specific(Color::Black, PieceType::King);

        match (white_king.is_some(), black_king.is_some()) {
            (true, true) => None,
            (true, false) => Some(self.user_of_color(Color::White).name().to_string()),
            (false, true) => Some(self.user_of_color(Color::Black).name().to_string()),
            (false, false) => panic!("Something went horribly wrong. Both kings are gone?"),
        }
    }

    pub fn start_game(&mut self) -> ChrustResult<String> {
        self.print_to_screen("Initial".to_string());

        let mut turn_num: u32 = 1;

        let winner = loop {
            //Human moves
            if !self.human_moves()? {
                println!("Had some difficulty parsing your input there, wanna try again?");
                continue;
            }

            if self.human_plays {
                self.print_to_screen(format!("after ai turn {}", turn_num));
            } else {
                thread::sleep(Duration::from_millis(self.tick_speed));
            }

            //AI moves
            if !self.ai_moves()? {
                continue;
            }

            self.print_to_screen(format!("after ai turn {}", turn_num));

            turn_num += 1;

            if let Some(w) = self.check_for_winner() {
                break w;
            }
        };

        Ok(winner)
    }

    pub fn ai_moves(&mut self) -> ChrustResult<bool> {
        println!("ai moving");

        let _ = max_decision(&self.board, self.ai_player.color, 2)?;

        todo!("I don't care about this functionality anymore. Not implementing");
        // self.board.move_piece(&ai_move);
        //Ok(true)
    }

    pub fn human_moves(&mut self) -> ChrustResult<bool> {
        println!("Human moving!");

        let (_, _) = if self.human_plays {
            println!("Select your piece!");

            println!("x:");
            let x = match get_input::<char>() {
                Ok(c) => c,
                Err(_) => {
                    return Ok(false);
                }
            };

            println!("y:");
            let y = match get_input::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    return Ok(false);
                }
            };

            let current_position = Coordinate::new(x, y);

            //this is syntax is really cool
            let coord_choices = match self
                .board
                .possible_moves_human(current_position, &self.human_player)
            {
                Ok(choices) => {
                    if choices.is_empty() {
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

            let choice = match get_input::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    return Ok(false);
                }
            };

            let coord_choice = coord_choices[choice as usize];
            println!("Choice: {:?}", coord_choice);

            (current_position, coord_choice)
        } else {
            let cells = self
                .board
                .cells_with_pieces_with_color(self.human_player.color);

            let (from, to) = loop {
                let random_white_square = cells.choose(&mut rand::thread_rng());

                let cell = random_white_square
                    .ok_or("There was an error while trying to get the human's player's squares")?;

                let from = Coordinate::new(cell.x, cell.y);

                let coord_choices = self.board.possible_moves_human(from, &self.human_player)?;

                if coord_choices.is_empty() {
                    //that piece can't go anywhere - try to get another one
                    continue;
                }

                let to = *coord_choices
                    .choose(&mut rand::thread_rng())
                    .ok_or("There was an error while trying to get a choice randomly")?;

                break (from, to);
            };

            (from, to)
        };

        todo!("I don't care about this functionality anymore. Not implementing");
        // self.board.move_piece(from, to, Color::White);
        //Ok(true)
    }

    fn user_of_color(&self, color: Color) -> &dyn ChessPlayer {
        match (
            self.human_player.color == color,
            self.ai_player.color == color,
        ) {
            (true, true) => panic!("Players with the same color?"),
            (true, false) => &self.human_player,
            (false, true) => &self.ai_player,
            (false, false) => panic!("Players with the same color?"),
        }
    }

    fn print_to_screen(&mut self, configuration_name: String) {
        self.board.print_to_screen(&configuration_name);
        println!("{}", self.board.fen());
    }
}

pub fn process_fen(
    fen: String,
    source: Coordinate,
    target: Coordinate,
    num_plies: u32,
) -> ChrustResult<String> {
    let board_before_white_action = ChessBoard::load_from_fen(&fen)?;

    board_before_white_action.print_to_screen(&board_before_white_action.fen());

    let chess_piece = board_before_white_action.piece(source.x, source.y)?;

    let options = chess_piece.available_moves(&board_before_white_action);

    let binding = options
        .iter()
        .filter(|o| o.0.x == target.x && o.0.y == target.y)
        .map(|o| o.1)
        .collect_vec();
    let move_type = binding
        .first()
        .expect("No move found for that input :(");

    let action = &PieceMove::new(chess_piece, source, target, Color::White, *move_type);

    let board_after_white_action = board_before_white_action.apply_action(action)?;

    board_after_white_action.print_to_screen(&board_after_white_action.fen());

    let ai_move = max_decision(&board_after_white_action, Color::Black, num_plies)?;

    let board_after_black_action = board_after_white_action.apply_action(&ai_move)?;

    board_after_black_action.print_to_screen("ai move");

    Ok(board_after_black_action.fen())
}

pub fn is_valid(
    fen: String,
    current_location: String,
    possible_location: String,
) -> ChrustResult<bool> {
    let board = ChessBoard::load_from_fen(&fen)?;

    let current_position = current_location.to_coord();

    let piece = board.piece(current_position.x, current_position.y)?;
    let possible_moves = piece.available_moves(&board);

    let possible_position = possible_location.to_coord();

    let valid_move = possible_moves
        .iter()
        .any(|coord| coord.0.x == possible_position.x && coord.0.y == possible_position.y);

    Ok(valid_move)
}

pub fn valid_moves(fen: String, location: String) -> ChrustResult<Vec<Coordinate>> {
    let board = ChessBoard::load_from_fen(&fen)
        .unwrap_or_else(|_| panic!("Unable to load board from fen: {}", fen));

    if location.as_str().len() != 2 {
        return Err(Box::from("Invalid format for location string"));
    }

    let current_position = location.to_coord();

    let piece = board.piece(current_position.x, current_position.y)?;
    if piece.color() == Color::Black {
        return Ok(vec![]);
    }

    let coords = piece
        .available_moves(&board)
        .iter()
        .map(|c| c.0)
        .collect_vec();

    Ok(coords)
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
