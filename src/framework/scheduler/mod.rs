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

pub mod looper;
pub mod topapp;

use std::{mem, process};

use anyhow::Result;
use libc::{self, CPU_SET, CPU_ZERO, cpu_set_t, sched_setaffinity};

use super::config::Data;
use crate::file_hander::{read, write};
pub use topapp::TopAppsWatcher;

pub struct Scheduler {
    config: Data,
}

impl Scheduler {
    pub fn new() -> Result<Self> {
        let context = read("/data/adb/modules/AstraPulse/config.toml")?;
        let context: Data = toml::from_str(context.as_str())?;
        Ok(Self { config: context })
    }

    pub fn start_scheduler(&self) -> Result<()> {
        looper::Looper::new(self.config.clone()).enter_looper()?;
        self.set_cpu()?;
        Ok(())
    }

    fn set_cpu(&self) -> Result<()> {
        write(
            "/dev/cpuset//cgroup.procs",
            std::process::id().to_string().as_str(),
        )?;
        unsafe {
            let mut set: cpu_set_t = mem::zeroed();
            CPU_ZERO(&mut set);
            CPU_SET(self.config.runtime.cpu, &mut set);
            let result = sched_setaffinity(
                process::id() as i32,
                mem::size_of::<cpu_set_t>(),
                &set as *const cpu_set_t,
            );
            if result != 0 {
                log::error!("无法绑定到CPU{}", self.config.runtime.cpu);
            }
        }
        Ok(())
    }
}
