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

use anyhow::{Context, Result};

use super::Policy;

pub trait Pubgmhd {
    fn pubgmhd(&mut self) -> Result<()>;
}

impl Pubgmhd for Policy {
    fn pubgmhd(&mut self) -> Result<()> {
        let pid = self
            .find_pid("com.tencent.tmgp.pubgmhd")
            .context("无法获取应用pid")?;
        let thread_13 = self.find_tid(pid, "Thread-13").context("无法获取进程tid")?;
        let rhithread = self.find_tid(pid, "RHIThread").context("无法获取进程tid")?;
        Self::set_affinity(thread_13 as libc::pid_t, 6)?;
        Self::set_scheduler(rhithread as libc::pid_t, 6)?;
        Self::set_scheduler(thread_13 as libc::pid_t, 6)?;
        Self::set_scheduler(rhithread as libc::pid_t, 6)?;
        Ok(())
    }
}
