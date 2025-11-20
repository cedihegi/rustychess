use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use log::error;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};
use std::{io, time::Duration};

use crate::models::{board::Board, game_state::GameState};

use super::{
    state::view_state::ViewState,
    widgets::{BoardWidget, ClickDirection, Header},
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    board_widget: BoardWidget,
    state: ViewState,
    board: Board,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.board = Board::standard_board();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.handle_state();
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout_outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(5), // Header
                    Constraint::Fill(1),
                    Constraint::Length(2), // Footer
                ],
            )
            .split(frame.area());

        let layout_board = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Fill(1),
                    Constraint::Length(10),
                ],
            )
            .split(layout_outer[1]);
        frame.render_widget(Header, layout_outer[0]);

        match &mut self.state {
            ViewState::Startup => {
                let start_paragraph = Paragraph::new("To start a new game, press 's'");
                frame.render_widget(start_paragraph, layout_board[1]);
            }
            ViewState::InGame {
                state: GameState::Ongoing,
                ..
            } => {
                frame.render_stateful_widget(&self.board_widget, layout_board[1], &mut self.state);
            }
            ViewState::InGame { state, .. } => {
                let outcome_paragraph = Paragraph::new(state.message());
                frame.render_widget(outcome_paragraph, layout_board[1]);
            }
        }
    }
    pub fn handle_events(&mut self) -> io::Result<()> {
        // performance optimization, after rendering, wait for an event
        // but then, consume all the events you can
        poll(Duration::MAX)?;
        loop {
            if poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event, self.state.is_lobby_mode())
                    }
                    Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
                    _ => {}
                }
            } else {
                return Ok(());
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, lobby_mode: bool) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') if lobby_mode => self.start_game(),
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) {
        match mouse_event.kind {
            event::MouseEventKind::Down(event::MouseButton::Left) => {
                self.board_widget
                    .click(self.board_widget.mouse_position, ClickDirection::Down);
            }
            event::MouseEventKind::Up(event::MouseButton::Left) => {
                self.board_widget
                    .click(self.board_widget.mouse_position, ClickDirection::Up);
            }
            event::MouseEventKind::Down(event::MouseButton::Right) => {
                self.board_widget.reset_clicks();
            }
            event::MouseEventKind::Moved => {
                self.board_widget.mouse_position = (mouse_event.column, mouse_event.row)
            }
            _ => {} // ignoring things like scroll events
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn start_game(&mut self) {
        self.state
            .start()
            .expect("Just before calling, we check that 'is_lobby_mode'. This should never fail");
    }

    fn handle_state(&mut self) {
        if let ViewState::InGame {
            game,
            next_step: next_step @ Some(_),
            state,
        } = &mut self.state
        {
            let res = game.apply_stepkind(next_step.take().unwrap());
            match res {
                Ok(new_state) => *state = new_state,
                Err(_) => error!("Invalid move!"),
            }
            self.board_widget.reset_clicks();
        }
    }
}
