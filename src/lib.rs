pub mod ai;
pub mod board;
pub mod chessmove;
pub mod ext;
pub mod frontend;
pub mod game;
pub mod player;
pub mod state;


#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;