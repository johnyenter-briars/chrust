use rocket::routes;
use rocket::{fs::FileServer, get, post};

use rocket::fs::relative;

use crate::game::chessgame::{is_valid, process_fen, valid_moves};

use super::responses::{MoveOptionsResponse, ValidateResponse};
// pub struct sharedgame {
//     chess_game: mutex<chessgame>,
// }

#[post("/process/<fen>")]
pub async fn process(fen: String) -> String {
    // TODO - have error handeling in the case that we can't unwrap cause the lock is poisoned
    // let mut game = game.chess_game.lock().unwrap();
    let returned_fen = match process_fen(fen) {
        Ok(f) => f,
        Err(e) => panic!("paniced because! {}", e),
    };
    returned_fen
}

#[get("/possible/<fen>/<location>")]
pub async fn possible(fen: String, location: String) -> MoveOptionsResponse {
    // TODO - have error handeling in the case that we can't unwrap cause the lock is poisoned
    // let game = game.chess_game.lock().unwrap();

    let move_options = valid_moves(fen, location).unwrap_or(vec![]);

    MoveOptionsResponse {
        options: move_options,
    }
}

#[get("/validate/<current_fen>/<current_location>/<possible_location>")]
pub async fn validate(
    current_fen: String,
    current_location: String,
    possible_location: String,
) -> ValidateResponse {
    // TODO - have error handeling in the case that we can't unwrap cause the lock is poisoned
    // let game = game.chess_game.lock().unwrap();

    let is_valid = is_valid(current_fen, current_location, possible_location).unwrap_or(false);

    ValidateResponse { is_valid }
}

pub async fn build_and_run_frontend() -> Result<(), rocket::Error> {
    rocket::build()
        // .manage(SharedGame {
        //     chess_game: Mutex::from(game),
        // })
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![process])
        .mount("/api", routes![possible])
        .mount("/api", routes![validate])
        .launch()
        .await
}
