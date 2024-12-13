use crate::models::step::StepKind;

pub struct BasicEvaluation {
    pub has_check: bool,
    pub has_checkmate: bool,
    pub has_stalemate: bool,
    pub possible_moves: Vec<StepKind>,
}
