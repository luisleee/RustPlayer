use crossterm::event::KeyCode;

use crate::{app::App, media::player::Player};

pub fn handle_gap(app: &mut App, code: KeyCode) -> bool {
    let player = &mut app.player;
    match code {
        KeyCode::Char('j') | KeyCode::Char('J') => {
            player.adjust_gap(-0.5);
            return true;
        }
        KeyCode::Char('k') | KeyCode::Char('K') => {
            player.adjust_gap(0.5);
            return true;
        }
        _ => {
            return false;
        }
    }
}