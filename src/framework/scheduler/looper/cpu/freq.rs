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

use crate::framework::{
    config::data::CpuCore,
    scheduler::looper::{speed_controller::SpeedController, Looper},
};

pub trait Cpu {
    fn write_cpu_min_freq(&self, freq: i32, policy_id: i8) -> std::io::Result<()>;
    fn write_cpu_max_freq(&self, freq: i32, policy_id: i8) -> std::io::Result<()>;
    fn configure_cpu_cluster(
        &self,
        config: &CpuCore,
        cluster_id: i8,
        controller: &mut SpeedController,
    );
    fn write_frequency(&self, cluster: i8, max_freq: i32, min_freq: i32);
    fn configure_controller(&self, cluster: i8, model: &str, controller: &mut SpeedController);
}
impl Cpu for Looper {
    fn write_cpu_min_freq(&self, freq: i32, policy_id: i8) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_min_freq");
        let write_permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, write_permissions)?;
        fs::write(&path, freq.to_string())?;
        let read_permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(&path, read_permissions)?;
        Ok(())
    }
    fn write_cpu_max_freq(&self, freq: i32, policy_id: i8) -> std::io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpufreq/policy{policy_id}/scaling_max_freq");
        let write_permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, write_permissions)?;
        fs::write(&path, freq.to_string())?;
        let read_permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(&path, read_permissions)?;
        Ok(())
    }
    fn configure_cpu_cluster(
        &self,
        config: &CpuCore,
        cluster_id: i8,
        controller: &mut SpeedController,
    ) {
        self.write_frequency(cluster_id, config.max_freq, config.min_freq);
        self.configure_controller(cluster_id, config.model.as_str(), controller);
    }

    fn write_frequency(&self, cluster: i8, max_freq: i32, min_freq: i32) {
        if let Err(e) = self.write_cpu_max_freq(max_freq, cluster) {
            log::error!("集群{}最大频率设置失败: {e}", cluster);
        }
        if let Err(e) = self.write_cpu_min_freq(min_freq, cluster) {
            log::error!("集群{}最小频率设置失败: {e}", cluster);
        }
    }

    fn configure_controller(&self, cluster: i8, model: &str, controller: &mut SpeedController) {
        if let Err(e) = controller.read_system_controller(cluster) {
            log::error!("无法读取集群{}控制器: {e}", cluster);
            return;
        }
        if let Err(e) = controller.change_controller(model.to_string(), cluster) {
            log::error!("集群{}控制器切换失败: {e}", cluster);
        }
    }
}
