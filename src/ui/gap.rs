use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn draw_repeat<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = app.player;
    // let rep = player.repetition;
    // let s = format!("x{:}", rep);
    let text = Paragraph::new(s)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Gap")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().add_modifier(Modifier::SLOW_BLINK));
}
