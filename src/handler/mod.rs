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

use crate::app::App;

use self::{
    fs::handle_fs,
    music_controller::handle_music_controller,
    player::handle_player,
    gap::handle_gap,
    repetition::handle_repetition,
};

mod fs;
mod music_controller;
mod player;
mod repetition;
mod gap;

pub fn handle_keyboard_event(app: &mut App, key: KeyCode) {
    let mut flag;
    
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
