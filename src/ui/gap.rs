use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_gap<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;
    let gap = player.gap;
    let s = format!("(g)◄ {:.1} s ►(h)", gap);
    let text = Paragraph::new(s)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Gap")
                .title_alignment(Alignment::Center),
        );
    frame.render_widget(text, area);
}
