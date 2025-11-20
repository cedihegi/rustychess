use crate::{
    logic::move_computer::StepComputer,
    models::{board::Board, game_state::GameState, step::StepKind},
};

#[derive(Debug, Default)]
pub struct Game {
    pub board: Board,
}
impl Game {
    pub fn new() -> Self {
        let board = Board::standard_board();
        Self { board }
    }

    pub fn apply_input(&mut self, input: &str) -> Result<GameState, String> {
        // 1. decode:
        let color = self.board.turn_color();
        let step_kind = StepKind::decode(input, color)?;
        let eval = self.board.evaluate_basic();

        // 2. check step is legal
        if eval.possible_moves.contains(&step_kind) {
            self.board.apply_step_kind(&step_kind)?
        } else {
            return Err(format!(
                "illegal move! {:?}, possible moves {:?}",
                step_kind, eval.possible_moves
            ));
        }

        let new_eval = self.board.evaluate_basic();

        if new_eval.has_stalemate {
            Ok(GameState::Stalemate)
        } else if new_eval.has_checkmate {
            Ok(GameState::Won(color))
        } else {
            Ok(GameState::Ongoing)
        }
    }

    pub fn apply_stepkind(&mut self, step_kind: StepKind) -> Result<GameState, String> {
        // 1. evaluate position beforehand:
        let color = self.board.turn_color();
        let eval = self.board.evaluate_basic();

        if eval.possible_moves.contains(&step_kind) {
            self.board.apply_step_kind(&step_kind)?;
        }
        let new_eval = self.board.evaluate_basic();
        if new_eval.has_stalemate {
            Ok(GameState::Stalemate)
        } else if new_eval.has_checkmate {
            Ok(GameState::Won(color))
        } else {
            Ok(GameState::Ongoing)
        }
    }
}
