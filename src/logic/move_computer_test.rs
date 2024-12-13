use crate::driver::game::Game;
use crate::logic::move_computer::StepComputer;
use crate::models::game_state::GameState;
use crate::models::step::Step;
use crate::models::{
    board::Board,
    location::Location,
    piece::{ColoredPiece, PieceColor, PieceKind},
    step::StepKind,
};
use crate::utils::board_creation::BoardCreation;

#[test]
pub fn simple_check() {
    let mut board = Board::new(8, 8);
    let white_king = ColoredPiece {
        kind: PieceKind::King,
        color: PieceColor::White,
    };
    let black_queen = ColoredPiece {
        kind: PieceKind::Queen,
        color: PieceColor::Black,
    };
    board.put_piece_on_field_xy(white_king, (1, 0), 0);
    board.put_piece_on_field_xy(black_queen, (2, 7), 0);
    println!("check? {}", board.has_check(Some(PieceColor::White)));
}

#[test]
pub fn possible_moves() {
    let mut board = Board::new(8, 8);
    let white_king = ColoredPiece {
        kind: PieceKind::King,
        color: PieceColor::White,
    };
    let black_queen = ColoredPiece {
        kind: PieceKind::Queen,
        color: PieceColor::Black,
    };
    let black_king = ColoredPiece {
        kind: PieceKind::King,
        color: PieceColor::Black,
    };
    board.put_piece_on_field_xy(white_king, (0, 0), 0);
    board.put_piece_on_field_xy(black_king, (1, 4), 0);
    board.put_piece_on_field_xy(black_queen, (1, 3), 0);

    // white's turn, only one possible move:
    let steps = board.compute_steps();
    assert!(steps.len() == 1);
}

#[test]
pub fn castle() {
    let mut board = Board::new(8, 8);
    let white_king = ColoredPiece {
        kind: PieceKind::King,
        color: PieceColor::White,
    };
    let white_rook = ColoredPiece {
        kind: PieceKind::Rook,
        color: PieceColor::White,
    };

    board.put_piece_on_field_xy(white_king, (4, 0), 0);
    board.put_piece_on_field_xy(white_rook, (0, 0), 0);
    board.put_piece_on_field_xy(white_rook, (7, 0), 0);

    let steps = board.compute_steps();

    let castles: Vec<StepKind> = steps
        .iter()
        .cloned()
        .filter(|step| matches!(step, StepKind::Castle { .. }))
        .collect();
    println!("Steps: {:?}", steps);
    assert_eq!(castles.len(), 2);
}

#[test]
pub fn checkmove() -> Result<(), String> {
    let mut game = Game::new();
    let _ = game.apply_input("e2e4")?;
    let _ = game.apply_input("a7a6")?;
    let _ = game.apply_input("d1h5")?;
    let _ = game.apply_input("a6a5")?;
    let _ = game.apply_input("f1c4")?;
    let _ = game.apply_input("a5a4")?;
    let res = game.apply_input("h5f7")?;
    assert_eq!(res, GameState::Won(PieceColor::Black));
    Ok(())
}

#[test]
pub fn simple_take() -> Result<(), String> {
    let descr: String = r#"
    a1kw
    a2rb
    b1rb
    "#
    .to_string();
    let board = BoardCreation::from_description(descr)?;
    println!("{}", board.to_pretty_string());
    let possible_moves = board.compute_steps();
    assert_eq!(possible_moves.len(), 2);
    Ok(())
}
