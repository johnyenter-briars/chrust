use std::{ops::DerefMut, sync::{Arc, Mutex}};

use rocket::{Build, Rocket, State, fs::{relative, FileServer}, get, launch, post, routes};

use crate::game::chessgame::ChessGame;
pub struct SharedGame {
    chess_game: Mutex<ChessGame>,
}

struct MyConfig {
    user_val: String
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

pub async fn build_and_run_frontend(game: ChessGame) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(SharedGame{chess_game: Mutex::from(game)})
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![process])
        .launch()
        .await
}
