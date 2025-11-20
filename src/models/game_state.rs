use super::piece::PieceColor;

#[derive(PartialEq, Debug)]
pub enum GameState {
    Ongoing,
    Stalemate,
    Won(PieceColor),
}

impl GameState {
    pub fn message(&self) -> String {
        match self {
            GameState::Ongoing => "Game is still ongoing".to_string(),
            GameState::Stalemate => "Game ended in a draw because of stalemate".to_string(),
            GameState::Won(piece_color) => format!("{:?} has won the game", piece_color),
        }
    }
}
