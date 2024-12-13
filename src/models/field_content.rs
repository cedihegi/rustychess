use super::piece::ColoredPiece;

#[derive(Debug, Clone, Copy)]
pub enum FieldContent {
    // usize limits the number of possible turns in a game ^^
    Occupied { piece: ColoredPiece, turn: usize },
    Empty,
}

impl FieldContent {
    pub fn get_content(&self) -> Option<(ColoredPiece, usize)> {
        if let Self::Occupied { piece, turn } = *self {
            Some((piece, turn))
        } else {
            None
        }
    }
    pub fn to_pretty_string(&self) -> String {
        match self {
            FieldContent::Occupied { piece, .. } => piece.to_colored_symbol(),
            FieldContent::Empty => " ".to_string(),
        }
    }
}
