use rocket::{
    fs::{relative, FileServer},
    get, launch, routes, Build, Rocket,
};

pub struct Server {}

mod manual {
    use rocket::fs::NamedFile;
    use std::path::{Path, PathBuf};

    #[rocket::get("/second/<path..>")]
    pub async fn second(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("static")).join(path);
        if path.is_dir() {
            path.push("index.html");
        }

        NamedFile::open(path).await.ok()
    }
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        // .mount("/", rocket::routes![manual::second])
        .mount("/", FileServer::from(relative!("static")))
}

pub async fn build_and_run_frontend() -> Result<(), rocket::Error> {
    // rocket::build()
    //     .mount("/", routes![world])
    //     .launch()
    //     .await
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await
}

pub fn test() {

}
