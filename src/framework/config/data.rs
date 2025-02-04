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
    pub gpu: AppGpu,
}

#[derive(Debug, Deserialize)]
pub struct ConfigApp {
    pub cpu: AppCpu,
    pub gpu: AppGpu,
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

#[derive(Debug, Deserialize)]
pub struct AppGpu {
    pub min_freq: i32,
    pub max_freq: i32,
}

/*
#[derive(Debug, Deserialize)]
pub struct AppCpu {
    pub cpu0: AppCpu0,
    pub cpu1: AppCpu1,
    pub cpu2: AppCpu2,
    pub cpu3: AppCpu3,
    pub cpu4: AppCpu4,
    pub cpu5: AppCpu5,
    pub cpu6: AppCpu6,
    pub cpu7: AppCpu7,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu0 {
    pub online: bool,
    pub freq: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu1 {
    pub online: bool,
    pub freq: i32,
}
#[derive(Debug, Deserialize)]
pub struct AppCpu2 {
    pub online: bool,
    pub freq: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu3 {
    pub online: bool,
    pub freq: i32,
}
#[derive(Debug, Deserialize)]
pub struct AppCpu4 {
    pub online: bool,
    pub freq: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu5 {
    pub online: bool,
    pub freq: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu6 {
    pub online: bool,
    pub freq: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppCpu7 {
    pub online: bool,
    pub freq: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppGpu {
    pub min_freq: i32,
    pub max_freq: i32,
}
*/
