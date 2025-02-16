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

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Deriver {
    pub name: String,
    pub cpuset: Cpuset,
    pub freq: Modefreqs,
}

#[derive(Deserialize, Clone)]
pub struct Modefreqs {
    pub powersave: Freqs,
    pub balance: Freqs,
    pub performance: Freqs,
    pub fast: Freqs,
}

#[derive(Deserialize, Clone)]
pub struct Freqs {
    pub big: Freq,
    pub middle: Freq,
    pub small: Freq,
}

#[derive(Deserialize, Clone)]
pub struct Freq {
    pub max: isize,
    pub min: isize,
}

#[derive(Deserialize, Clone)]
pub struct Cpuset {
    pub top_app: String,
    pub background: String,
    pub foreground: String,
    pub system_background: String,
}
