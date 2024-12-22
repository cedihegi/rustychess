use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal, Frame,
};
use std::{io, time::Duration};

use super::widgets::{Board, ClickDirection, Header};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    counter: i32,
    board: Board,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if !event::poll(Duration::from_millis(250))? {
                continue;
            }
            self.handle_events()?
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let layout_outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(5), // Header
                    Constraint::Fill(1),
                    Constraint::Length(2), // Footer
                ]
                .into_iter(),
            )
            .split(frame.area());

        let layout_board = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Fill(1),
                    Constraint::Length(10),
                ]
                .into_iter(),
            )
            .split(layout_outer[1]);
        frame.render_widget(Header(self.counter), layout_outer[0]);
        frame.render_widget(&self.board, layout_board[1]);
    }
    pub fn handle_events(&mut self) -> io::Result<()> {
        // performance optimization, after rendering, wait for an event
        // but then, consume all the events you can
        poll(Duration::MAX)?;
        loop {
            if poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event)
                    }
                    Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
                    _ => {}
                }
            } else {
                return Ok(());
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.counter -= 1,
            KeyCode::Right => self.counter += 1,
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) {
        match mouse_event.kind {
            event::MouseEventKind::Down(event::MouseButton::Left) => {
                self.board
                    .click(self.board.mouse_position, ClickDirection::Down);
            }
            event::MouseEventKind::Up(event::MouseButton::Left) => {
                if let Some(_step) = self
                    .board
                    .click(self.board.mouse_position, ClickDirection::Up)
                {
                    self.counter += 1;
                }
            }
            event::MouseEventKind::Down(event::MouseButton::Right) => {
                self.board.reset_clicks();
            }
            event::MouseEventKind::Moved => {
                self.board.mouse_position = (mouse_event.column, mouse_event.row)
            }
            _ => {} // ignoring things like scroll events
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
