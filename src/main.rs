#![allow(unused)]

mod board;
use board::*;
use board::{cell::color::Color, chessboard::Board};

mod player;
use player::aiplayer::AIPlayer;
use player::chessplayer::ChessPlayer;
use player::humanplayer::HumanPlayer;

mod game;
use game::chessgame::ChessGame;

use crate::board::cell::chesspiece::ChessPiece;

mod chessmove;

mod visualize;
use visualize::visualizer::Visualizer;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board_result = Board::load_from_file("game_start_pawn_close");

    let board = match board_result {
        Ok(brd) => brd,
        Err(error) => panic!("error in creating the board: {}", error),
    };

    let human = HumanPlayer::new("kasparov", Color::White);
    let ai = AIPlayer::new("rusty", Color::Black);

    let mut game = ChessGame::new(human, ai, board); //values are MOVED

    // let winner = match game.start_game() {
    //     Ok(wnnr) => wnnr,
    //     Err(err) => {
    //         panic!("Error in game!: {:?}", err)
    //     }
    // };
    // println!("Winner: {}", winner);
	
    let mut viz = Visualizer::new(game);
	viz.start_viz();

	Ok(())
}