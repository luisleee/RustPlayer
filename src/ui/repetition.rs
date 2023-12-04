use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_repeat<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;
    let rep = player.repetition;
    let s = format!("(g)◄ x{:} ►(h)", rep);
    let text = Paragraph::new(s)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Repeat")
                .title_alignment(Alignment::Center),
        );
    frame.render_widget(text, area);
}
