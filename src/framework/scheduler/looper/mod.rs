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

mod boost;
mod buffer;

use anyhow::Result;
use boost::Boost;
use buffer::Buffer;
use libc::{PRIO_PROCESS, setpriority};
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
    boost: Boost,
    default: Mode,
}

impl Looper {
    pub fn new(config: Data) -> Self {
        Self {
            buffer: Buffer::new(Mode::Balance, String::new()).unwrap(),
            last: Last { topapp: None },
            topapp: TopAppsWatcher::new(),
            config,
            boost: Boost::new(),
            default: Mode::Balance,
        }
    }

    fn chang_default(&mut self) -> Result<()> {
        let context = read("/data/adb/modules/AstraPulse/mode")?;
        let mode = vec!["powersave", "balance", "performance", "fast"];
        for i in mode {
            if context.contains(i) {
                match context.as_str() {
                    "powersave" => self.default = Mode::Powersave,
                    "balance" => self.default = Mode::Balance,
                    "performance" => self.default = Mode::Performance,
                    "fast" => self.default = Mode::Fast,
                    _ => {
                        log::error!("/data/adb/modules/AstraPulse/mode 文件异常，使用默认配置");
                        self.default = Mode::Balance;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        self.chang_default()?;
        self.boost.try_run()?;
        loop {
            self.topapp.topapp_dumper();
            self.buffer.clone().set_topapps(self.topapp.topapps.clone());
            self.boost.set_topapps(self.topapp.topapps.clone());
            self.buffer.try_set_touch()?;
            self.change_mode();
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }

    fn change_mode(&mut self) {
        for (app, mode) in self.config.app.clone() {
            if self.last.topapp.clone().unwrap_or_default() != self.topapp.topapps
                && self.topapp.topapps == app
            {
                self.last.topapp = Some(self.topapp.topapps.clone());
                self.match_mode(mode.clone());
                if self.last.topapp.clone().unwrap_or_default() == self.topapp.topapps {
                    log::info!("已为{}设置{:?}", self.topapp.topapps, mode);
                }
            } else {
                self.match_mode(self.default.clone());
            }
        }
    }

    fn try_init_priority(mode: Mode) -> Result<()> {
        let prio = match mode {
            Mode::Powersave => 0,
            Mode::Balance => -5,
            Mode::Performance => -10,
            Mode::Fast => -20,
        };
        unsafe {
            setpriority(PRIO_PROCESS, find_pid("/system/bin/surfaceflinger")?, prio);
            setpriority(
                PRIO_PROCESS,
                find_pid("com.google.android.apps.nexuslauncher")?,
                prio,
            );
            setpriority(PRIO_PROCESS, find_pid("com.android.launcher3")?, prio);
            setpriority(
                PRIO_PROCESS,
                find_pid("com.sec.android.app.launcher")?,
                prio,
            );
            setpriority(PRIO_PROCESS, find_pid("com.oppo.launcher")?, prio);
            setpriority(PRIO_PROCESS, find_pid("com.vivo.launcher")?, prio);
            setpriority(PRIO_PROCESS, find_pid("com.huawei.android.launcher")?, prio);
            setpriority(PRIO_PROCESS, find_pid("com.miui.home")?, prio);
        }
        Ok(())
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
        let _ = buffer.try_set_touch();
        let _ = buffer.try_set_thread();
        let _ = buffer.try_enable_feas();
        let _ = Self::try_init_priority(mode.clone());
    }
}

pub fn find_pid(package_name: &str) -> Result<u32> {
    if let Ok(entries) = std::fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let pid_str = entry.file_name().into_string().ok().unwrap_or_default();
            let pid = pid_str.parse::<u32>()?;
            let cmdline_path = format!("/proc/{}/cmdline", pid);
            if let Ok(cmdline) = std::fs::read_to_string(cmdline_path) {
                if cmdline.trim_matches('\0').contains(package_name) {
                    return Ok(pid);
                }
            }
        }
    }
    Ok(0)
}
