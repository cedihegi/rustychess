use log::debug;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    symbols::border::{self, Set},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::{
    driver::game::Game,
    gui::state::view_state::ViewState,
    models::step::{Step, StepKind},
};

use super::board_field::BoardField;

#[derive(Debug, Default)]
pub struct BoardWidget {
    pub mouse_position: (u16, u16),
    pub click_state_1: ClickState,
    pub click_state_2: ClickState,
}

const COORDINATE_WIDTH: u16 = 4;
const COORDINATE_HEIGHT: u16 = 2;
pub const CELL_WIDTH: u16 = 9;
pub const CELL_HEIGHT: u16 = 5;
const BORDER_WIDTH: u16 = 1;
const BORDER_HEIGHT: u16 = 1;

const BORDER_SET: Set = Set {
    top_left: border::QUADRANT_BOTTOM_HALF,
    horizontal_top: border::QUADRANT_BOTTOM_HALF,
    top_right: border::QUADRANT_BOTTOM_HALF,
    bottom_left: border::QUADRANT_TOP_HALF,
    horizontal_bottom: border::QUADRANT_TOP_HALF,
    bottom_right: border::QUADRANT_TOP_HALF,
    ..border::FULL
};

impl StatefulWidget for &BoardWidget {
    type State = ViewState;
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut ViewState) {
        // this must match, otherwise this render method is not called:
        let ViewState::InGame {
            game: Game { board },
            next_step,
            ..
        } = state
        else {
            unreachable!()
        };

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

        Block::bordered()
            .border_set(BORDER_SET)
            .border_style(Style::default().fg(Color::Gray))
            .render(border_square, buf);

        Block::bordered()
            .border_set(border::PLAIN)
            .render(outer_square, buf);

        let (board_fields, selected_1, selected_2) = self.board_fields(&fields_square);
        if let (Some(pos1), Some(pos2)) = (selected_1, selected_2) {
            let stepkind = StepKind::GoTo(Step::new(pos1, pos2));
            *next_step = Some(stepkind);
        }
        for field in board_fields.iter() {
            let (x, y) = field.chess_coordinates;
            let mut field_content = board
                .field_at_xy(x, y)
                .expect("Error indexing board fields")
                .clone();
            field.render(fields_square, buf, &mut field_content);
        }
    }
}

impl BoardWidget {
    fn board_fields(
        &self,
        area: &Rect,
    ) -> (
        Vec<BoardField>,
        Option<(usize, usize)>,
        Option<(usize, usize)>,
    ) {
        let mut res_fields = vec![];
        let selected1 = self.click_state_1.selected.unwrap_or((u16::MAX, u16::MAX));
        let selected2 = self.click_state_2.selected.unwrap_or((u16::MAX, u16::MAX));
        let mut selected1_chess_cords = None;
        let mut selected2_chess_cords = None;
        for i in 0..8 {
            for j in 0..8 {
                // we draw top to bottom, but the chess board is indexed bottom to top
                let chess_i = i as usize;
                let chess_j = 7 - j as usize;
                let x = CELL_WIDTH * i;
                let y = CELL_HEIGHT * j;
                let color = if (i + j) % 2 == 0 {
                    Color::DarkGray
                } else {
                    Color::Black
                };
                let left = area.x + x;
                let right = left + CELL_WIDTH;
                let top = area.y + y;
                let bottom = top + CELL_HEIGHT;
                let contains = |pos: (u16, u16)| -> bool {
                    let contains_vertical = left <= pos.0 && right > pos.0;
                    let contains_horiz = top <= pos.1 && bottom > pos.1;
                    contains_vertical && contains_horiz
                };
                let contains_mouse = contains(self.mouse_position);
                let contains_first_click = contains(selected1);
                let contains_second_click = contains(selected2);
                if contains_first_click {
                    selected1_chess_cords = Some((chess_i, chess_j))
                }
                if contains_second_click {
                    selected2_chess_cords = Some((chess_i, chess_j))
                }

                let cell = BoardField::new(
                    x,
                    y,
                    CELL_WIDTH,
                    CELL_HEIGHT,
                    (chess_i, chess_j),
                    color,
                    contains_mouse,
                    contains_first_click,
                );
                res_fields.push(cell);
            }
        }
        (res_fields, selected1_chess_cords, selected2_chess_cords)
    }

    /// a somewhat ugly function ot handle the state when mouse is up/down
    pub fn click(&mut self, position: (u16, u16), click_direction: ClickDirection) -> Option<Step> {
        debug!(
            "registered click for position: {:?}, {:?}",
            position, click_direction
        );
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
        debug!("reseting all clicks");
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

#[derive(Default, Debug)]
pub enum ClickDirection {
    Up,
    #[default]
    Down,
}
