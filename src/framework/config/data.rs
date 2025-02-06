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

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigData {
    pub default: DefaultConfig,
    pub app: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    pub cpu: AppCpu,
}

#[derive(Debug, Deserialize)]
pub struct ConfigApp {
    pub cpu: AppCpu,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu {
    pub cores: HashMap<String, CpuCore>,
}

#[derive(Debug, Deserialize)]
pub struct CpuCore {
    pub online: bool,
    pub min_freq: i32,
    pub max_freq: i32,
}