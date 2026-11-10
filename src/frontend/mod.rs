//! Terminal presentation and input for the emulator.

mod widgets;

use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
};

use crate::core::GameBoy;

/// State owned by the terminal frontend.
pub struct Frontend {
    active: bool,
    status: &'static str,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            active: true,
            status: "Ready. The emulation core is still under construction.",
        }
    }

    pub fn run(&mut self, _game_boy: &mut GameBoy) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        let result = (|| {
            while self.active {
                terminal.draw(|frame| self.draw(frame))?;
                self.handle_event(event::read()?);
            }
            Ok(())
        })();
        ratatui::restore();
        result
    }

    fn draw(&self, frame: &mut Frame) {
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(6),
            ])
            .split(frame.area());

        widgets::title::render(frame, sections[0]);
        widgets::viewport::render(frame, sections[1]);
        widgets::controls::render(frame, sections[2], self.status);
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.active = false,
                KeyCode::Char('i') | KeyCode::Char('f') => {
                    self.status = "Stepping is waiting for the core's instance methods."
                }
                _ => {}
            }
        }
    }
}
