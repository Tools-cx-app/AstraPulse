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

use std::{fs::metadata, path::PathBuf, str::FromStr};

use anyhow::{Context, Ok, Result};

use crate::{
    file_hander::{read, write},
    framework::scheduler::looper::Mode,
};

pub struct Cpu {
    pub policy: Vec<i32>,
    pub path: Vec<PathBuf>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            policy: Vec::new(),
            path: Vec::new(),
        }
    }

    pub fn get_policy(&mut self) -> Result<()> {
        self.policy.push(0);
        self.path.push(
            PathBuf::from_str("/sys/devices/system/cpu/cpufreq/policy0/")
                .context("无法添加路径")?,
        );
        if metadata("/sys/devices/system/cpu/cpufreq/policy4/").is_ok() {
            self.policy.push(4);
            self.path.push(
                PathBuf::from_str("/sys/devices/system/cpu/cpufreq/policy4/")
                    .context("无法添加路径")?,
            );
        } else if metadata("/sys/devices/system/cpu/cpufreq/policy6/").is_ok() {
            self.policy.push(6);
            self.path.push(
                PathBuf::from_str("/sys/devices/system/cpu/cpufreq/policy6/")
                    .context("无法添加路径")?,
            );
        }
        self.policy.push(7);
        Ok(())
    }

    pub fn set_freqs(&self, mode: Mode) -> Result<()> {
        for _policy in self.policy.clone() {
            for path in self.path.clone() {
                let max = path.join("scaling_max_freq");
                let min = path.join("scaling_min_freq");
                let freqs: String = read(
                    path.join("scaling_available_frequencies")
                        .to_string_lossy()
                        .to_string()
                        .as_str(),
                )?;
                let context: Vec<isize> = freqs
                    .split_whitespace()
                    .filter_map(|s| s.parse::<isize>().ok())
                    .collect();
                let max_freq: isize;
                let min_freq: isize;
                match mode {
                    Mode::Powersave => {
                        max_freq = context[context.len() - 5];
                        min_freq = context[context.len() - 3];
                    }
                    Mode::Balance => {
                        max_freq = context[5];
                        min_freq = context[context.len() - 6];
                    }
                    Mode::Performance => {
                        max_freq = context[1];
                        min_freq = context[context.len() - 6];
                    }
                    Mode::Fast => {
                        max_freq = context[1];
                        min_freq = context[1];
                    }
                }
                write(max.to_str().unwrap(), max_freq.to_string().as_str())
                    .context(format!("无法设置cpu{_policy}频率"))?;
                write(min.to_str().unwrap(), min_freq.to_string().as_str())
                    .context(format!("无法设置cpu{_policy}频率"))?;
            }
        }
        Ok(())
    }

    pub fn set_governors(&self) -> Result<()> {
        for _policy in self.policy.clone() {
            for path in self.path.clone() {
                let governors = path.join("scaling_available_governors");
                let context = {
                    let governors_context = read(path.to_str().unwrap())?;
                    if governors_context.contains("walt") {
                        "walt"
                    } else {
                        "schedutil"
                    }
                };
                write(governors.to_str().unwrap(), context)
                    .context(format!("无法设置cpu{_policy}调速器"))?;
            }
        }
        Ok(())
    }
}
