use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use rocket::{Build, Rocket, State, fs::{relative, FileServer}, get, http::Status, launch, post, response::{self, status}, routes};
use rocket::Error;

use crate::{
    board::{cell::Cell, coordinate::Coordinate},
    game::chessgame::ChessGame,
};
pub struct SharedGame {
    chess_game: Mutex<ChessGame>,
}

struct MyConfig {
    user_val: String,
}

// #[get("/")]
// fn index(state: &State<MyConfig>) -> String {
//     state.user_val = String::from("test");
//     format!("The config value is: {}", state.user_val)
// }

#[post("/process/<fen>")]
pub async fn process(fen: String, game: &State<SharedGame>) -> String {
    let returned_fen = game.chess_game.lock().unwrap().process_fen(fen);
    returned_fen
}

// #[get("/validate/<fen>/<location>")]
// pub async fn validate(fen: String, location: String, game: &State<SharedGame>) ->  Result<status::Custom<JSON<Vec<Coordinate>>>, Error> {
//     match game.chess_game.lock().unwrap().valid_moves(fen, location) {
//         Ok(_) => {
//             let response = status::BadRequest(Some("error message"));

//             response
//         },
//         Err(_) => {
//             let response = status::BadRequest(Some("error message"));

//             response
//         }
//     }
// }

pub async fn build_and_run_frontend(game: ChessGame) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(SharedGame {
            chess_game: Mutex::from(game),
        })
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![process])
        // .mount("/api", routes![validate])
        .launch()
        .await
}
