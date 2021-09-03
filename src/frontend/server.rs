use rocket::{
    fs::{relative, FileServer},
    get, launch, routes, Build, Rocket,
};

mod api {
    use rocket::{fs::NamedFile, get};
    use std::path::{Path, PathBuf};

    #[get("/process")]
    pub async fn process() -> &'static str {
        "rnbqkbnr/1ppppppp/p7/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 1"
    }
}

pub async fn build_and_run_frontend() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![api::process])
        .launch()
        .await
}