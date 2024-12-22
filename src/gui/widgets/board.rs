use std::str::FromStr;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Widget},
};

use crate::models::step::{Step, StepKind};

use super::board_field::BoardField;

#[derive(Debug, Default)]
pub struct Board {
    pub mouse_position: (u16, u16),
    pub click_state_1: ClickState,
    pub click_state_2: ClickState,
}

const COORDINATE_WIDTH: u16 = 4;
const COORDINATE_HEIGHT: u16 = 2;
const CELL_WIDTH: u16 = 8;
const CELL_HEIGHT: u16 = 4;
const BORDER_WIDTH: u16 = 1;
const BORDER_HEIGHT: u16 = 1;

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let full_width = CELL_WIDTH * 8 + COORDINATE_WIDTH * 2 + 2 * BORDER_WIDTH;
        let full_height = CELL_HEIGHT * 8 + COORDINATE_HEIGHT * 2 + 2 * BORDER_HEIGHT;
        let outer_square = Rect::new(area.x, area.y, full_width, full_height);
        let border_square = Rect::new(
            outer_square.x + COORDINATE_WIDTH,
            outer_square.y + COORDINATE_HEIGHT,
            full_width - 2 * COORDINATE_WIDTH,
            full_height - 2 * COORDINATE_HEIGHT,
        );
        let fields_square = Rect::new(
            border_square.x + BORDER_WIDTH,
            border_square.y + BORDER_HEIGHT,
            8 * CELL_WIDTH,
            8 * CELL_HEIGHT,
        );
        let mut border_set = border::FULL;
        border_set.top_left = border::QUADRANT_BOTTOM_HALF;
        border_set.horizontal_top = border::QUADRANT_BOTTOM_HALF;
        border_set.top_right = border::QUADRANT_BOTTOM_HALF;
        border_set.bottom_left = border::QUADRANT_TOP_HALF;
        border_set.horizontal_bottom = border::QUADRANT_TOP_HALF;
        border_set.bottom_right = border::QUADRANT_TOP_HALF;

        Block::bordered()
            .border_set(border_set)
            .border_style(Style::default().fg(Color::from_str("#000000").expect("invalid color")))
            .render(border_square, buf);

        Block::bordered()
            .border_set(border::PLAIN)
            .render(outer_square, buf);
        let board_fields = self.board_fields(&fields_square);
        for field in board_fields.into_iter() {
            field.render(fields_square, buf);
        }
    }
}

impl Board {
    fn board_fields(&self, area: &Rect) -> Vec<BoardField> {
        let mut res_fields = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let x = CELL_WIDTH * i;
                let y = CELL_HEIGHT * j;
                let color = if (i + j) % 2 == 0 {
                    Color::White
                } else {
                    Color::Black
                };
                let contains_vertical = area.x + x <= self.mouse_position.0
                    && area.x + x + CELL_WIDTH > self.mouse_position.0;
                let contains_horiz = area.y + y <= self.mouse_position.1
                    && area.y + y + CELL_HEIGHT > self.mouse_position.1;
                let cell = BoardField::new(
                    x,
                    y,
                    CELL_WIDTH,
                    CELL_HEIGHT,
                    (i, j),
                    color,
                    contains_vertical && contains_horiz,
                );
                res_fields.push(cell);
            }
        }
        res_fields
    }

    /// a somewhat ugly function ot handle the state when mouse is up/down
    pub fn click(&mut self, position: (u16, u16), click_direction: ClickDirection) -> Option<Step> {
        if let Some(first_click) = self.click_state_1.selected {
            let second_click_opt = self.click_state_2.update(position, click_direction);
            if let Some(second_click) = second_click_opt {
                Some(Step::new(
                    (first_click.0 as usize, first_click.1 as usize),
                    (second_click.0 as usize, second_click.1 as usize),
                ))
            } else {
                None
            }
        } else {
            self.click_state_1.update(position, click_direction);
            None
        }
    }

    pub fn reset_clicks(&mut self) {
        self.click_state_1.reset();
        self.click_state_2.reset();
    }
}

#[derive(Default, Debug)]
pub struct ClickState {
    pub pressed_down_on: Option<(u16, u16)>,
    pub selected: Option<(u16, u16)>,
}

impl ClickState {
    fn update(
        &mut self,
        position: (u16, u16),
        click_direction: ClickDirection,
    ) -> Option<(u16, u16)> {
        if let Some(down_click_pos) = self.pressed_down_on {
            // check second one is up
            if let ClickDirection::Down = click_direction {
                self.reset();
                return None;
            }
            if position == down_click_pos {
                self.selected = Some(position);
                Some(position)
            } else {
                self.reset();
                None
            }
        } else {
            if let ClickDirection::Up = click_direction {
                self.reset()
            } else {
                self.pressed_down_on = Some(position);
            }
            None
        }
    }

    fn reset(&mut self) {
        self.pressed_down_on = None;
        self.selected = None;
    }
}

pub enum ClickDirection {
    Up,
    Down,
}
