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

use std::{fs, io};

use super::config::data::{ConfigApp, ConfigData};

pub mod looper;
pub mod topapp;

#[derive(Debug)]
pub struct Scheduler {
    def_config: Option<ConfigData>,
    app_config: Option<ConfigApp>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            def_config: None,
            app_config: None,
        }
    }
    pub fn def_parser(&mut self) -> Result<ConfigData, io::Error> {
        let config =
            fs::read_to_string("/data/adb/modules/LightScheduling/config.toml").map_err(|e| {
                log::error!("无法读取配置文件：{e}");
                e
            })?;
        let def_config: ConfigData = toml::from_str(&config).map_err(|e| {
            log::error!("无法转换配置文件：{e}");
            io::Error::new(io::ErrorKind::InvalidData, e.to_string())
        })?;
        self.def_config = Some(def_config.clone());
        Ok(def_config)
    }
    pub fn app_config_parser(&mut self, path: &String) -> Result<ConfigApp, io::Error> {
        let config = fs::read_to_string(path).map_err(|e| {
            log::error!("无法读取配置文件：{e}");
            e
        })?;
        let app_config: ConfigApp = toml::from_str(&config).map_err(|e| {
            log::error!("无法转换配置文件：{e}");
            io::Error::new(io::ErrorKind::InvalidData, e.to_string())
        })?;
        self.app_config = Some(app_config.clone());
        Ok(app_config)
    }
}
