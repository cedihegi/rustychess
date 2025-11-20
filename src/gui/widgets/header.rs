use ratatui::{
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct Header;

impl Widget for Header {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new("Terminal Chess :)")
            .centered()
            .block(Block::bordered().border_set(border::PLAIN))
            .render(area, buf);
    }
}
