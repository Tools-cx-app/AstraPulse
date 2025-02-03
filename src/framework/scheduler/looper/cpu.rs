// Copyright 2023-2025, [rust@localhost] $ (@github-handle)
// // //
// // // This file is part of LightScheduling.
// // //
// // // LightScheduling is free software: you can redistribute it and/or modify it under
// // // the terms of the GNU General Public License as published by the Free
// // // Software Foundation, either version 3 of the License, or (at your option)
// // // any later version.
// // //
// // // LightScheduling is distributed in the hope that it will be useful, but WITHOUT ANY
// // // WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// // // FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// // // details.
// // //
// // // You should have received a copy of the GNU General Public License along
// // // with LightScheduling. If not, see <https://www.gnu.org/licenses/>.

use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use super::Looper;

pub trait Cpu {
    fn write_cpu_min_freq(&self, freq: u32, core: u32) -> std::io::Result<()>;
    fn write_cpu_max_freq(&self, freq: u32, core: u32) -> std::io::Result<()>;
    fn set_core_online(&self, core: u32, online: bool) -> std::io::Result<()>;
}

impl Cpu for Looper {
    fn write_cpu_min_freq(&self, freq: u32, core_id: u32) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpu{core_id}/cpufreq/scaling_min_freq");
        fs::write(&path, freq.to_string())?;
        let permissions = fs::Permissions::from_mode(0o440);
        fs::set_permissions(&path, permissions)?;
        log::info!("核心 {core_id} 最小频率已设置为: {freq} MHz");
        Ok(())
    }

    fn write_cpu_max_freq(&self, freq: u32, core_id: u32) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpu{core_id}/cpufreq/scaling_max_freq");
        fs::write(&path, freq.to_string())?;
        let permissions = fs::Permissions::from_mode(0o440);
        fs::set_permissions(&path, permissions)?;
        log::info!("核心 {core_id} 最大频率已设置为: {freq} MHz");
        Ok(())
    }

    fn set_core_online(&self, core_id: u32, online: bool) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpu{core_id}/online");
        let value = if online { "1" } else { "0" };
        fs::write(&path, value)?;
        let permissions = fs::Permissions::from_mode(0o440);
        fs::set_permissions(&path, permissions)?;
        log::info!("核心 {core_id} 状态已设置为: {value}");
        Ok(())
    }
}

// 独立解析函数
pub fn parse_core_id(key: &str) -> Option<u32> {
    key.chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .ok()
}
