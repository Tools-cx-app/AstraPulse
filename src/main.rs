// Copyright 2023-2025, [rust@localhost] $ (@github-handle)
//
// This file is part of LightScheduling.
//
// LightScheduling is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// LightScheduling is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along
// with LightScheduling. If not, see <https://www.gnu.org/licenses/>.

#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

mod framework;
mod logger;

fn main() {
    if let Err(e) = logger::init() {
        eprintln!("❌ 日志初始化失败: {e}");
        std::process::exit(1);
    }
    log::info!("✅ 日志系统初始化成功");
    framework::scheduler::init();
}

//sys/devices/system/cpu/cpu7/cpufreq/scaling_min_freq
