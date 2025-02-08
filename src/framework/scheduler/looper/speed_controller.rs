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

use std::{fs, io, os::unix::fs::PermissionsExt, path};

#[derive(Debug)]
pub struct SpeedController {
    controller: String,
    //system_controller: String,
}

impl SpeedController {
    pub fn new() -> Self {
        Self {
            controller: String::new(),
            //system_controller: String::new(),
        }
    }
    pub fn change_controller(
        &mut self,
        controller: String,
        policy_id: i32,
    ) -> Result<(), io::Error> {
        let path = format!("/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_governor");
        let available_governors: Vec<&str> = self.controller.split_whitespace().collect();
        if available_governors.contains(&controller.as_str()) {
            let write_permissions = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&path, write_permissions)?;
            fs::write(&path, &controller)?;
            let read_permissions = fs::Permissions::from_mode(0o400);
            fs::set_permissions(&path, read_permissions)?;
            log::info!("核心集 {policy_id} 的调速器已设置为 {controller}");
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("调速器 {} 不可用于策略 {}", controller, policy_id),
            ))
        }
    }
    pub fn read_system_controller(&mut self, policy_id: i32) -> Result<(), io::Error> {
        let path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_available_governors"
        );
        let content = fs::read_to_string(&path)?;
        self.controller = content.trim().to_string();
        Ok(())
    }
}
