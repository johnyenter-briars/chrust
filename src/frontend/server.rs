use rocket::{
    fs::{relative, FileServer},
    get, launch, routes, Build, Rocket,
};

mod api {
    use rocket::{fs::NamedFile, get};
    use std::path::{Path, PathBuf};

    #[get("/test")]
    pub async fn test() -> &'static str {
        "YOU HIT THE API"
    }
}

pub async fn build_and_run_frontend() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![api::test])
        .launch()
        .await
}