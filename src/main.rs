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

mod state;
use state::programstate::ProgramState;
use state::viztype::VizType;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;
use clap::{App, Arg};

fn get_args() -> Result<ProgramState, Box<dyn std::error::Error>> {
    let matches = App::new("Chrust")
        .version("0.1")
        .author("John YB. <jyenterbriars@gmail.com>")
        .about("Simple Chess Engine")
        .arg(
            Arg::new("viz")
                .short('z')
                .long("visualization_mode")
                .value_name("GUI|TERM")
                .about("Sets the method of visualization for the app")
                .takes_value(true)
                .required(true)
                
        )
        .arg(
            Arg::new("hplay")
                .short('h')
                .long("human_plays")
                .value_name("true|false")
                .about("Sets whether or not the human player will play the game. If false, the human player makes random decisions")
                .takes_value(true)
                .default_value("true")
        )
        .get_matches();

    let viz_type = matches.value_of("viz").ok_or("idk")?;
    let human_plays = matches.value_of("hplay").ok_or("idk")?;

    let program_state = ProgramState {
        viz_type: match viz_type {
            "GUI" => VizType::GUI,
            "TERM" => VizType::TERM,
            _ => {
                return Err(Box::from("You must pass in a vaid argument for the -v flag!",));
            }
        },
        human_plays: match human_plays {
            "true" => true,
            "false" => false,
            _ => {
                return Err(Box::from("You must pass in a vaid argument for the -h flag!",));
            }
        }
    };

    Ok(program_state)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_state = get_args()?;

    let board = Board::load_from_file("game_start_small")?;

    let human_player = HumanPlayer::new("kasparov", Color::White);
    let ai_player = AIPlayer::new("rusty", Color::Black);

    let mut game = ChessGame::new(human_player, ai_player, board, program_state.human_plays); //values are MOVED

    match program_state.viz_type {
        VizType::TERM => {
            let winner = match game.start_game() {
                Ok(wnnr) => wnnr,
                Err(err) => {
                    panic!("Error in game!: {:?}", err)
                }
            };
            println!("Winner: {}", winner);
            Ok(())
        }
        VizType::GUI => {
            let mut viz = Visualizer::new(game);
            viz.start_viz();
            Ok(())
        }
    }
}
