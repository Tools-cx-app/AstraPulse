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

use std::{fs::read_dir, path::Path};

use anyhow::{Context, Result};
use libc::cpu_set_t;

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

    pub fn try_set_cpu_affinity_scheduler(&self) -> Result<()> {
        for i in self.deriver.clone() {
            if self.topapps == i.pkg {
                let pid = super::find_pid(i.pkg.as_str())?;
                let tid = Self::find_tid(pid, i.processes.thread.as_str())? as libc::pid_t;
                unsafe {
                    let mut set = std::mem::MaybeUninit::<cpu_set_t>::uninit();
                    let set_ptr = set.as_mut_ptr();
                    let set_ref = &mut *set_ptr;
                    libc::CPU_ZERO(set_ref);
                    libc::CPU_SET(i.processes.cpu as usize, set_ref);
                    if libc::sched_setaffinity(
                        tid,
                        std::mem::size_of::<cpu_set_t>(),
                        set_ptr as *const _,
                    ) != 0
                    {
                        return Err(std::io::Error::last_os_error().into());
                    }
                }
            }
        }
        Ok(())
    }

    fn find_tid(pid: u32, thread_name: &str) -> Result<u32> {
        let task_dir = format!("/proc/{}/task", pid);
        if let Ok(entries) = std::fs::read_dir(task_dir) {
            for entry in entries.flatten() {
                let tid_str = entry.file_name().into_string().unwrap_or_default();
                let tid = tid_str.parse::<u32>()?;
                let comm_path = format!("/proc/{}/task/{}/comm", pid, tid);
                if let Ok(comm) = std::fs::read_to_string(comm_path) {
                    if comm.trim() == thread_name {
                        return Ok(tid);
                    }
                }
            }
        }
        Ok(0)
    }
}
