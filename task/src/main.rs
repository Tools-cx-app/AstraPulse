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
    fs::{self},
    path::{Path, PathBuf},
    process::{self, Command},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use fs_extra::{dir, file};
use zip::{CompressionMethod, write::FileOptions};
use zip_extensions::zip_create_from_directory_with_options;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[clap(short, long, default_value = "false")]
        release: bool,
        #[clap(short, long, default_value = "false")]
        verbose: bool,
    },
    Build {
        #[clap(short, long, default_value = "false")]
        release: bool,
        #[clap(short, long, default_value = "false")]
        verbose: bool,
    },
    Clean,
    Format {
        #[clap(short, long, default_value = "false")]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        eprintln!("无可用命令，请 --help查看");
        process::exit(1);
    };

    match command {
        Commands::Check { release, verbose } => {
            check(release, verbose)?;
        }
        Commands::Build { release, verbose } => {
            build(release, verbose)?;
        }
        Commands::Clean => {
            clean()?;
        }
        Commands::Format { verbose } => {
            format(verbose)?;
        }
    }

    Ok(())
}

fn build(release: bool, verbose: bool) -> Result<()> {
    let temp_dir = temp_dir(release);

    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)?;

    let mut cargo = cargo_ndk();
    cargo.args(["build", "--target", "aarch64-linux-android"]);

    if release {
        cargo.arg("--release");
    }

    if verbose {
        cargo.arg("--verbose");
    }

    cargo.spawn()?.wait()?;

    let module_dir = module_dir();
    dir::copy(
        &module_dir,
        &temp_dir,
        &dir::CopyOptions::new().overwrite(true).content_only(true),
    )
    .unwrap();
    // fs::remove_file(temp_dir.join()).unwrap();
    file::copy(
        bin_path(release),
        temp_dir.join("AstraPulse"),
        &file::CopyOptions::new().overwrite(true),
    )
    .unwrap();
    file::copy(
        "README.md",
        temp_dir.join("README.md"),
        &file::CopyOptions::new().overwrite(true),
    )
    .unwrap();

    let build_type = if release { "release" } else { "debug" };
    let package_path = Path::new("output").join(format!("AstraPulse-({build_type}).zip"));

    let options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));
    zip_create_from_directory_with_options(&package_path, &temp_dir, |_| options).unwrap();

    println!("AstraPulse编译成功: {:?}", package_path);

    Ok(())
}

fn check(release: bool, verbose: bool) -> Result<()> {
    let mut cargo = cargo_ndk();
    cargo.args(["check", "--target", "aarch64-linux-android"]);
    cargo.env("RUSTFLAGS", "-C default-linker-libraries");

    if release {
        cargo.arg("--release");
    }

    if verbose {
        cargo.arg("--verbose");
    }

    cargo.spawn()?.wait()?;

    Ok(())
}

fn clean() -> Result<()> {
    let temp_dir = temp_dir(false);
    let _ = fs::remove_dir_all(&temp_dir);

    Command::new("cargo").arg("clean").spawn()?.wait()?;

    Ok(())
}

fn format(verbose: bool) -> Result<()> {
    let mut command = Command::new("cargo");
    command.args(["fmt", "--all"]);
    if verbose {
        command.arg("--verbose");
    }
    command.spawn()?.wait()?;

    Ok(())
}

fn module_dir() -> PathBuf {
    Path::new("modules").to_path_buf()
}

fn temp_dir(release: bool) -> PathBuf {
    Path::new("output")
        .join(".temp")
        .join(if release { "release" } else { "debug" })
}

fn bin_path(release: bool) -> PathBuf {
    Path::new("target")
        .join("aarch64-linux-android")
        .join(if release { "release" } else { "debug" })
        .join("AstraPulse")
}

fn cargo_ndk() -> Command {
    let mut command = Command::new("cargo");
    command.args(["ndk", "-t", "arm64-v8a"]);
    command.env("RUSTFLAGS", "-C default-linker-libraries");
    command.env("CARGO_CFG_BPF_TARGET_ARCH", "aarch64");
    command
}
