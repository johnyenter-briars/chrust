pub mod board;
pub mod player;
pub mod game;
mod chessmove;
mod ai;
pub mod state;
pub mod frontend;
mod ext;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;
