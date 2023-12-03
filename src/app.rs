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

use std::{
    io::stdout,
    sync::mpsc,
    thread::{self},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use failure::Error;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    config::Config,
    fs::FsExplorer,
    handler::handle_keyboard_event,
    media::player::{MusicPlayer, Player},
    ui::{
        fs::draw_fs_tree,
        music_board::draw_music_board,
        music_board::MusicController,
        EventType,
    },
};

pub enum InputMode {
    Normal,
}
pub struct App {
    pub mode: InputMode,
    pub fs: FsExplorer,
    pub player: MusicPlayer,
    pub music_controller: MusicController,
    pub config: Config,
    msg: String,
}

impl App {
    pub fn new() -> Option<Self> {
        Some(Self {
            mode: InputMode::Normal,
            fs: FsExplorer::default(Some(|err| {
                eprintln!("{}", err);
            }))
            .ok()?,
            player: Player::new(),
            music_controller: MusicController {
                state: ListState::default(),
            },
            msg: "Welcome to RustPlayer".to_string(),
            config: Config::default(),
        })
    }

    // block thread and show screen
    pub fn run(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        terminal.hide_cursor()?;
        self.draw_frame(&mut terminal)?;
        // tick daemon thread
        let (sd, rd) = mpsc::channel::<EventType>();
        let tick = self.config.tick_gap.clone();
        thread::spawn(move || loop {
            thread::sleep(tick);
            let _ = sd.send(EventType::Player);
        });
        // start event
        let (evt_sender, evt_receiver) = mpsc::sync_channel(1);
        let (exit_sender, exit_receiver) = mpsc::channel();
        let evt_th = thread::spawn(move || loop {
            let evt = event::read();
            match evt {
                Ok(evt) => {
                    if let Event::Key(key) = evt {
                        match self.mode {
                            InputMode::Normal => match key.code {
                                KeyCode::Char('q') | KeyCode::Char('Q') => {
                                    drop(evt_sender);
                                    let _ = exit_sender.send(());
                                    return;
                                }
                                code => {
                                    match evt_sender.send(code) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            // send error, exit.
                                            return;
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
                Err(_) => {
                    // exit.
                    return;
                }
            }
        });
        loop {
            thread::sleep(self.config.refresh_rate);
            if let Ok(_) = exit_receiver.try_recv() {
                break;
            }
            match evt_receiver.try_recv() {
                Ok(code) => handle_keyboard_event(self, code),
                _ => {}
            }
            // 10 fps
            self.draw_frame(&mut terminal)?;
            if let Ok(event) = rd.try_recv() {
                self.handle_events(event);
            }
        }
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
        terminal.show_cursor()?;
        let _ = evt_th.join();
        Ok(())
    }

    fn handle_events(&mut self, event: EventType) {
        // event
        match event {
            EventType::Player => {
                let player = &mut self.player;
                player.tick();
            }
        }
    }

    pub fn draw_frame<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Error>
    where
        B: Backend,
    {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
                .split(size);
            self.draw_header(frame, chunks[0]);
            self.draw_body(frame, chunks[1]).unwrap();
        })?;
        Ok(())
    }

    pub fn draw_header<B>(&mut self, frame: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let block = Block::default()
            .title("RustPlayer - Music Player For Rust")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White));
        let msg_p = Paragraph::new(Text::from(self.msg.as_str()))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(block)
            .wrap(Wrap { trim: true });
        // total
        frame.render_widget(msg_p, area);
    }

    pub fn draw_body<B>(&mut self, frame: &mut Frame<B>, area: Rect) -> Result<(), Error>
    where
        B: Backend,
    {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);
        // 左侧
        draw_fs_tree(self, frame, main_layout[0]);
        // 右侧
        draw_music_board(self, frame, main_layout[1]);
        Ok(())
    }

    pub fn set_msg(&mut self, msg: &str) {
        self.msg = String::from(msg);
    }
}
