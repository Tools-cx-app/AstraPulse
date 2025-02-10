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

use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
use chrono::{DateTime, Utc, FixedOffset};

use std::{
    error::Error,
    io::{self, Write},
};

fn log_format(write: &mut dyn Write, now: &mut DeferredNow, record: &Record<'_>) -> io::Result<()> {
    let utc_time: DateTime<Utc> = now.now_utc_owned();
    let beijing_time = utc_time.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    write!(write, "[{beijing_time} {}] {}", record.level(), record.args())?;
    Ok(())
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let spec = LogSpecification::trace();
    Logger::with(spec)
        .log_to_stdout()
        .format(log_format)
        .start()
        .map_err(|e| format!("无法启动日志系统: {e}"))?;
    Ok(())
}
