use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let viewport =
        Paragraph::new("LCD output will appear here once the PPU exposes a framebuffer.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray))
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" Display ").borders(Borders::ALL));
    frame.render_widget(viewport, area);
}
