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
mod gpu;
mod speed_controller;

use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time,
};

use cpu::Cpu;
use gpu::Gpu;
use speed_controller::SpeedController;

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
        let topapp_clone = Arc::clone(&self.topapp);
        thread::spawn(move || loop {
            let mut topapp = topapp_clone.lock().unwrap();
            *topapp = Topapp::new();
        });
        let config = match Scheduler::new().def_parser() {
            Ok(config) => config,
            Err(e) => {
                log::error!("无法读取配置文件：{e}");
                return;
            }
        };
        let mut controller = SpeedController::new();
        loop {
            for (app, path) in &config.app {
                if self.topapp.lock().unwrap().eq(app) && state {
                    let app_config = Scheduler::new().app_config_parser(path).unwrap();
                    let mut controller = SpeedController::new();
                    let _ = self.write_cpu_max_freq(app_config.cpu.big.max_freq, 7);
                    let _ = self.write_cpu_min_freq(app_config.cpu.big.min_freq, 7);
                    let _ = self.write_cpu_max_freq(app_config.cpu.middle.max_freq, 4);
                    let _ = self.write_cpu_min_freq(app_config.cpu.middle.min_freq, 4);
                    let _ = self.write_cpu_max_freq(app_config.cpu.small.max_freq, 0);
                    let _ = self.write_cpu_min_freq(app_config.cpu.small.min_freq, 0);
                    let _ = controller.read_system_controller(7);
                    let _ = controller.change_controller(app_config.cpu.big.model, 7);
                    let _ = controller.read_system_controller(4);
                    let _ = controller.change_controller(app_config.cpu.middle.model, 4);
                    let _ = controller.read_system_controller(0);
                    let _ = controller.change_controller(app_config.cpu.small.model, 0);
                    Gpu::new(self.topapp.lock().unwrap().get()).gpu_scheduler();
                } else {
                    let _ = self.write_cpu_max_freq(config.default.cpu.big.max_freq, 7);
                    let _ = self.write_cpu_min_freq(config.default.cpu.big.min_freq, 7);
                    let _ = self.write_cpu_max_freq(config.default.cpu.middle.max_freq, 4);
                    let _ = self.write_cpu_min_freq(config.default.cpu.middle.min_freq, 4);
                    let _ = self.write_cpu_max_freq(config.default.cpu.small.max_freq, 0);
                    let _ = self.write_cpu_min_freq(config.default.cpu.small.min_freq, 0);
                    let _ = controller.read_system_controller(7);
                    let _ = controller.change_controller(config.default.cpu.big.model.clone(), 7);
                    let _ = controller.read_system_controller(4);
                    let _ =
                        controller.change_controller(config.default.cpu.middle.model.clone(), 4);
                    let _ = controller.read_system_controller(0);
                    let _ = controller.change_controller(config.default.cpu.small.model.clone(), 0);
                }
            }
        }
    }
}
