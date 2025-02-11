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

use std::{fs, io, os::unix::fs::PermissionsExt};

#[allow(clippy::pedantic)]
#[derive(Debug)]
pub struct SpeedController {
    controller: String,
}

#[allow(clippy::pedantic)]
impl SpeedController {
    pub const fn new() -> Self {
        Self {
            controller: String::new(),
        }
    }
    pub fn change_controller(&self, controller: String, policy_id: i8) -> Result<(), io::Error> {
        let path = format!("/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_governor");
        // let available_governors: Vec<&str> = self.controller.split_whitespace().collect();
        if self
            .controller
            .split_whitespace()
            .any(|x| x == controller.as_str())
        {
            let write_permissions = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&path, write_permissions)?;
            fs::write(&path, &controller)?;
            let read_permissions = fs::Permissions::from_mode(0o400);
            fs::set_permissions(&path, read_permissions)?;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("调速器 {controller} 不可用于策略 {policy_id}"),
            ))
        }
    }
    pub fn read_system_controller(&mut self, policy_id: i8) -> Result<(), io::Error> {
        let path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_available_governors"
        );
        let content = fs::read_to_string(&path)?;
        self.controller = content.trim().to_string();
        Ok(())
    }
}
