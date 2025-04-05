use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph, Widget},
};

const FOCUSED_COLOR: Color = Color::Green;
const UNFOCUSED_COLOR: Color = Color::White;

enum FocusArea {
    Area1,
    Area2,
    Area3,
    Area4,
    Area5,
}

// Application Struct
pub struct App {
    focus_area: FocusArea,
    quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            focus_area: FocusArea::Area1,
            quit: false,
        }
    }
}

// Main Logic of the application
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit = true,
            KeyCode::Char('1') => self.focus_area = FocusArea::Area1,
            KeyCode::Char('2') => self.focus_area = FocusArea::Area2,
            KeyCode::Char('3') => self.focus_area = FocusArea::Area3,
            KeyCode::Char('4') => self.focus_area = FocusArea::Area4,
            KeyCode::Char('5') => self.focus_area = FocusArea::Area5,
            _ => {}
        }
    }
}

// Widget implementation for the App
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]);
        let [left_area, right_area] = main_layout.areas(area);

        let left_area_layout = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ]);
        let [area1_area, area2_area, area3_area] = left_area_layout.areas(left_area);

        let right_area_layout =
            Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
        let [area4_area, area5_area] = right_area_layout.areas(right_area);

        // Area 1
        Paragraph::new("Area 1")
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .style(Style::default().fg(match self.focus_area {
                        FocusArea::Area1 => FOCUSED_COLOR,
                        _ => UNFOCUSED_COLOR,
                    }))
                    .title("[1] Area 1"),
            )
            .render(area1_area, buf);

        // Area 2
        Paragraph::new("Area 2")
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .style(Style::default().fg(match self.focus_area {
                        FocusArea::Area2 => FOCUSED_COLOR,
                        _ => UNFOCUSED_COLOR,
                    }))
                    .title("[2] Area 2"),
            )
            .render(area2_area, buf);

        // Area 3
        Paragraph::new("Area 3")
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .style(Style::default().fg(match self.focus_area {
                        FocusArea::Area3 => FOCUSED_COLOR,
                        _ => UNFOCUSED_COLOR,
                    }))
                    .title("[3] Area 3"),
            )
            .render(area3_area, buf);

        // Area 4
        Paragraph::new("Area 4")
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .style(Style::default().fg(match self.focus_area {
                        FocusArea::Area4 => FOCUSED_COLOR,
                        _ => UNFOCUSED_COLOR,
                    }))
                    .title("[4] Area 4"),
            )
            .render(area4_area, buf);

        // Area 5
        Paragraph::new("Area 5")
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .style(Style::default().fg(match self.focus_area {
                        FocusArea::Area5 => FOCUSED_COLOR,
                        _ => UNFOCUSED_COLOR,
                    }))
                    .title("[5] Area 5"),
            )
            .render(area5_area, buf);
    }
}
