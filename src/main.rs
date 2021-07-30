#![allow(unused)]

mod board;
use board::*;
use board::{chessboard::Board, cell::color::Color};

mod player;
use player::humanplayer::HumanPlayer;
use player::aiplayer::AIPlayer;
use player::chessplayer::ChessPlayer;

mod game;
use game::chessgame::ChessGame;
use tui::layout::Direction;
use tui::text::Span;
use tui::widgets::BorderType;

mod chessmove;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{error::Error, io};
// use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color as TUIColor, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Terminal,
};

use termion::raw::IntoRawMode;


pub struct StatefulTable<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> StatefulTable<'a> {
    fn new() -> StatefulTable<'a> {
        StatefulTable {
            state: TableState::default(),
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62\nTest", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"],
                vec!["Row101", "Row102", "Row103"],
                vec!["Row111", "Row112", "Row113"],
                vec!["Row121", "Row122", "Row123"],
                vec!["Row131", "Row132", "Row133"],
                vec!["Row141", "Row142", "Row143"],
                vec!["Row151", "Row152", "Row153"],
                vec!["Row161", "Row162", "Row163"],
                vec!["Row171", "Row172", "Row173"],
                vec!["Row181", "Row182", "Row183"],
                vec!["Row191", "Row192", "Row193"],
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let board_result = Board::load_from_file("game_start_better");

    // let board  = match board_result {
    //     Ok(brd) => {brd}
    //     Err(error)  => panic!("error in creating the board: {}", error)
    // };

    // let human = HumanPlayer::new("kasparov", Color::White);
    // let ai = AIPlayer::new("rusty", Color::Black);


    // let mut game = ChessGame::new(human, ai, board); //values are MOVED


    // let winner = match game.start_game() {
    //     Ok(wnnr) => wnnr,
    //     Err(err) => {panic!("Error in game!: {:?}", err)}
    // };


    // println!("Winer is: {}", winner);



     // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    // let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    // let events = Events::new();

    loop {
        terminal.draw(|f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Main block with round corners")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);
            let block = Block::default()
                .title(vec![
                    Span::styled("With", Style::default().fg(TUIColor::Yellow)),
                    Span::from(" background"),
                ])
                .style(Style::default().bg(TUIColor::Green));
            f.render_widget(block, top_chunks[0]);

            let block = Block::default().title(Span::styled(
                "Styled title",
                Style::default()
                    .fg(TUIColor::White)
                    .bg(TUIColor::Red)
                    .add_modifier(Modifier::BOLD),
            ));
            f.render_widget(block, top_chunks[1]);

            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, bottom_chunks[0]);
            let block = Block::default()
                .title("With styled borders and doubled borders")
                .border_style(Style::default().fg(TUIColor::Cyan))
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_type(BorderType::Double);
            f.render_widget(block, bottom_chunks[1]);
        })?;

        // if let Event::Input(key) = events.next()? {
        //     if key == Key::Char('q') {
        //         break;
        //     }
        // }
    }
    Ok(())
}

