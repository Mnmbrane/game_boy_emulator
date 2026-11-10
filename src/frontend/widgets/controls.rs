use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, area: Rect, status: &str) {
    let controls = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(" q / Esc ", Style::default().fg(Color::LightGreen)),
            Span::raw("Quit"),
        ]),
        Line::from(vec![
            Span::styled(" i / f   ", Style::default().fg(Color::Yellow)),
            Span::raw("Instruction / frame step (pending core support)"),
        ]),
        Line::from(status.to_owned()),
    ])
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(" Controls & status ")
            .borders(Borders::ALL),
    );
    frame.render_widget(controls, area);
}
