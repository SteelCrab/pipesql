use super::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Esc {
        app.quit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent};
    #[test]
    fn test_handle_key_exit() {
        let mut app = App::new();
        handle_key(&mut app, KeyEvent::from(KeyCode::Esc));
        assert!(!app.is_running());
    }
    #[test]
    fn test_handle_key_others() {
        let mut app = App::new();
        handle_key(&mut app, KeyEvent::from(KeyCode::Char('c')));
        assert!(app.is_running());
    }
}
