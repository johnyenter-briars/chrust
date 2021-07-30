use crate::board::cell::chesspiece::ChessPiece;
use crate::board::chessboard::Board;
use crate::chessmove::piecemove::Move;
use crate::chessmove::{piecemove::PieceMove};
use crate::player::humanplayer::{*, self};
use crate::player::aiplayer::*;

use crate::chessmove::action::{self, Action, HumanAction};

use std::borrow::Borrow;
use std::error::Error;


pub struct ChessGame<'a> {
    human_player: HumanPlayer,
    ai_player: AIPlayer,
    board: Board,
    history: Vec<&'a dyn Action<'a>>
}


impl<'a> ChessGame<'a> {
    pub fn new(human_player: HumanPlayer, ai_player: AIPlayer, board: Board) -> Self {
        ChessGame{human_player, ai_player, board, history:Vec::new()}
    }

    // pub fn get_move(&self) -> &dyn Action {

    // }

    pub fn start_game(&mut self) -> Result<&str, Box<dyn Error>> {

        //know a white pawn is at a,2
        // let first_move = Move {x: 'a', y: 3};

        // let board = &mut self.board;

        // let target_piece = self.board.get_piece('a', 2);

        let idkyet = self.board.test_move_piece('a', 2, &self.human_player);



        // let possible_actions = 
        // let mut target_piece2 = self.board.get_piece('a', 1);
        
        // println!("{:?}", target_piece);
        // println!("{:?}", target_piece2);
        // // let target_piece = self.board.get_piece(first_move.x, first_move.y);

        // let piece_move = PieceMove::new(target_piece, first_move);

        // let human_action = HumanAction::new(piece_move, &self.human_player);

        // let my_action: &dyn Action = &human_action;

        // drop(target_piece);
        // let acn = HumanAction
        // {
        //     piece_move: PieceMove{
        //         chess_move: Move{
        //             x: 'a',
        //             y: 3
        //         },
        //         chess_piece: &ChessPiece{
        //             color: crate::board::cell::color::Color::Black,
        //             piece_type: crate::board::cell::piecetype::PieceType::King
        //         }
        //     },
        //     player: &self.human_player,
        // };
        // self.board.apply_action(&acn);
        // self.board.apply_action(my_action);

        // println!("get piece: {:?}", target_piece);

        Ok(&self.human_player.name)
    }
}