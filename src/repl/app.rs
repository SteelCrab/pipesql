//! App state and main loop
///  ┌─ PipeSQL ────────────────────┐
///  │                              │
///  │                              │
///  │                              │
///  └──────────────────────────────┘
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io;

use super::{event::handle_key, ui};

pub struct App {
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        let runner = self.run_loop(&mut terminal);
        ratatui::restore();
        runner
    }

    fn run_loop(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            terminal.draw(ui::render)?;
            if let Event::Key(key) = event::read()? {
                handle_key(self, key);
            }
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
