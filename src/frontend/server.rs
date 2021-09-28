use super::responses::{SettingsResponse, ValidateResponse};
use crate::{
    frontend::responses::MoveOptionsResponse,
    game::chessgame::{is_valid, process_fen, valid_moves},
    state::programstate::ProgramState,
};
use rocket::{
    fs::{relative, FileServer},
    get, post, routes, State,
};
use std::sync::Mutex;

pub struct Settings {
    program_state: Mutex<ProgramState>,
}

#[post("/process/<fen>")]
pub async fn process(fen: String, settings: &State<Settings>) -> String {
    let program_state = *settings.program_state.lock().unwrap();
    let returned_fen = match process_fen(fen, program_state.num_plies) {
        Ok(f) => f,
        Err(e) => panic!("paniced because! {}", e),
    };
    returned_fen
}

#[get("/possible/<fen>/<location>")]
pub async fn possible(fen: String, location: String) -> MoveOptionsResponse {
    let options = valid_moves(fen, location).unwrap_or(vec![]);
    MoveOptionsResponse { options }
}

#[get("/validate/<current_fen>/<current_location>/<possible_location>")]
pub async fn validate(
    current_fen: String,
    current_location: String,
    possible_location: String,
) -> ValidateResponse {
    let is_valid = is_valid(current_fen, current_location, possible_location).unwrap_or(false);
    ValidateResponse { is_valid }
}

#[get("/settings")]
pub async fn settings(settings: &State<Settings>) -> SettingsResponse {
    let program_state = *settings.program_state.lock().unwrap();
    SettingsResponse { program_state }
}

pub async fn build_and_run_frontend(ps: ProgramState) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(Settings {
            program_state: Mutex::from(ps),
        })
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![process])
        .mount("/api", routes![possible])
        .mount("/api", routes![validate])
        .mount("/api", routes![settings])
        .launch()
        .await
}
