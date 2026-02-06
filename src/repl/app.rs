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
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        let reuslt = self.run_loop(&mut terminal);
        ratatui::restore();
        reuslt
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_app_new() {
        let app_new = App::new();
        assert!(app_new.is_running());
    }
    #[test]
    fn test_repl_app_default() {
        let default = App::default();
        assert!(default.is_running());
    }
    #[test]
    fn test_repl_app_quit() {
        let mut app = App::new();
        app.quit();
        assert!(!app.running);
    }
}
