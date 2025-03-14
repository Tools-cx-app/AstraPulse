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

use std::{ffi::CString, fs, os::unix::fs::PermissionsExt, ptr};

use anyhow::{Context, Result};
use libc::{MS_BIND, MS_REC, mount, umount, umount2};

pub fn write(path: &str, context: &str) -> Result<()> {
    fs::set_permissions(path, fs::Permissions::from_mode(0o644))
        .context(format!("😂无法设置{path}的权限"))?;
    fs::write(path, context).context(format!("😂无法写入{path}"))?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o400))
        .context(format!("😂无法设置{path}的权限"))?;
    Ok(())
}

pub fn read(path: &str) -> Result<String> {
    let context = fs::read_to_string(path).context(format!("😂无法读取{path}"))?;
    Ok(context)
}

pub fn lock_value(value: &str, path: Vec<&str>) -> Result<()> {
    for p in path {
        if fs::metadata(p).is_ok() {
            let mount_path = format!("/cache/mount_mask_{value}");
            unmount(&p)?;
            fs::set_permissions(p, fs::Permissions::from_mode(0o644))
                .context(format!("😂无法设置{p}的权限"))?;
            fs::write(p, value).context(format!("😂无法写入{p}"))?;
            fs::set_permissions(p, fs::Permissions::from_mode(0o400))
                .context(format!("😂无法设置{p}的权限"))?;
            fs::write(&mount_path, value).context(format!("😂无法写入{mount_path}"))?;
            mount_bind(&mount_path, &p)?;
        }
    }
    Ok(())
}

fn mount_bind(src_path: &str, dest_path: &str) -> Result<()> {
    let src_path = CString::new(src_path)?;
    let dest_path = CString::new(dest_path)?;

    unsafe {
        umount2(dest_path.as_ptr(), libc::MNT_DETACH);

        if mount(
            src_path.as_ptr().cast(),
            dest_path.as_ptr().cast(),
            ptr::null(),
            MS_BIND | MS_REC,
            ptr::null(),
        ) != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
    }

    Ok(())
}

fn unmount(file_system: &str) -> Result<()> {
    let path = CString::new(file_system)?;
    if unsafe { umount(path.as_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}
