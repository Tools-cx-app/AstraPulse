// Copyright 2023-2025, [rust@localhost] $ (@github-handle)
// //
// // This file is part of LightScheduling.
// //
// // LightScheduling is free software: you can redistribute it and/or modify it under
// // the terms of the GNU General Public License as published by the Free
// // Software Foundation, either version 3 of the License, or (at your option)
// // any later version.
// //
// // LightScheduling is distributed in the hope that it will be useful, but WITHOUT ANY
// // WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// // FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// // details.
// //
// // You should have received a copy of the GNU General Public License along
// // with LightScheduling. If not, see <https://www.gnu.org/licenses/>.

use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Topapp {
    topapp: String,
}

impl Topapp {
    pub fn new() -> Self {
        Self {
            topapp: get_current_topapp(),
        }
    }
    pub fn eq(&self, s: &String) -> bool {
        self.topapp == *s
    }
    pub fn get(&self) -> String {
        self.topapp.clone()
    }
}

fn parse_topapp_from_dumpsys(output: &str) -> String {
    output
        .lines()
        .find(|line| line.contains("mCurrentFocus"))
        .and_then(|line| {
            line.split_whitespace()
                .find(|s| s.contains('/'))
                .map(|s| s.split('/').next().unwrap_or("").to_string())
        })
        .unwrap_or_default()
}

fn get_current_topapp() -> String {
    let output = match Command::new("sh")
        .arg("-c")
        .arg("dumpsys activity activities | grep mCurrentFocus")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            log::error!("执行dumpsys命令失败: {}", e);
            return String::new();
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_topapp_from_dumpsys(&stdout)
}
