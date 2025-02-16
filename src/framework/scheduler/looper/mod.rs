// Copyright 2023-2025, [rust@localhost] $ (@3532340532)
//
// This file is part of AstraPulse.
//
// AstraPulse is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// AstraPulse is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along
// with AstraPulse. If not, see <https://www.gnu.org/licenses/>.

mod buffer;
mod deriver;
mod screen;

use std::collections::HashMap;

use anyhow::{Result};
use buffer::Buffer;
use screen::Screen;
use serde::Deserialize;

use crate::{file_hander::read, framework::config::Data};

use super::TopAppsWatcher;

#[derive(Deserialize, Debug, Clone)]
pub enum Mode {
    Powersave,
    Balance,
    Performance,
    Fast,
}

struct Last {
    topapp: Option<String>,
}
pub struct Looper {
    buffer: Buffer,
    last: Last,
    topapp: TopAppsWatcher,
    config: Data,
    screen: Screen,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(Mode::Balance).unwrap(),
            last: Last { topapp: None },
            topapp: TopAppsWatcher::new(),
            config: Data {
                default: Mode::Balance,
                rest_screen: Mode::Powersave,
                app: HashMap::new(),
            },
            screen: Screen::new(),
        }
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let context = read("/data/adb/modules/AstraPulse/config.toml")?;
        let context: Data = toml::from_str(context.as_str())?;
        self.config = context;
        loop {
            self.screen.get_state();
            self.topapp.topapp_dumper();
            self.switch_mode();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn switch_mode(&mut self) {
        for (app, mode) in self.config.app.clone() {
            if self.last.topapp.clone().unwrap_or_default() != self.topapp.topapps
                && self.topapp.topapps == app
            {
                self.last.topapp = Some(self.topapp.topapps.clone());
                self.match_mode(mode);
            } else if self.screen.state {
                self.match_mode(self.config.rest_screen.clone());
            } else {
                self.match_mode(self.config.default.clone());
            }
        }
    }

    fn match_mode(&mut self, mode: Mode) {
        let mut buffer = self.buffer.clone();
        match mode {
            Mode::Powersave => buffer.set_mode(Mode::Powersave),
            Mode::Balance => buffer.set_mode(Mode::Balance),
            Mode::Performance => buffer.set_mode(Mode::Performance),
            Mode::Fast => buffer.set_mode(Mode::Fast),
        };
        let _ = buffer.try_set_cpuset();
        let _ = buffer.try_set_cpu();
        if self.last.topapp.clone().unwrap_or_default() == self.topapp.topapps {
            log::info!("已为{}设置{:?}", self.topapp.topapps, mode);
        }
    }
}
