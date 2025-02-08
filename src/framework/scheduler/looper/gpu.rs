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

use std::{fs, process::Command};

#[derive(Debug)]
pub struct Gpu {
    model: String,
    product: String,
    topapp: String,
}

impl Gpu {
    pub fn new(topapp: String) -> Self {
        let Ok(gpu_model_output) = Command::new("sh")
            .arg("-c")
            .arg("getprop ro.hardware")
            .output()
        else {
            log::error!("无法获取GPU型号");
            return Self {
                model: String::new(),
                product: String::new(),
                topapp: String::new(),
            };
        };
        let gpu_model = String::from_utf8_lossy(&gpu_model_output.stdout);
        if gpu_model.starts_with("mt") {
            return Self {
                model: "Mali".to_string(),
                product: gpu_model.into_owned(),
                topapp,
            };
        }
        Self {
            model: "Adreno".to_string(),
            product: gpu_model.into_owned(),
            topapp,
        }
    }

    fn d9000(&self) {
        let gpu_max_freq = "/sys/devices/platform/soc/13000000.mali/devfreq/13000000.mali/max_freq";
        let gpu_min_freq = "/sys/devices/platform/soc/13000000.mali/devfreq/13000000.mali/min_freq";
        if self.topapp.contains("com.miHoYo.ys.bilibili")
            | self.topapp.contains("com.miHoYo.Yuanshen")
        {
            let _ = fs::write(gpu_min_freq, "484000000");
            let _ = fs::write(gpu_max_freq, "518000000");
        } else if self.topapp.contains("com.tencent.tmgp.pubgmhd") {
            let _ = fs::write(gpu_min_freq, "252000000");
            let _ = fs::write(gpu_max_freq, "303000000");
        }
    }

    pub fn gpu_scheduler(&self) {
        if self.model.contains("Mali") {
            if self.product.contains("mt6983") {
                self.d9000();
            }
        }
    }
}
