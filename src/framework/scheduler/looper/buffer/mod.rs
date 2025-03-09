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

pub mod deriver;

use std::{fs::read_dir, io::Write, path::Path, process::Command};

use anyhow::{Context, Result};
use libc::cpu_set_t;
use tempfile::NamedTempFile;

use crate::{
    cpu::Cpu,
    file_hander::{read, write},
};
use deriver::Deriver;

use super::Mode;

#[derive(Clone)]
pub struct Buffer {
    pub deriver: Vec<Deriver>,
    pub mode: Mode,
    pub topapps: String,
}

impl Buffer {
    pub fn new(mode: Mode, topapps: String) -> Result<Self> {
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
            topapps,
        })
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_topapps(&mut self, topapps: String) {
        self.topapps = topapps;
    }

    pub fn try_set_cpu(&self) -> Result<()> {
        let mode = self.mode.clone();
        for i in self.deriver.clone() {
            if self.topapps == i.pkg {
                let mut cpu = Cpu::new();
                let _ = cpu.get_policy();
                let _ = cpu.set_governors();
                let _ = match mode {
                    Mode::Powersave => cpu.set_freqs(mode.clone()),
                    Mode::Balance => cpu.set_freqs(mode.clone()),
                    Mode::Performance => cpu.set_freqs(mode.clone()),
                    Mode::Fast => cpu.set_freqs(mode.clone()),
                };
            }
        }
        Ok(())
    }

    pub fn try_set_touch(&self) -> Result<()> {
        let balance = include_str!("./balance.prop");
        let performance = include_str!("./performance.prop");
        let mut temp = NamedTempFile::new()?;
        match self.mode {
            Mode::Balance | Mode::Powersave => temp.write_all(balance.as_bytes()),
            Mode::Fast | Mode::Performance => temp.write_all(performance.as_bytes()),
        }?;
        Command::new("sh")
            .arg("-c")
            .arg("resetprop")
            .arg("-f")
            .arg(temp.path().to_str().unwrap())
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub fn try_set_cpuset(&self) -> Result<()> {
        for i in self.deriver.clone() {
            if self.topapps == i.pkg {
                write("/dev/cpuset/background/cpus", i.cpuset.background.as_str())?;
                write("/dev/cpuset/foreground/cpus", i.cpuset.foreground.as_str())?;
                write("/dev/cpuset/top-app/cpus", i.cpuset.top_app.as_str())?;
                write(
                    "/dev/cpuset/system-background/cpus",
                    i.cpuset.system_background.as_str(),
                )?;
            }
        }
        Ok(())
    }
}
