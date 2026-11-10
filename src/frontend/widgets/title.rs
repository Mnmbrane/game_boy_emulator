use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(" GAME BOY  /  TERMINAL ")
        .style(Style::default().fg(Color::LightGreen))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}
