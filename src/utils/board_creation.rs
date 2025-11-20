use crate::models::{
    board::Board,
    location::Location,
    piece::{ColoredPiece, PieceColor, PieceKind},
};

pub struct BoardCreation;
impl BoardCreation {
    pub fn from_description(descr: String) -> Result<Board, String> {
        let mut board = Board::new(8, 8);
        let lines = descr.lines();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let chars: Vec<char> = line.chars().collect();
            let location = Location::decode(&line[0..2])?;
            let piece_kind = PieceKind::decode(chars[2])?;
            let piece_color = PieceColor::decode(chars[3])?;

            let piece = ColoredPiece {
                kind: piece_kind,
                color: piece_color,
            };
            board.put_piece_on_location(piece, &location, 0);
        }
        Ok(board)
    }
}
