use ratatui::{
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct Header(pub i32);

impl Widget for Header {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(format!("Terminal Chess :), Counter: {}", self.0))
            .centered()
            .block(Block::bordered().border_set(border::PLAIN))
            .render(area, buf);
    }
}
