use ratatui::{
    layout::Rect,
    style::{Color, Stylize},
    symbols::border::{self, Set},
    widgets::{Block, Borders, Widget},
};

pub struct BoardField {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    _coordinates: (u16, u16),
    color: Color,
    contains_mouse: bool,
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

impl Widget for BoardField {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let rect = Rect::new(area.x + self.x, area.y + self.y, self.width, self.height);
        let mut block = Block::default().bg(self.color);
        if self.contains_mouse {
            block = block.border_set(BORDER_OUTER).cyan().borders(Borders::ALL)
        }

        block.render(rect, buf)
    }
}

impl BoardField {
    pub fn new(
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        coordinates: (u16, u16),
        color: Color,
        contains_mouse: bool,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            _coordinates: coordinates,
            color,
            contains_mouse,
        }
    }
}
