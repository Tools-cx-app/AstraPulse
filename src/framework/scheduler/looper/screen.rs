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

static PATTERNS: Lazy<[(Lazy<Regex>, [&str; 2]); 3]> = Lazy::new(|| {
    [
        (
            Lazy::new(|| Regex::new(r"Display Power: state=(\w+)").unwrap()),
            ["ON", "OFF"],
        ),
        (
            Lazy::new(|| Regex::new(r"mWakefulness=(\w+)").unwrap()),
            ["Awake", "Asleep"],
        ),
        (
            Lazy::new(|| Regex::new(r"mScreenOn=(\w+)").unwrap()),
            ["true", "false"],
        ),
    ]
});

pub struct Screen {
    dumper: Dumpsys,
    pub state: bool,
    time: Instant,
}

impl Screen {
    pub fn new() -> Self {
        let dumper = loop {
            if let Some(dump) = Dumpsys::new("power") {
                break dump;
            } else {
                log::error!("无法获取屏幕状态，正在重试");
                std::thread::sleep(Duration::from_secs(1));
            }
        };
        Self {
            dumper,
            state: true,
            time: Instant::now(),
        }
    }

    pub fn get_state(&mut self) {
        if self.time.elapsed() > RESET_TIME {
            let dump = loop {
                match self.dumper.dump(&[""]) {
                    Ok(dump) => break dump,
                    Err(e) => {
                        log::error!("无法获取顶层应用：{e}，正在重试");
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            };
            for (re, states) in PATTERNS.iter() {
                if let Some(caps) = re.captures(&dump) {
                    let state = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                    self.state = match state {
                        s if s == states[0] => true,
                        s if s == states[1] => false,
                        "true" => true,
                        "false" => false,
                        _ => true,
                    };
                }
            }
            #[cfg(debug_assertions)]
            {
                log::debug!("当前屏幕状态 {}", self.state);
            }
        }
    }
}
