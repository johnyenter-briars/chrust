use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color as TUIColor, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
        TableState, Tabs,
    },
    Terminal,
};

use crate::{
    board::{cell::color::Color, chessboard::Board},
    game::chessgame::ChessGame,
};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Game,
    Settings,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Game => 1,
            MenuItem::Settings => 2,
        }
    }
}

pub struct Visualizer {
    game: ChessGame,
}

impl Visualizer {
    pub fn new(game: ChessGame) -> Self {
        Visualizer { game }
    }

    pub fn start_viz(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode().expect("can run in raw mode");

        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let menu_titles = vec!["Home", "Game", "Settings", "Quit"];
        let mut active_menu_item = MenuItem::Home;
        let mut table_state = TableState::default();
        // table_state.select(Some(0));
        table_state.select(Some(0));

        loop {
            terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(2),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let menu = menu_titles
                    .iter()
                    .map(|t| {
                        let (first, rest) = t.split_at(1);
                        Spans::from(vec![
                            Span::styled(
                                first,
                                Style::default()
                                    .fg(TUIColor::Yellow)
                                    .add_modifier(Modifier::UNDERLINED),
                            ),
                            Span::styled(rest, Style::default().fg(TUIColor::White)),
                        ])
                    })
                    .collect();

                let tabs = Tabs::new(menu)
                    .select(active_menu_item.into())
                    .block(Block::default().title("Menu").borders(Borders::ALL))
                    .style(Style::default().fg(TUIColor::White))
                    .highlight_style(Style::default().fg(TUIColor::Yellow))
                    .divider(Span::raw("|"));

                rect.render_widget(tabs, chunks[0]);

                match active_menu_item {
                    MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                    MenuItem::Game => {
                        rect.render_stateful_widget(
                            render_board(&self.game.board),
                            chunks[1],
                            &mut table_state,
                        );
                    }
                    MenuItem::Settings => {
                        let settings_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                            )
                            .split(chunks[1]);
                        let right = render_settings();
                        rect.render_widget(right, settings_chunks[1]);
                    }
                }
            })?;

            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.clear();
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                    KeyCode::Char('g') => active_menu_item = MenuItem::Game,
                    KeyCode::Char('s') => active_menu_item = MenuItem::Settings,
                    KeyCode::Char('x') => {
                        // self.game.board.test_move_piece('a', 2, &self.game.human_player);
                    }
                    _ => {}
                },
                Event::Tick => {}
            }
        }

        Ok(())
    }
}
fn render_board<'a>(board: &'a Board) -> Table<'a> {
    let mut board_cells: Vec<Row> = Vec::new();

    for row in &board.squares {
        let mut new_row: Vec<Cell> = Vec::new();
        for boardcell in row {
            if let Some(piece) = &boardcell.space {
                new_row.push(
                    Cell::from(piece.get_str())
                        .style(Style::default().bg(translate_color(boardcell.color))),
                );
            } else {
                new_row.push(
                    Cell::from("  ").style(Style::default().bg(translate_color(boardcell.color))),
                );
            }
        }
        board_cells.push(Row::new(new_row));
    }

    let board_detail = Table::new(board_cells)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(TUIColor::White))
                .title("Board")
                .border_type(BorderType::Thick),
        )
        .highlight_style(Style::default().fg(TUIColor::Black))
        .column_spacing(1)
        .widths(&[
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);
    board_detail
}

fn translate_color(color: Color) -> TUIColor {
    match color {
        Color::White => TUIColor::LightBlue,
        Color::Black => TUIColor::Gray,
    }
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "CHRUST",
            Style::default().fg(TUIColor::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'g' to access the game, 's' to access settings, and 'h' to return home")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(TUIColor::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_settings<'a>() -> Table<'a> {
    let settings_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw("test".to_string())),
        Cell::from(Span::raw("test".to_string())),
        Cell::from(Span::raw("test".to_string())),
        Cell::from(Span::raw("test".to_string())),
        Cell::from(Span::raw("test".to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(TUIColor::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    settings_detail
}
