use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use rocket::Error;
use rocket::{
    fs::{relative, FileServer},
    get,
    http::Status,
    launch, post, routes, Build, Rocket, State,
};

use crate::{
    board::{cell::Cell, coordinate::Coordinate},
    game::chessgame::ChessGame,
};

use super::responses::{MoveOptionsResponse, ValidateResponse};
pub struct SharedGame {
    chess_game: Mutex<ChessGame>,
}

struct MyConfig {
    user_val: String,
}

#[post("/process/<fen>")]
pub async fn process(fen: String, game: &State<SharedGame>) -> String {
    // TODO - have error handeling in the case that we can't unwrap cause the lock is poisoned
    let returned_fen = game.chess_game.lock().unwrap().process_fen(fen);
    returned_fen
}

#[get("/possible/<fen>/<location>")]
pub async fn possible(
    fen: String,
    location: String,
    game: &State<SharedGame>,
) -> MoveOptionsResponse {
    // TODO - have error handeling in the case that we can't unwrap cause the lock is poisoned
    let game = game.chess_game.lock().unwrap();

    let move_options = game.valid_moves(fen, location).unwrap_or(vec![]);

    MoveOptionsResponse {
        options: move_options,
    }
}

#[get("/validate/<current_fen>/<current_location>/<possible_location>")]
pub async fn validate(
    current_fen: String,
    current_location: String,
    possible_location: String,
    game: &State<SharedGame>,
) -> ValidateResponse {
    // TODO - have error handeling in the case that we can't unwrap cause the lock is poisoned
    let game = game.chess_game.lock().unwrap();

    let is_valid = game.is_valid(current_fen, current_location, possible_location).unwrap_or(false);

    ValidateResponse{is_valid}
}

pub async fn build_and_run_frontend(game: ChessGame) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(SharedGame {
            chess_game: Mutex::from(game),
        })
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![process])
        .mount("/api", routes![possible])
        .mount("/api", routes![validate])
        // .mount("/api", routes![validate])
        .launch()
        .await
}
