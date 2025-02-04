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

use std::{ffi::OsStr, fs, os::unix::prelude::PermissionsExt, path::Path, process::Command};

use super::Looper;

pub trait Gpu {
    fn write_gpu_min_freq(&self, freq: i32) -> std::io::Result<()>;
    fn write_gpu_max_freq(&self, freq: i32) -> std::io::Result<()>;
}

impl Gpu for Looper {
    fn write_gpu_max_freq(&self, freq: i32) -> std::io::Result<()> {
        let path = format!("{}/max_freq", get_gpu_path());
        let write_permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, write_permissions)?;
        fs::write(&path, freq.to_string())?;
        let read_permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(&path, read_permissions)?;
        log::info!("GPU 最大频率已设置为: {freq} Hz");
        Ok(())
    }
    fn write_gpu_min_freq(&self, freq: i32) -> std::io::Result<()> {
        let path = format!("{}/min_freq", get_gpu_path());
        let write_permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, write_permissions)?;
        fs::write(&path, freq.to_string())?;
        let read_permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(&path, read_permissions)?;
        log::info!("GPU 最小频率已设置为: {freq} Hz");
        Ok(())
    }
}

fn get_gpu_path() -> String {
    let gpu_hd = get_gpu_hardware();
    if gpu_hd.starts_with("mt") {
        const MALI_GPU_PATH: &str = "/sys/devices/platform/soc/";
        let mali_gpu_path = Path::new(MALI_GPU_PATH);

        let entries = match fs::read_dir(mali_gpu_path) {
            Ok(s) => s,
            Err(e) => {
                log::error!("无法读取 {}：{}", MALI_GPU_PATH, e);
                return String::new();
            }
        };
        for entry in entries.flatten() {
            if entry.file_name() == OsStr::new("devfreq") {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(path_str) = entry.path().to_str() {
                            return format!("{MALI_GPU_PATH}/devfreq/{}", path_str.to_owned());
                        }
                        log::warn!("发现非 UTF-8 路径: {:?}", entry.path());
                    }
                }
            }
        }
    } else {
        return "/sys/class/kgsl/kgsl-3d0/devfreq/".to_string();
    }
    String::new()
}

fn get_gpu_hardware() -> String {
    let output = match Command::new("sh")
        .arg("-c")
        .arg("getprop ro.hardware")
        .output()
    {
        Ok(s) => s,
        Err(e) => {
            log::error!("无法获取gpu代号：{e}");
            return String::new();
        }
    };
    String::from_utf8_lossy(&output.stdout).to_string()
}
