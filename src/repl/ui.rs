use ratatui::{
    Frame,
    widgets::{Block, Borders},
};

pub fn render(frame: &mut Frame) {
    let block = Block::default().title(" PipeSQL ").borders(Borders::ALL);
    frame.render_widget(block, frame.area());
}
