// Copyright 2023-2025, [rust@localhost] $ (@github-handle)
// //
// // This file is part of LightScheduling.
// //
// // LightScheduling is free software: you can redistribute it and/or modify it under
// // the terms of the GNU General Public License as published by the Free
// // Software Foundation, either version 3 of the License, or (at your option)
// // any later version.
// //
// // LightScheduling is distributed in the hope that it will be useful, but WITHOUT ANY
// // WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// // FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// // details.
// //
// // You should have received a copy of the GNU General Public License along
// // with LightScheduling. If not, see <https://www.gnu.org/licenses/>.

mod cpu;
mod topapp;

use crate::framework::config::{
    data::{AppCpu, ConfigApp, ConfigData},
    parser::ConfigParser,
};

use cpu::Cpu;
use topapp::TopappDetector;

#[derive(Debug)]
pub struct Looper {
    pub topapp: String,
}

impl Looper {
    pub fn run(&mut self) {
        let data: ConfigData = match ConfigParser::parser() {
            Ok(s) => s,
            Err(e) => {
                log::error!("无法加载主配置: {}", e);
                return;
            }
        };
        loop {
            let current_top = self.get_current_topapp();
            if !current_top.is_empty() && current_top != self.topapp {
                if let Some(path) = data.app.get(&current_top) {
                    log::info!("检测到前台应用切换至: {current_top}");
                    match ConfigParser::app_config_parser(path) {
                        Ok(app_data) => {
                            self.apply_cpu_config(&app_data.cpu);
                            self.topapp.clone_from(&current_top);
                        }
                        Err(e) => {
                            log::error!("应用配置解析失败: {e}");
                        }
                    }
                } else {
                    self.apply_cpu_config(&data.default.cpu);
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn apply_cpu_config(&self, cpu_config: &AppCpu) {
        for (core_key, core) in &cpu_config.cores {
            let Some(core_id) = cpu::parse_core_id(core_key) else {
                log::error!("无效的核心键格式: {core_key}");
                continue;
            };
            if core.min_freq < 0 || core.max_freq < 0 {
                log::error!(
                    "核心 {} 的配置包含负频率值 (min: {}, max: {})",
                    core_id,
                    core.min_freq,
                    core.max_freq
                );
                continue;
            }
            if let Err(e) = self.set_core_online(core_id, core.online) {
                log::error!("无法设置核心 {} 状态: {}", core_id, e);
            }
            if let Err(e) = self.write_cpu_min_freq(core.min_freq as u32, core_id) {
                log::error!("核心 {} 最小频率设置失败: {}", core_id, e);
            }
            if let Err(e) = self.write_cpu_max_freq(core.max_freq as u32, core_id) {
                log::error!("核心 {} 最大频率设置失败: {}", core_id, e);
            }
        }
    }
}
