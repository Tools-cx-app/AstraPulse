// Copyright 2023-2025, [rust@localhost] $ (@3532340532)
//
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

mod cpu;
mod file_hander;
mod framework;
mod logger;

use anyhow::Context;
use file_hander::write;

fn main() -> anyhow::Result<(), anyhow::Error> {
    logger::log_init().context("😂无法初始化日志")?;
    write(
        "/dev/cpuset/background/cgroup.procs",
        std::process::id().to_string().as_str(),
    )?;
    framework::scheduler::looper::Looper::new()
        .enter_looper()
        .context("😂无法启动")?;
    Ok(())
}
