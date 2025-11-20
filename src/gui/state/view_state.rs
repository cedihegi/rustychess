use crate::{
    driver::game::Game,
    models::{game_state::GameState, step::StepKind},
};

#[derive(Debug, Default)]
pub enum ViewState {
    #[default]
    Startup,
    InGame {
        state: GameState,
        game: Game,
        next_step: Option<StepKind>,
    },
}

impl ViewState {
    pub fn is_lobby_mode(&self) -> bool {
        matches!(
            self,
            Self::InGame {
                state: GameState::Stalemate | GameState::Won(_),
                ..
            } | Self::Startup
        )
    }

    pub fn start(&mut self) -> Result<(), String> {
        if !self.is_lobby_mode() {
            return Err("End this game first".to_string());
        }
        *self = Self::InGame {
            state: GameState::Ongoing,
            game: Game::new(),
            next_step: None,
        };
        Ok(())
    }
}
