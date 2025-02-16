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

use std::{fs::read_dir, path::Path, process::Command};

use anyhow::{Context, Ok, Result};

use crate::{
    cpu::Cpu,
    file_hander::{read, write},
};

use super::{deriver::Deriver, Mode};

#[derive(Clone)]
pub struct Buffer {
    pub deriver: Vec<Deriver>,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(mode: Mode) -> Result<Self> {
        let mut deriver_path = Vec::new();
        let path = Path::new("/data/adb/modules/AstraPulse/config");
        for entry in read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.ends_with("toml") {
                deriver_path.push(entry_path.display().to_string());
            }
        }
        let mut context = Vec::new();
        for deriver in deriver_path {
            context.push(read(deriver.as_str())?);
        }
        let mut deriver_struct = Vec::new();
        for toml_context in context {
            let deriver_context: Deriver =
                toml::from_str(toml_context.as_str()).context("无法转换配置文件")?;
            deriver_struct.push(deriver_context);
        }
        Ok(Self {
            deriver: deriver_struct,
            mode,
        })
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn try_set_cpu(&self) -> Result<()> {
        let mode = self.mode.clone();
        for freqs in self.deriver.clone() {
            let soc = Self::get_soc()?;
            if soc == freqs.name {
                let mut cpu = Cpu::new();
                let _ = cpu.get_policy();
                let _ = match mode {
                    Mode::Powersave => cpu.set_freqs(freqs.freq.powersave),
                    Mode::Balance => cpu.set_freqs(freqs.freq.balance),
                    Mode::Performance => cpu.set_freqs(freqs.freq.performance),
                    Mode::Fast => cpu.set_freqs(freqs.freq.fast),
                };
            }
        }
        Ok(())
    }

    pub fn get_soc() -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("getprop ro.soc.model")
            .output()
            .context("无法获取Cpu型号")?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn try_set_cpuset(&self) -> Result<()> {
        for freqs in self.deriver.clone() {
            let soc = Self::get_soc()?;
            if soc == freqs.name {
                write(
                    "/dev/cpuset/background/cpus",
                    freqs.cpuset.background.as_str(),
                )?;
                write(
                    "/dev/cpuset/foreground/cpus",
                    freqs.cpuset.foreground.as_str(),
                )?;
                write("/dev/cpuset/top-app/cpus", freqs.cpuset.top_app.as_str())?;
                write(
                    "/dev/cpuset/system-background/cpus",
                    freqs.cpuset.system_background.as_str(),
                )?;
            }
        }
        Ok(())
    }
}
