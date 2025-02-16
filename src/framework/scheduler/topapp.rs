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

use std::time::{Duration, Instant};

use dumpsys_rs::Dumpsys;
use once_cell::sync::Lazy;
use regex::Regex;

const RESET_TIME: Duration = Duration::from_secs(1);

static ACTIVITY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"ActivityRecord\{[^\s]+\s+u\d+\s+([a-zA-Z0-9._]+)").unwrap());

pub struct TopAppsWatcher {
    dumper: Dumpsys,
    pub topapps: String,
    time: Instant,
}

impl TopAppsWatcher {
    pub fn new() -> Self {
        let dumper = loop {
            if let Some(dump) = Dumpsys::new("window") {
                break dump;
            } else {
                log::error!("无法获取顶层应用，正在重试");
                std::thread::sleep(Duration::from_secs(1));
            }
        };
        Self {
            dumper,
            topapps: String::new(),
            time: Instant::now(),
        }
    }

    pub fn topapp_dumper(&mut self) {
        if self.time.elapsed() > RESET_TIME {
            let dump = loop {
                match self.dumper.dump(&["visible-apps"]) {
                    Ok(dump) => break dump,
                    Err(e) => {
                        log::error!("无法获取顶层应用：{e}，正在重试");
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            };
            self.topapps = Self::parse_top_app(&dump);
        }
    }

    fn parse_top_app(dump: &str) -> String {
        ACTIVITY_REGEX
            .captures(dump)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_default()
    }
}
