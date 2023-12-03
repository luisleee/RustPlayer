use crossterm::event::KeyCode;

use crate::{app::App, media::player::Player};

pub fn handle_repetition(app: &mut App, code: KeyCode) -> bool {
    let player = &mut app.player;
    match code {
        KeyCode::Char('1') | KeyCode::Char('!') => {
            player.adjust_repetition(false);
            return true;
        }
        KeyCode::Char('2') | KeyCode::Char('@') => {
            player.adjust_repetition(true);
            return true;
        }
        _ => {
            return false;
        }
    }
}
