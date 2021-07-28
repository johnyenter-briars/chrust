extern crate serde_derive;
extern crate serde_json;

pub mod cell;
use cell::Cell;
use std::fs::File;
use std::io::{BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    squares: Vec<Vec<Cell>>,
    board_size: i32,
}

impl Board {
    pub fn new(board_size: i32) -> Board {
        let mut squares: Vec<Vec<Cell>> = Vec::new();

        for _ in 0..board_size {
            let mut sub_squares: Vec<Cell> = Vec::new();
            for _ in 0..board_size {
                sub_squares.push(Cell {
                    space: Option::None,
                });
            }

            squares.push(sub_squares);
        }

        Board {
            squares,
            board_size,
        }
    }
 
    pub fn load_from_file(board_name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let mut current_dir =
            std::env::current_dir().expect("Cant find the path to the current directory!");

        current_dir.push("src");
        current_dir.push("boards");
        current_dir.push(format!("{}.json", board_name));

        if ! current_dir.is_file() {
            return Err(Box::from("Not a valid file path to board!"));
        }

        let file = File::open(current_dir)?;

        let reader = BufReader::new(file);

        let board: Board = serde_json::from_reader(reader)?;

        Ok(board)
    }
}
