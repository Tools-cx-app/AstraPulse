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

use std::{fs, os::unix::fs::PermissionsExt};

use anyhow::{Context, Result};

pub fn write(path: &str, context: &str) -> Result<()> {
    fs::set_permissions(path, fs::Permissions::from_mode(0o644))
        .context("😂无法设置{path}的权限")?;
    fs::write(path, context).context("😂无法写入{path}")?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o400))
        .context("😂无法设置{path}的权限")?;
    Ok(())
}

pub fn read(path: &str) -> Result<String> {
    let context = fs::read_to_string(path).context("😂无法读取{path}")?;
    Ok(context)
}
