use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Widget},
};

pub struct CoordinateField {
    content: String,
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    align_vertical: bool,
}

impl Widget for CoordinateField {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let rect = Rect::new(area.x + self.x, area.y + self.y, self.width, self.height);
        let paragraph = Paragraph::new(self.content).alignment(Alignment::Center);

        let target_area = if self.align_vertical {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Fill(1),
                        Constraint::Length(1),
                        Constraint::Fill(1),
                    ]
                    .into_iter(),
                )
                .split(rect);
            layout[1]
        } else {
            rect
        };
        paragraph.render(target_area, buf);
    }
}

impl CoordinateField {
    pub fn new(
        content: impl Into<String>,
        x: u16,
        y: u16,
        height: u16,
        width: u16,
        align_vertical: bool,
    ) -> Self {
        Self {
            content: content.into(),
            x,
            y,
            height,
            width,
            align_vertical,
        }
    }
}
