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

use anyhow::Result;
use libc::cpu_set_t;

use com_tencent_tmgp_pubgmhd::Pubgmhd;
use com_tencent_jkchess::Jkchess;

mod com_tencent_jkchess;
mod com_tencent_tmgp_pubgmhd;

pub struct Policy {
    pub pid: u32,
    pub tid: u32,
}

impl Policy {
    pub fn new() -> Self {
        Self { pid: 0, tid: 0 }
    }

    fn find_pid(&mut self, package_name: &str) -> Result<u32> {
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let pid_str = entry.file_name().into_string().ok().unwrap_or_default();
                let pid = pid_str.parse::<u32>()?;
                let cmdline_path = format!("/proc/{}/cmdline", pid);
                if let Ok(cmdline) = std::fs::read_to_string(cmdline_path) {
                    if cmdline.trim_matches('\0').contains(package_name) {
                        self.pid = pid;
                    }
                }
            }
        }
        Ok(0)
    }

    fn find_tid(&mut self, pid: u32, thread_name: &str) -> Result<u32> {
        let task_dir = format!("/proc/{}/task", pid);
        if let Ok(entries) = std::fs::read_dir(task_dir) {
            for entry in entries.flatten() {
                let tid_str = entry.file_name().into_string().unwrap_or_default();
                let tid = tid_str.parse::<u32>()?;
                let comm_path = format!("/proc/{}/task/{}/comm", pid, tid);
                if let Ok(comm) = std::fs::read_to_string(comm_path) {
                    if comm.trim() == thread_name {
                        self.tid = tid;
                    }
                }
            }
        }
        Ok(0)
    }

    pub fn try_set(&mut self, topapps: String) -> Result<()> {
        match topapps.as_str() {
            "com.tencent.tmgp.pubgmhd" => self.pubgmhd(),
            "com.tencent.jkchess" => self.jkchess(),
            _ => Ok(()),
        }?;
        Ok(())
    }

    fn set_affinity(tid: libc::pid_t, cpu: usize) -> Result<(), std::io::Error> {
        unsafe {
            let mut set = std::mem::MaybeUninit::<cpu_set_t>::uninit();
            let set_ptr = set.as_mut_ptr();
            let set_ref = &mut *set_ptr;
            libc::CPU_ZERO(set_ref);
            libc::CPU_SET(cpu, set_ref);
            if libc::sched_setaffinity(tid, std::mem::size_of::<cpu_set_t>(), set_ptr as *const _)
                == 0
            {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error())
            }
        }
    }

    fn set_scheduler(tid: libc::pid_t, policy: i32) -> Result<(), std::io::Error> {
        unsafe {
            let param = libc::sched_param { sched_priority: 50 };
            if libc::sched_setscheduler(tid, policy, &param) == 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error())
            }
        }
    }
}
