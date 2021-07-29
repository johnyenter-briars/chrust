mod board;
use board::*;

mod player;
use player::humanplayer::HumanPlayer;
use player::aiplayer::AIPlayer;
use player::chessplayer::ChessPlayer;

mod game;
use game::chessgame::ChessGame;


#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() {
    let board_result = board::Board::load_from_file("game_start");

    let board1: Board = match board_result {
        Ok(brd) => {brd}
        Err(error)  => panic!("error! {}", error)
    };
    println!("{:?} ", board1);

    let human = HumanPlayer::new("kasparov");
    let ai = AIPlayer::new("rusty");


    let mut game = ChessGame::new(human, ai); //values are MOVED


    let winner = match game.start_game() {
        Ok(wnnr) => wnnr,
        Err(err) => {panic!("Error in game!: {:?}", err)}
    };

    println!("Winer is: {}", winner);
}

