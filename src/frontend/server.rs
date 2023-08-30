use super::response::ValidateResponse;
use crate::{
    board::{coordinate::ToCoord, piece::piecetype::ToPieceType},
    frontend::response::{MoveOptionsResponse, SettingsResponse},
    game::chessgame::{is_valid, process_fen, valid_moves},
    state::programstate::ProgramState,
};
use actix_web::web;
use actix_web::{get, post, HttpResponse};
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::{Responder, Result};
use std::{collections::HashMap, sync::Mutex};

pub struct Settings {
    program_state: Mutex<ProgramState>,
    image_map: Mutex<HashMap<String, Vec<u8>>>,
}

#[post("/api/process/{fen}/{piece_coord}/{target_coord}/{default_promotion_piece}")]
pub async fn process(
    args: web::Path<(String, String, String, String)>,
    s: web::Data<Settings>,
) -> String {
    let fen = &args.0;
    let source = args.1.to_coord();
    let target = args.2.to_coord();
    let default_promotion_piece = &args
        .3
        .to_piece_type()
        .expect("No valid default promotion piece found");
    let program_state = *s.program_state.lock().unwrap();
    let returned_fen = match process_fen(
        fen.to_string(),
        source,
        target,
        *default_promotion_piece,
        program_state.num_plies,
    ) {
        Ok(f) => f,
        Err(e) => panic!("paniced because! {}", e),
    };
    returned_fen
}

#[get("/api/possible/{fen}/{location}")]
pub async fn possible(query: web::Path<(String, String)>) -> Result<impl Responder> {
    let options = valid_moves(query.0.to_string(), query.1.to_string())?;
    Ok(web::Json(MoveOptionsResponse { options }))
}

#[get("/api/validate/{current_fen}/{current_location}/{possible_location}")]
pub async fn validate(query: web::Path<(String, String, String)>) -> Result<impl Responder> {
    let is_valid = is_valid(
        query.0.to_string(),
        query.1.to_string(),
        query.2.to_string(),
    )?;
    Ok(web::Json(ValidateResponse { is_valid }))
}

#[get("/api/settings")]
async fn settings(settings: web::Data<Settings>) -> Result<impl Responder> {
    let program_state = *settings.program_state.lock().unwrap();
    Ok(web::Json(SettingsResponse { program_state }))
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../static/index.html"))
}

#[get("js/chrust.js")]
async fn javascript() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(include_str!("../../static/js/chrust.js"))
}

#[get("css/chrust.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("../../static/css/chrust.css"))
}

// /img/chesspieces/wikipedia/wB.png
#[get("img/chesspieces/wikipedia/{piece}")]
async fn chesspiece(piece: web::Path<String>, s: web::Data<Settings>) -> HttpResponse {
    let image_map = s.image_map.lock().unwrap().clone();
    let byte_vec = match image_map.get(&piece.replace(".png", "")) {
        Some(byte_vec) => byte_vec,
        None => panic!("Unable to find the piece image in the image_map!"),
    };
    HttpResponse::Ok()
        .content_type("image/png")
        .body(byte_vec.to_owned())
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

pub async fn build_and_run_frontend(program_state: ProgramState) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(Settings {
        program_state: Mutex::new(program_state),
        image_map: Mutex::new(init_image_map()),
    });

    HttpServer::new(move || {
        App::new().service(
            web::scope("/chrust")
                .service(possible)
                .service(process)
                .service(validate)
                .service(css)
                .service(javascript)
                .service(index)
                .service(chesspiece)
                .service(settings)
                .wrap(Logger::default())
                .app_data(app_state.clone()),
        )
    })
    .bind(("0.0.0.0".to_string(), 8083))?
    .run()
    .await
}
