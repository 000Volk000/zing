use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    step: u16,
    exit: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
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
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char('q') => self.exit(),
            KeyCode::Backspace => self.decrement_step(),
            KeyCode::Left => self.decrement_step(),
            KeyCode::Char(' ') => self.increment_step(),
            KeyCode::Right => self.increment_step(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_step(&mut self) {
        self.step = self.step.saturating_add(1);
    }

    fn decrement_step(&mut self) {
        self.step = self.step.saturating_sub(1);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Zing Is Not Generating ".bold());
        let instructions = Line::from(vec![
            " Previous step ".into(),
            "<Left> / <Backspace>".blue().bold(),
            " | Next step ".into(),
            "<Right> / <Space>".blue().bold(),
            " | Quit ".into(),
            "<Q> / <Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        let counter_text = Text::from(vec![Line::from(vec![
            "Step: ".into(),
            self.step.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 92, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "╭───────────────────────────────── Zing Is Not Generating ─────────────────────────────────╮",
            "│                                          Step: 0                                         │",
            "│                                                                                          │",
            "╰── Previous step <Left> / <Backspace> | Next step <Right> / <Space> | Quit <Q> / <Esc> ───╯",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(34, 0, 24, 1), title_style);
        expected.set_style(Rect::new(49, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(18, 3, 20, 1), key_style);
        expected.set_style(Rect::new(51, 3, 17, 1), key_style);
        expected.set_style(Rect::new(76, 3, 12, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();

        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.step, 1);
        app.handle_key_event(KeyCode::Char(' ').into());
        assert_eq!(app.step, 2);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.step, 1);
        app.handle_key_event(KeyCode::Backspace.into());
        assert_eq!(app.step, 0);
        app.handle_key_event(KeyCode::Backspace.into());
        assert_eq!(app.step, 0);

        app.step = u16::MAX;
        app.handle_key_event(KeyCode::Char(' ').into());
        assert_eq!(app.step, u16::MAX);

        let mut app = App::default();

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        let mut app = App::default();

        app.handle_key_event(KeyCode::Esc.into());
        assert!(app.exit);

        Ok(())
    }
}
