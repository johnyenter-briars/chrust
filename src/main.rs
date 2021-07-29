mod board;
use board::*;
use board::{chessboard::Board, cell::color::Color};

mod player;
use player::humanplayer::HumanPlayer;
use player::aiplayer::AIPlayer;
use player::chessplayer::ChessPlayer;

mod game;
use game::chessgame::ChessGame;

mod chessmove;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() {
    let board_result = Board::load_from_file("game_start_better");

    let board  = match board_result {
        Ok(brd) => {brd}
        Err(error)  => panic!("error in creating the board: {}", error)
    };

    let human = HumanPlayer::new("kasparov", Color::White);
    let ai = AIPlayer::new("rusty", Color::Black);


    let mut game = ChessGame::new(human, ai, board); //values are MOVED


    let winner = match game.start_game() {
        Ok(wnnr) => wnnr,
        Err(err) => {panic!("Error in game!: {:?}", err)}
    };


    println!("Winer is: {}", winner);
}

