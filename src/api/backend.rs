use rocket::{
    fs::{relative, FileServer},
    get, launch, routes, Build, Config, Rocket,
};

use crate::game::chessgame::ChessGame;

pub struct Server {}
mod manual {
    use rocket::{fs::NamedFile, get};
    use std::path::{Path, PathBuf};

    #[get("/second")]
    pub async fn second() -> &'static str {
        "second - API"
    }

    #[get("/first")]
    pub async fn first() -> &'static str {
        "first - API"
    }
}

pub async fn build_and_run_api(game: ChessGame<'_>) -> Result<(), rocket::Error> {
    let figment = rocket::Config::figment().merge(("port", 8001));

    rocket::custom(figment)
        .mount("/", rocket::routes![manual::first])
        .mount("/", rocket::routes![manual::second])
        .launch()
        .await
}
