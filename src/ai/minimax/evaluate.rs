use crate::board::{
    cell::{chesspiece::ChessPiece, color::Color, piecetype::PieceType},
    chessboard::Board,
};

//NOTE: this evaluation function assumes that the WHITE is the MIN player and BLACK is the MAX player
pub fn evaluate(board: &Board) -> i32 {
    let cells = board.all_cells();
    cells
        .iter()
        .map(|c| match c.space {
            Some(p) => {
                let abs_value = piece_value(p.piece_type) + piece_position_score(&p, c.x, c.y);
                if p.color == Color::White {
                    -abs_value
                } else {
                    abs_value
                }
            }
            None => 0,
        })
        .sum()
}

fn piece_value(piece_type: PieceType) -> i32 {
    match piece_type {
        PieceType::Pawn => 10,
        PieceType::Rook => 50,
        PieceType::Knight => 30,
        PieceType::Bishop => 30,
        PieceType::Queen => 90,
        PieceType::King => 900,
    }
}

fn piece_position_score(p: &ChessPiece, x: char, y: i32) -> i32 {
    let vector = piece_eval_vector(p.piece_type, p.color);
    vector[char_to_int(x) as usize][y as usize - 1] as i32
}

fn char_to_int(x: char) -> i32 {
    match x {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Coordinate not well structured!"),
    }
}

fn piece_eval_vector(piece_type: PieceType, color: Color) -> Vec<Vec<f64>> {
    match (piece_type, color) {
        (PieceType::Pawn, Color::White) => arr_to_vec(PAWN_EVAL_WHITE),
        (PieceType::Pawn, Color::Black) => reverse(arr_to_vec(PAWN_EVAL_WHITE)),
        (PieceType::Rook, Color::White) => arr_to_vec(ROOK_EVAL_WHITE),
        (PieceType::Rook, Color::Black) => reverse(arr_to_vec(ROOK_EVAL_WHITE)),
        (PieceType::Knight, Color::White) => arr_to_vec(KNIGHT_EVAL_WHITE),
        (PieceType::Knight, Color::Black) => reverse(arr_to_vec(KNIGHT_EVAL_WHITE)),
        (PieceType::Bishop, Color::White) => arr_to_vec(BISHOP_EVAL_WHITE),
        (PieceType::Bishop, Color::Black) => reverse(arr_to_vec(BISHOP_EVAL_WHITE)),
        (PieceType::Queen, Color::White) => arr_to_vec(QUEEN_EVAL_WHITE),
        (PieceType::Queen, Color::Black) => reverse(arr_to_vec(QUEEN_EVAL_WHITE)),
        (PieceType::King, Color::White) => arr_to_vec(KING_EVAL_WHITE),
        (PieceType::King, Color::Black) => reverse(arr_to_vec(KING_EVAL_WHITE)),
    }
}

fn arr_to_vec(arr: [[f64; 8]; 8]) -> Vec<Vec<f64>> {
    arr.to_vec().iter().map(|row| row.to_vec()).collect()
}

fn reverse(vec: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    vec.into_iter().rev().collect::<Vec<Vec<f64>>>()
}

static PAWN_EVAL_WHITE: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.5, 1.0, 1.0, -2.0, -2.0, 1.0, 1.0, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

static KNIGHT_EVAL_WHITE: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

static BISHOP_EVAL_WHITE: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

static ROOK_EVAL_WHITE: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
];

static QUEEN_EVAL_WHITE: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
];

static KING_EVAL_WHITE: [[f64; 8]; 8] = [
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
];
