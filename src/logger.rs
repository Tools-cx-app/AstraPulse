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

use std::io;

use chrono::FixedOffset;
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};

fn log_format(
    write: &mut dyn io::Write,
    now: &mut DeferredNow,
    record: &Record<'_>,
) -> anyhow::Result<(), io::Error> {
    let utc_time = now.now();
    let time_offset = FixedOffset::east_opt(8 * 3600).unwrap();
    let time = utc_time.with_timezone(&time_offset);
    let time_format = time.format("%Y-%m-%d %H:%M:%S");
    write!(
        write,
        "[{time_format}] {}: {}",
        record.level(),
        record.args()
    )
}

pub fn log_init() -> anyhow::Result<()> {
    let logger_spec = LogSpecification::info();
    Logger::with(logger_spec)
        .log_to_stdout()
        .format(log_format)
        .start()?;
    log::info!(
        "fas-rs v{} {}, llvm-{}, rustc-{}, build by {} at {} on {},{},{}",
        env!("CARGO_PKG_VERSION"),
        build_type(),
        env!("VERGEN_RUSTC_LLVM_VERSION"),
        env!("VERGEN_RUSTC_SEMVER"),
        env!("VERGEN_SYSINFO_USER"),
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_SYSINFO_NAME"),
        env!("VERGEN_SYSINFO_OS_VERSION"),
        env!("VERGEN_RUSTC_HOST_TRIPLE")
    );
    Ok(())
}

const fn build_type() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
