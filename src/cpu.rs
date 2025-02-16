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

use anyhow::{Context, Result};

use crate::{file_hander, framework::scheduler::looper::deriver::Freqs};

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

    fn mhz_to_khz(mhz: isize) -> isize {
        mhz * 1000
    }

    pub fn set_freqs(&self, freqs: Freqs) -> Result<()> {
        for policy in self.policy.clone() {
            for path in self.path.clone() {
                let max = path.join("scaling_max_freq");
                let min = path.join("scaling_min_freq");
                let (max_freq, min_freq) = match policy {
                    0 => (
                        Self::mhz_to_khz(freqs.small.max),
                        Self::mhz_to_khz(freqs.small.min),
                    ),
                    4 | 6 => (
                        Self::mhz_to_khz(freqs.middle.max),
                        Self::mhz_to_khz(freqs.middle.min),
                    ),
                    7 => (
                        Self::mhz_to_khz(freqs.big.max),
                        Self::mhz_to_khz(freqs.big.min),
                    ),
                    _ => anyhow::bail!("不支持的CPU策略: {}", policy),
                };
                file_hander::write(max.to_str().unwrap(), max_freq.to_string().as_str())
                    .context("无法设置cpu{_policy}频率")?;
                file_hander::write(min.to_str().unwrap(), min_freq.to_string().as_str())
                    .context("无法设置cpu{_policy}频率")?;
            }
        }
        Ok(())
    }
}
