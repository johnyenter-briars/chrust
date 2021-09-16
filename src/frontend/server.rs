use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use rocket::Error;
use rocket::{
    fs::{relative, FileServer},
    get,
    http::Status,
    launch, post, routes, Build, Rocket, State,
};

use crate::{
    board::{cell::Cell, coordinate::Coordinate},
    game::chessgame::ChessGame,
};

use super::responses::MoveOptionsResponse;
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

// pub struct Person {
//     name: String,
//     age: u16
// }

// use std::io::Cursor;

// use rocket::request::Request;
// use rocket::response::{self, Response, Responder};
// use rocket::http::ContentType;

// impl<'r> Responder<'r, 'static> for Person {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         let person_string = format!("{}:{}", self.name, self.age);
//         Response::build()
//             .sized_body(person_string.len(), Cursor::new(person_string))
//             .raw_header("X-Person-Name", self.name)
//             .raw_header("X-Person-Age", self.age.to_string())
//             .header(ContentType::new("application", "x-person"))
//             .ok()
//     }
// }

#[post("/process/<fen>")]
pub async fn process(fen: String, game: &State<SharedGame>) -> String {
    let returned_fen = game.chess_game.lock().unwrap().process_fen(fen);
    returned_fen
}

#[get("/validate/<fen>/<location>")]
pub async fn validate(
    fen: String,
    location: String,
    game: &State<SharedGame>,
) -> MoveOptionsResponse {
    let game = game.chess_game.lock().unwrap();

    let move_options = game.valid_moves(fen, location).unwrap_or(vec![]);

    MoveOptionsResponse {
        options: move_options,
    }
}

// #[get("/person/<id>")]
// fn person(id: usize) -> Option<Person> {
//     Some(Person{name:"test".to_string(), age: 10})
// }

// #[get("/personidk/<id>")]
// fn personidk(id: usize) -> Person {
//     Person{name:"test".to_string(), age: 10}
// }

pub async fn build_and_run_frontend(game: ChessGame) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(SharedGame {
            chess_game: Mutex::from(game),
        })
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![process])
        .mount("/api", routes![validate])
        // .mount("/api", routes![validate])
        .launch()
        .await
}
