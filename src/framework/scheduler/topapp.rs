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

use std::{
    process::Command,
    time::{Duration, Instant},
};

use dumpsys_rs::Dumpsys;
use once_cell::sync::Lazy;
use regex::Regex;

const RESET_TIME: Duration = Duration::from_secs(1);

static WINDOW_TYPES: &[(&str, &str)] = &[
    ("overlay", r"type=APPLICATION_OVERLAY"),
    ("freeform", r"windowingMode=5"),
    ("pip", r"mPictureInPicture"),
    ("dialog", r"type=APPLICATION_ABOVE_SUB_PANEL"),
];
static WINDOW_REGEX: Lazy<Regex> = Lazy::new(|| {
    let patterns: Vec<&str> = WINDOW_TYPES.iter().map(|(_, p)| *p).collect();
    Regex::new(&format!(
        r"Window\{{.*?\s+([a-zA-Z0-9._]+)/.*?({})",
        patterns.join("|")
    ))
    .unwrap()
});
static FOCUSED_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mCurrentFocus=Window\{.*?\s+([a-zA-Z0-9._]+)/").unwrap());

pub struct TopAppsWatcher {
    command_dumper: String,
    dumper: Dumpsys,
    pub topapps: String,
    time: Instant,
}

impl TopAppsWatcher {
    pub fn new() -> Self {
        let mut command_dumper = String::new();
        let dumper = loop {
            if let Some(dump) = Dumpsys::new("window") {
                break dump;
            } else {
                /*log::error!("无法获取顶层应用，正在重试");
                std::thread::sleep(Duration::from_secs(1));*/
                let output = Command::new("sh")
                    .arg("-c")
                    .arg("dumpsys window visible-apps")
                    .output()
                    .unwrap();
                command_dumper = String::from_utf8_lossy(&output.stdout).to_string();
            }
        };
        Self {
            command_dumper,
            dumper,
            topapps: String::new(),
            time: Instant::now(),
        }
    }

    pub fn topapp_dumper(&mut self) {
        if self.time.elapsed() > RESET_TIME {
            if !self.command_dumper.is_empty() {
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
            } else {
                self.topapps = Self::parse_top_app(self.command_dumper.as_str());
            }
            #[cfg(debug_assertions)]
            {
                log::debug!("当前顶层应用 {}", self.topapps);
            }
        }
    }

    fn parse_top_app(dump: &str) -> String {
        let mut windows: Vec<&str> = WINDOW_REGEX
            .captures_iter(dump)
            .filter_map(|c| c.get(1).map(|m| m.as_str()))
            .collect();
        if let Some(top_window) = windows.pop() {
            return top_window.to_string();
        }
        FOCUSED_REGEX
            .captures(dump)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default()
    }
}
