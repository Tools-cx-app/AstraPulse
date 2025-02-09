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

use std::{fs, os::unix::fs::PermissionsExt};

use super::Looper;

pub trait Cpu {
    fn write_cpu_min_freq(&self, freq: i32, policy_id: i32) -> std::io::Result<()>;
    fn write_cpu_max_freq(&self, freq: i32, policy_id: i32) -> std::io::Result<()>;
}

impl Cpu for Looper {
    fn write_cpu_min_freq(&self, freq: i32, policy_id: i32) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_min_freq");
        let write_permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, write_permissions)?;
        fs::write(&path, freq.to_string())?;
        let read_permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(&path, read_permissions)?;
        log::info!("核心集 {policy_id} 最小频率已设置为: {freq} Hz");
        Ok(())
    }
    fn write_cpu_max_freq(&self, freq: i32, policy_id: i32) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_max_freq");
        let write_permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, write_permissions)?;
        fs::write(&path, freq.to_string())?;
        let read_permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(&path, read_permissions)?;
        log::info!("核心集 {policy_id} 最大频率已设置为: {freq} Hz");
        Ok(())
    }
}
