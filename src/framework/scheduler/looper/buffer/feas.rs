// Copyright 2023-2025, [rust@localhost] $ (@3532340532)
//
// This file is part of AstraPulse.
//
// AstraPulse is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// AstraPulse is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along
// with AstraPulse. If not, see <https://www.gnu.org/licenses/>.

use std::path::Path;

use anyhow::Result;

use crate::file_hander::lock_value;

const FEAS_PATH: &[(&str, &str)] = &[
    ("mtk", "/sys/module/mtk_fpsgo/parameters/perfmgr_enable"),
    (
        "old_qocm",
        "/sys/module/perfmgr_policy/parameters/perfmgr_enable",
    ),
    ("qocm", "/sys/module/perfmgr/parameters/perfmgr_enable"),
    (
        "bocchi",
        "/sys/module/bocchi_perfmgr/parameters/perfmgr_enable",
    ),
];

#[derive(Clone)]
pub struct Feas {
    has_feas: bool,
    model: String,
    fps: i32,
}

impl Feas {
    pub fn new() -> Self {
        let mut has_feas: bool = false;
        let mut model = String::new();
        for (m, p) in FEAS_PATH {
            let path: &Path = Path::new(p);
            if path.exists() {
                log::info!("支持feas");
                has_feas = true;
                model = m.to_string();
            }
        }
        Self {
            has_feas,
            model,
            fps: 0,
        }
    }

    pub fn set_fps(&mut self, fps: i32) {
        self.fps = fps;
    }

    pub fn enable_feas(&self) -> Result<()> {
        if self.has_feas {
            match self.model.as_str() {
                "qocm" => {
                    lock_value("1", vec!["/sys/module/perfmgr/parameters/perfmgr_enable"])?;
                    lock_value(
                        self.fps.to_string().as_str(),
                        vec!["/sys/module/perfmgr/parameters/perfmgr_enable"],
                    )?;
                    self.target_fps_helper_qocm()?;
                }
                "mtk" => {
                    lock_value("1", vec!["/sys/module/mtk_fpsgo/parameters/perfmgr_enable"])?;
                    lock_value(
                        self.fps.to_string().as_str(),
                        vec!["/sys/module/mtk_fpsgo/parameters/fixed_target_fps"],
                    )?;
                    self.target_fps_helper_mtk()?;
                }
                "old_qocm" => {
                    lock_value(
                        "1",
                        vec!["/sys/module/perfmgr_policy/parameters/perfmgr_enable"],
                    )?;
                    lock_value(
                        self.fps.to_string().as_str(),
                        vec!["/sys/module/perfmgr_policy/parameters/fixed_target_fp"],
                    )?;
                }
                "bocchi" => {
                    lock_value(
                        "1",
                        vec!["/sys/module/bocchi_perfmgr/parameters/perfmgr_enable"],
                    )?;
                    lock_value(
                        self.fps.to_string().as_str(),
                        vec!["/sys/module/bocchi_perfmgr/parameters/fixed_target_fps"],
                    )?;
                }
                _ => (),
            }
            log::info!("已开启feas");
        }
        Ok(())
    }

    fn target_fps_helper_mtk(&self) -> Result<()> {
        if self.fps > 100 {
            lock_value(
                "0",
                vec![
                    "/sys/module/mtk_fpsgo/parameters/target_fps_61",
                    "/sys/module/mtk_fpsgo/parameters/target_fps_91",
                ],
            )?;
            lock_value("1", vec!["/sys/module/mtk_fpsgo/parameters/target_fps_121"])?;
        } else if self.fps > 70 && self.fps < 100 {
            lock_value(
                "0",
                vec![
                    "/sys/module/mtk_fpsgo/parameters/target_fps_61",
                    "/sys/module/mtk_fpsgo/parameters/target_fps_121",
                ],
            )?;
            lock_value("1", vec!["/sys/module/mtk_fpsgo/parameters/target_fps_91"])?;
        } else {
            lock_value(
                "0",
                vec![
                    "/sys/module/mtk_fpsgo/parameters/target_fps_91",
                    "/sys/module/mtk_fpsgo/parameters/target_fps_121",
                ],
            )?;
            lock_value("1", vec!["/sys/module/mtk_fpsgo/parameters/target_fps_61"])?;
        }
        Ok(())
    }

    fn target_fps_helper_qocm(&self) -> Result<()> {
        if self.fps > 100 {
            lock_value(
                "0",
                vec![
                    "/sys/module/perfmgr/parameters/target_fps_61",
                    "/sys/module/perfmgr/parameters/target_fps_91",
                ],
            )?;
            lock_value("1", vec!["/sys/module/perfmgr/parameters/target_fps_121"])?;
        } else if self.fps > 70 && self.fps < 100 {
            lock_value(
                "0",
                vec![
                    "/sys/module/perfmgr/parameters/target_fps_61",
                    "/sys/module/perfmgr/parameters/target_fps_121",
                ],
            )?;
            lock_value("1", vec!["/sys/module/perfmgr/parameters/target_fps_91"])?;
        } else {
            lock_value(
                "0",
                vec![
                    "/sys/module/perfmgr/parameters/target_fps_91",
                    "/sys/module/perfmgr/parameters/target_fps_121",
                ],
            )?;
            lock_value("1", vec!["/sys/module/perfmgr/parameters/target_fps_61"])?;
        }
        Ok(())
    }
}
