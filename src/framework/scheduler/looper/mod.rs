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

mod cpu;
mod speed_controller;

use std::{
    fs::File,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use cpu::{Cpu, Migrate};
use speed_controller::SpeedController;
use tempfile::tempdir;

use crate::framework::config::data::{ConfigApp, ConfigData};

use super::{topapp::Topapp, Scheduler};

#[derive(Debug)]
pub struct Looper {
    topapp: Arc<Mutex<Topapp>>,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            topapp: Arc::new(Mutex::new(Topapp::new())),
        }
    }

    pub fn run(self) {
        let topapp_monitor = Arc::clone(&self.topapp);
        thread::spawn(move || loop {
            *topapp_monitor.lock().unwrap() = Topapp::new();
            thread::sleep(Duration::from_secs(1));
        });

        let config = match Scheduler::new().def_parser() {
            Ok(config) => config,
            Err(e) => {
                log::error!("无法读取配置文件：{e}");
                return;
            }
        };

        loop {
            let current_app = self.get_current_app();
            let temp = match tempdir() {
                Ok(t) => t,
                Err(e) => {
                    log::warn!("无法创建临时目录：{e}");
                    continue;
                }
            };

            self.process_application_config(&current_app, &config, &temp);
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn get_current_app(&self) -> String {
        self.topapp.lock().unwrap().get()
    }

    fn process_application_config(
        &self,
        app_name: &str,
        config: &ConfigData,
        temp: &tempfile::TempDir,
    ) {
        let tempfile_path = temp.path().join(app_name);

        if tempfile_path.exists() {
            return;
        }

        match config.app.iter().find(|(name, _)| *name == app_name) {
            Some((_, config_path)) => self.handle_app_specific_config(app_name, config_path),
            None => self.apply_default_config(config),
        }

        if let Err(e) = File::create(&tempfile_path) {
            log::warn!("无法创建临时文件[{}]: {e}", tempfile_path.display());
        }
    }

    fn handle_app_specific_config(&self, app_name: &str, config_path: &str) {
        match Scheduler::new().app_config_parser(&config_path.to_string()) {
            Ok(app_config) => {
                log::info!("正在为 {app_name} 应用性能配置");
                self.apply_cpu_config(None, Some(&app_config));
                Self::apply_cpu_migration(app_name);
            }
            Err(e) => {
                log::error!("应用配置[{}]加载失败: {}", config_path, e);
                let config = match Scheduler::new().def_parser() {
                    Ok(config) => config,
                    Err(e) => {
                        log::error!("无法读取配置文件：{e}");
                        return;
                    }
                };
                self.apply_default_config(&config);
            }
        }
    }

    fn apply_default_config(&self, config: &ConfigData) {
        self.apply_cpu_config(Some(config), None);
    }

    fn apply_cpu_migration(app_name: &str) {
        if let Err(e) = Migrate::new(app_name).setting() {
            log::error!("CPU迁移设置失败: {e}");
        }
    }

    fn apply_cpu_config(
        &self,
        default_config: Option<&ConfigData>,
        app_config: Option<&ConfigApp>,
    ) {
        let mut controller = SpeedController::new();

        match (default_config, app_config) {
            (None | Some(_), Some(app)) => {
                self.configure_cpu_cluster(&app.cpu.big, 7, &mut controller);
                self.configure_cpu_cluster(&app.cpu.middle, 4, &mut controller);
                self.configure_cpu_cluster(&app.cpu.small, 0, &mut controller);
            }
            (Some(default), None) => {
                self.configure_cpu_cluster(&default.default.cpu.big, 7, &mut controller);
                self.configure_cpu_cluster(&default.default.cpu.middle, 4, &mut controller);
                self.configure_cpu_cluster(&default.default.cpu.small, 0, &mut controller);
            }
            _ => log::warn!("无有效CPU配置"),
        }
    }
}
