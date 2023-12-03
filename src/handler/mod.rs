// Copyright (C) 2022 KetaNetwork
//
// This file is part of RustPlayer.
//
// RustPlayer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RustPlayer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RustPlayer.  If not, see <http://www.gnu.org/licenses/>.

use crossterm::event::KeyCode;

use crate::app::{App, Routes};

use self::{
    fs::handle_fs,
    help::handle_help,
    music_controller::{handle_music_controller},
    player::{handle_player},
};

mod fs;
mod help;
mod music_controller;
mod player;
mod repetition;
mod gap;

pub fn handle_routes(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('h') | KeyCode::Char('H') => {
            if let Some(page) = app.route_stack.last() {
                match page {
                    Routes::Main => {
                        app.route_stack.push(Routes::Help);
                    }
                    Routes::Help => {
                        app.route_stack.pop();
                    }
                }
            }
            return true;
        }
        _ => {}
    }
    false
}

pub fn handle_keyboard_event(app: &mut App, key: KeyCode) {
    let mut flag;
    let top_route = app.route_stack.last().unwrap();

    match top_route {
        Routes::Main => {
            flag = handle_fs(app, key);
            if flag {
                return;
            }
            flag = handle_player(app, key);
            if flag {
                return;
            }
            flag = handle_music_controller(app, key);
            if flag {
                return;
            }
            flag = handle_repetition(app, key);
            if flag {
                return;
            }
            flag = handle_gap(app, key);
            if flag {
                return;
            }
        }
        Routes::Help => {
            flag = handle_help(app, key);
            if flag {
                return;
            }
        }
    }
    flag = handle_routes(app, key);
    if flag {
        return;
    }
}
