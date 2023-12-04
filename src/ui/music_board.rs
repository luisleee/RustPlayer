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

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::{self},
    widgets::{Block, BorderType, Borders, LineGauge, ListState, Paragraph, Row, Table},
    Frame,
};

use crate::{app::App, media::player::Player};

use super::{play_list::draw_play_list, progress::draw_progress, repetition::draw_repeat, gap::draw_gap};

pub struct MusicController {
    pub state: ListState,
}

pub fn draw_music_board<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let main_layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Length(3),
            Constraint::Percentage(20),
        ])
        .split(area);

    let top_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout_chunks[0]);
    
    let middle_layout_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Percentage(100),
    ])
    .split(top_layout_chunks[0]);
    draw_header(app, frame, middle_layout_chunks[0]);
    let help_table = Table::new([
        Row::new(["->", "add to playlist"]),
        Row::new(["Enter","select/play all",]),
        Row::new(["Space", "pause/resume"]),
        Row::new(["Esc", "parent folder"]),
        Row::new(["n", "next"]),
        Row::new(["q", "quit"]),
        Row::new(["c", "clear list"]),
        Row::new(["↑/↓", "change selected index"]),
    ])
    .header(
        Row::new(vec!["Key", "Function"])
            .style(Style::default().fg(Color::White))
            .bottom_margin(1),
    )
    .block(
        Block::default()
            .title("Help")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL),
    )
    .column_spacing(2)
    .widths(&[Constraint::Min(6), Constraint::Percentage(100)]);
    frame.render_widget(help_table, middle_layout_chunks[1]);

    draw_play_list(app, frame, top_layout_chunks[1]);

    let bottom_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout_chunks[1]);
    draw_repeat(app, frame, bottom_layout_chunks[0]);
    draw_gap(app, frame, bottom_layout_chunks[1]);

    draw_progress(app, frame, main_layout_chunks[2]);
}

pub fn draw_header<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;
    let main_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let playing_text = if let Some(item) = player.playing_song() {
        String::from(item.name.as_str())
    } else {
        String::from("None")
    };

    let mut text = Paragraph::new(playing_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Now Playing")
                .title_alignment(Alignment::Center),
        );
    if player.is_playing() {
        text = text.style(Style::default().add_modifier(Modifier::SLOW_BLINK));
    };

    let sound_volume_percent = app.player.volume();
    let volume = LineGauge::default()
        .ratio(sound_volume_percent.into())
        .label("VOL(-/+)")
        .line_set(symbols::line::THICK)
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .gauge_style(
            Style::default()
                .fg(Color::LightCyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(text, main_layout_chunks[0]);
    frame.render_widget(volume, main_layout_chunks[1]);
}