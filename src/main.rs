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

use std::{fs, process::exit};

use anyhow::{Context, Result};
use file_hander::lock_value;
use framework::scheduler::Scheduler;

fn wait_boot() {
    while android_system_properties::AndroidSystemProperties::new()
        .get("sys.boot_completed")
        .unwrap_or_default()
        .contains("1")
    {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn check_process() {
    let mut count = 0;
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let pid = entry.file_name().into_string().unwrap_or_default();
            if pid.parse::<u32>().is_err() {
                continue;
            }
            if let Ok(cmdline) = fs::read_to_string(format!("/proc/{}/cmdline", pid)) {
                if cmdline.contains("AstraPulse") {
                    count += 1;
                }
            }
        }
    }
    if count > 1 {
        eprintln!("发现另一个进程，程序退出");
        exit(1);
    }
}

fn init() -> Result<()> {
    lock_value(
        "0",
        vec![
            "/sys/module/mtk_fpsgo/parameters/perfmgr_enable",
            "/sys/module/perfmgr/parameters/perfmgr_enable",
            "/sys/module/perfmgr_policy/parameters/perfmgr_enable",
            "/sys/module/perfmgr_mtk/parameters/perfmgr_enable",
            "/sys/module/migt/parameters/glk_fbreak_enable",
            "/sys/kernel/fpsgo/fbt/switch_idleprefer",
            "/sys/module/mtk_fpsgo/parameters/boost_affinity",
            "/sys/kernel/fpsgo/minitop/enable",
        ],
    )?;
    lock_value(
        "1",
        vec![
            "/sys/module/migt/parameters/glk_disable",
            "/proc/game_opt/disable_cpufreq_limit",
        ],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    init()?;
    wait_boot();
    check_process();
    logger::log_init().context("😂无法初始化日志")?;
    Scheduler::new()?.start_scheduler()?;
    Ok(())
}
