use ratatui::{
    layout::Rect,
    style::{Color, Style, Styled, Stylize},
    symbols::border::{self, Set},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::models::{field_content::FieldContent, piece::PieceColor};

use super::board::{CELL_HEIGHT, CELL_WIDTH};

#[derive(Debug, Default)]
pub struct BoardField {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    pub chess_coordinates: (usize, usize),
    color: Color,
    contains_mouse: bool,
    was_clicked: bool,
}

pub const BORDER_OUTER: Set = Set {
    top_left: border::QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
    top_right: border::QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT,
    bottom_left: border::QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT,
    bottom_right: border::QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
    vertical_left: border::QUADRANT_LEFT_HALF,
    vertical_right: border::QUADRANT_RIGHT_HALF,
    horizontal_top: border::QUADRANT_TOP_HALF,
    horizontal_bottom: border::QUADRANT_BOTTOM_HALF,
};

impl StatefulWidget for &BoardField {
    type State = FieldContent;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        field: &mut FieldContent,
    ) {
        let rect = Rect::new(area.x + self.x, area.y + self.y, self.width, self.height);
        let mut block = Block::default().bg(self.color);
        if self.was_clicked {
            block = block
                .border_set(BORDER_OUTER)
                .magenta()
                .borders(Borders::ALL);
        } else if self.contains_mouse {
            block = block.border_set(BORDER_OUTER).cyan().borders(Borders::ALL);
        }

        block.render(rect, buf);

        let content_opt = BoardField::content_paragraph(field);
        if let Some(content_par) = content_opt {
            let centered_rect = Rect::new(
                area.x + self.x + CELL_WIDTH / 2,
                area.y + self.y + CELL_HEIGHT / 2,
                1,
                1,
            );
            content_par.render(centered_rect, buf);
        }
    }
}

impl BoardField {
    pub fn new(
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        chess_coordinates: (usize, usize),
        color: Color,
        contains_mouse: bool,
        was_clicked: bool,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            chess_coordinates,
            color,
            contains_mouse,
            was_clicked,
        }
    }

    pub fn content_paragraph(content: &FieldContent) -> Option<Paragraph> {
        if let Some(color) = content.get_color() {
            let tui_color = match color {
                PieceColor::White => Color::Cyan,
                PieceColor::Black => Color::Red,
            };
            let symbol = content.to_uncolored_piece_string();
            Some(
                Paragraph::new(symbol)
                    .set_style(Style::default().fg(tui_color))
                    .bold(),
            )
        } else {
            None
        }
    }
}
