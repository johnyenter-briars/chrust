use chrust::board::{cell::color::Color, chessboard::Board};
use chrust::player::aiplayer::AIPlayer;
use chrust::player::humanplayer::{HumanPlayer};
use chrust::game::chessgame::ChessGame;
use chrust::frontend::server::build_and_run_frontend;
use chrust::state::programstate::get_args;
use chrust::state::viztype::VizType;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_state = get_args()?;
    let board = Board::load_from_file("game_start")?;

    let human_player = HumanPlayer {
        name: "kasparov".to_string(),
        color: Color::White,
    };

    let ai_player = AIPlayer {
        name: "rusty".to_string(),
        color: Color::Black,
    };

    let mut game = ChessGame::new(
        human_player,
        ai_player,
        board,
        program_state.human_plays,
        program_state.tick_speed,
    );

    match program_state.viz_type {
        VizType::Term => {
            let winner = game.start_game()?;
            println!("Winner: {}", &winner);
            Ok(())
        }
        VizType::Web => {
            if build_and_run_frontend(program_state).await.is_err() {
                panic!("Error while trying to start frontend!");
            }
            Ok(())
        }
    }
}
