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

use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use anyhow::Result;
use frame_analyzer::Analyzer;

use crate::cpu::Cpu;

use super::{find_pid, Mode};

pub struct Sf {
    topapps: String,
}

impl Sf {
    pub fn new() -> Self {
        Self { topapps : String::new() }
    }

    pub fn set_topapps(&mut self, topapps: String) {
        self.topapps = topapps;
    }

    pub fn try_run(&self) -> Result<()> {
        let mut analyzer = Analyzer::new()?;
        analyzer.attach_app(find_pid(self.topapps.as_str())? as i32)?;
        let running = Arc::new(AtomicBool::new(true));
        {
            let running = running.clone();
            ctrlc::set_handler(move || {
                running.store(false, Ordering::Release);
            })?;
        }
        let mut buffer = VecDeque::with_capacity(120);
        thread::spawn(move || {
            while running.load(Ordering::Acquire) {
                if let Some((_, frametime)) = analyzer.recv() {
                    if buffer.len() >= 120 {
                        buffer.pop_back();
                    }
                    buffer.push_front(frametime);
                    if buffer.len() > 30 {
                        let mut cpu = Cpu::new();
                        let _ = cpu.get_policy();
                        let _ = cpu.set_freqs(Mode::Fast);
                    }
                }
            }
        });
        Ok(())
    }
}
