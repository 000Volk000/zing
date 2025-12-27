use std::{
    env,
    fs::{self, File},
    io::{self, Error, Write},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::Style,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use tui_big_text::{BigText, PixelSize};

#[derive(Debug, Default)]
pub struct App {
    step: u16,
    step_vector: Vec<u16>,
    fich_name: String,
    failed_save_flag: bool,
    saved_flag: bool,
    exit: bool,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut terminal = ratatui::init();
    let app_result = App::new(
        args.get(1)
            .expect("Please put the file route on the first argument\nEx: ./zing pattern.txt")
            .to_string(),
    )
    .run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    pub fn new(fich_name: String) -> Self {
        let fich: Vec<String> = fs::read_to_string(fich_name.clone())
            .expect("Couldn't find the file")
            .trim()
            .lines()
            .map(|line| line.to_string())
            .collect();
        Self {
            exit: false,
            failed_save_flag: false,
            saved_flag: false,
            step_vector: fich
                .first()
                .expect("No first line on the file")
                .split(",")
                .map(|n| {
                    n.trim()
                        .parse::<u16>()
                        .expect("Bad format on file step sequence")
                })
                .collect(),
            step: fich
                .get(1)
                .map(|s| s.as_str())
                .unwrap_or("0")
                .parse::<u16>()
                .expect("Something wrong setting the last checkpoint"),
            fich_name,
        }
    }

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
        self.saved_flag = false;
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char('q') => self.exit(),
            pressed_key => {
                self.failed_save_flag = false;
                match pressed_key {
                    KeyCode::Backspace => self.decrement_step(),
                    KeyCode::Left => self.decrement_step(),
                    KeyCode::Char(' ') => self.increment_step(),
                    KeyCode::Right => self.increment_step(),
                    KeyCode::Char('s') => self.save(),
                    _ => {}
                }
            }
        }
    }

    fn exit(&mut self) {
        if self.failed_save_flag {
            self.exit = true;
        } else {
            match self.save_step_to_file() {
                Ok(_) => self.exit = true,
                _ => {
                    self.failed_save_flag = true;
                }
            }
        }
    }

    fn increment_step(&mut self) {
        if self.step < (self.step_vector.len() - 1) as u16 {
            self.step = self.step.saturating_add(1);
        }
    }

    fn decrement_step(&mut self) {
        self.step = self.step.saturating_sub(1);
    }

    fn save_step_to_file(&self) -> Result<(), Error> {
        let mut fich = File::create(format!("{}.tmp", self.fich_name.clone()))?;

        fich.write_all(
            self.step_vector
                .clone()
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .as_bytes(),
        )?;
        fich.write_all("\n".to_string().as_bytes())?;
        fich.write_all(self.step.to_string().as_bytes())?;

        fs::rename(
            format!("{}.tmp", self.fich_name.clone()),
            self.fich_name.clone(),
        )
    }

    fn save(&mut self) {
        match self.save_step_to_file() {
            Ok(_) => self.saved_flag = true,
            _ => self.failed_save_flag = true,
        }
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
            " | Save ".into(),
            "<S>".blue().bold(),
            " | Quit ".into(),
            "<Q> / <Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);
        block.clone().render(area, buf);

        let inner_block_area = block.inner(area);
        let vertical_layout = Layout::vertical([
            Constraint::Min(1),
            Constraint::Percentage(100),
            Constraint::Min(3),
        ]);
        let [current_step_area, step_area, lower_notification_area] =
            vertical_layout.areas(inner_block_area);

        let current_step = Text::from(vec![Line::from(vec![
            "Step: ".into(),
            self.step.to_string().yellow(),
        ])]);

        Paragraph::new(current_step)
            .centered()
            .render(current_step_area, buf);

        let [centered_step_area, _, centered_below_area] = Layout::vertical([
            Constraint::Length(7),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .flex(Flex::Center)
        .areas(step_area);

        let step = BigText::builder()
            .centered()
            .pixel_size(PixelSize::Full)
            .style(Style::new().white())
            .lines(vec![
                self.step_vector
                    .get(self.step as usize)
                    .expect("Critical error getting the step")
                    .to_string()
                    .into(),
            ])
            .build();
        step.render(centered_step_area, buf);

        let [last_step_area, next_step_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(centered_below_area);
        let last_text = BigText::builder()
            .centered()
            .pixel_size(PixelSize::Octant)
            .style(Style::new().yellow())
            .lines(vec![
                ("Last:".white()
                    + (self
                        .step_vector
                        .get(self.step.saturating_sub(1) as usize)
                        .expect("Critical error getting the last step")
                        .to_string()
                        .yellow())),
            ])
            .build();
        last_text.render(last_step_area, buf);

        let next_text = BigText::builder()
            .centered()
            .pixel_size(PixelSize::Octant)
            .style(Style::new().yellow())
            .lines(vec![
                ("Next:".white()
                    + (self
                        .step_vector
                        .get(self.step.saturating_add(1) as usize)
                        .or(self.step_vector.get(self.step as usize))
                        .expect("Critical error getting the step")
                        .to_string()
                        .yellow())),
            ])
            .build();
        next_text.render(next_step_area, buf);

        if self.failed_save_flag {
            let failed_save_notification = Text::from(vec![
                Line::from("Something wrong happened saving the current position".bold()),
                Line::from(vec![
                    "Press ".bold(),
                    "<Q>".red().bold(),
                    " to quit without saving".bold(),
                ]),
            ]);

            Paragraph::new(failed_save_notification)
                .centered()
                .render(lower_notification_area, buf);
        }

        if self.saved_flag {
            let failed_save_notification = Text::from(vec![Line::from("Saved!".bold())]);

            Paragraph::new(failed_save_notification)
                .centered()
                .render(lower_notification_area, buf);
        }
    }
}
