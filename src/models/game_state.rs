use super::piece::PieceColor;

#[derive(PartialEq, Debug)]
pub enum GameState {
    Ongoing,
    Stalemate,
    Won(PieceColor),
}
