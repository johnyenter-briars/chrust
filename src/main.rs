#![allow(unused)]

mod board;
use std::env;

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

mod ai;

mod visualize;
use visualize::visualizer::Visualizer;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;
use clap::{Arg, App};

fn get_args() {
    let matches = App::new("Chrust")
        .version("1.0")
        .author("John YB. <jyenterbriars@gmail.com>")
        .about("Simple Chess Engine")
        .arg(Arg::new("viz")
            .short('z')
            .long("visualization_mode")
            .value_name("GUI|TERM")
            .about("Sets the method of visualization for the app")
            .takes_value(true))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(i) = matches.value_of("INPUT") {
        println!("Value for input: {}", i);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    if let Some(c) = matches.value_of("viz") {
        println!("Value for viz: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("debug") {
            // "$ myapp test -d" was run
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    get_args();

    return Ok(());

    let board_result = Board::load_from_file("game_start_small");

    let board = match board_result {
        Ok(brd) => brd,
        Err(error) => panic!("error in creating the board: {}", error),
    };

    let human = HumanPlayer::new("kasparov", Color::White);
    let ai = AIPlayer::new("rusty", Color::Black);

    let mut game = ChessGame::new(human, ai, board); //values are MOVED

    let winner = match game.start_game() {
        Ok(wnnr) => wnnr,
        Err(err) => {
            panic!("Error in game!: {:?}", err)
        }
    };
    println!("Winner: {}", winner);
	
    // let mut viz = Visualizer::new(game);
	// viz.start_viz();

	Ok(())
}