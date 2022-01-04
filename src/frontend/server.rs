use super::{
    cors::CORS,
    responses::{SettingsResponse, ValidateResponse},
};
use crate::{
    frontend::responses::MoveOptionsResponse,
    game::chessgame::{is_valid, process_fen, valid_moves},
    state::programstate::ProgramState,
};
use rocket::{
    get,
    http::ContentType,
    post,
    response::{
        content::{Css, Custom, Html, JavaScript},
    },
    routes, State,
};
use std::{collections::HashMap, sync::Mutex};

pub struct Settings {
    program_state: Mutex<ProgramState>,
}

pub struct ImageMap {
    map: HashMap<String, Vec<u8>>,
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

#[get("/")]
fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

#[get("/chrust.js")]
fn javascript() -> JavaScript<&'static str> {
    JavaScript(include_str!("../../static/js/chrust.js"))
}

#[get("/chrust.css")]
fn css() -> Css<&'static str> {
    Css(include_str!("../../static/css/chrust.css"))
}

// /img/chesspieces/wikipedia/wB.png
#[get("/chesspieces/wikipedia/<piece>")]
fn chesspiece(piece: String, image_map: &State<ImageMap>) -> Custom<Vec<u8>> {
    let byte_vec = match image_map.map.get(&piece.replace(".png", "")) {
        Some(byte_vec) => byte_vec,
        None => panic!("Unable to find the piece image in the image_map!"),
    };

    Custom(ContentType::PNG, byte_vec.clone())
}

fn init_image_map() -> HashMap<String, Vec<u8>> {
    HashMap::from([
        (
            "bB".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/bB.png").to_vec(),
        ),
        (
            "bK".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/bK.png").to_vec(),
        ),
        (
            "bN".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/bN.png").to_vec(),
        ),
        (
            "bP".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/bP.png").to_vec(),
        ),
        (
            "bQ".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/bQ.png").to_vec(),
        ),
        (
            "bR".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/bR.png").to_vec(),
        ),
        (
            "wB".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/wB.png").to_vec(),
        ),
        (
            "wK".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/wK.png").to_vec(),
        ),
        (
            "wN".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/wN.png").to_vec(),
        ),
        (
            "wP".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/wP.png").to_vec(),
        ),
        (
            "wQ".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/wQ.png").to_vec(),
        ),
        (
            "wR".to_string(),
            include_bytes!("../../static/img/chesspieces/wikipedia/wR.png").to_vec(),
        ),
    ])
}

pub async fn build_and_run_frontend(ps: ProgramState) -> Result<(), rocket::Error> {
    let cors = CORS {};

    rocket::build()
        .attach(cors)
        .manage(Settings {
            program_state: Mutex::from(ps),
        })
        .manage(ImageMap {
            map: init_image_map(),
        })
        .mount("/chrust", routes![index])
        .mount("/chrust/js", routes![javascript])
        .mount("/chrust/css", routes![css])
        .mount("/img", routes![chesspiece])
        .mount("/chrust/api", routes![process])
        .mount("/chrust/api", routes![possible])
        .mount("/chrust/api", routes![validate])
        .mount("/chrust/api", routes![settings])
        .launch()
        .await
}
