#![cfg_attr(release, windows_subsystem = "windows")]

use anyhow::Context;
use clap::Parser;
use color_print::{cformat, cprintln};
use data::Data;
use output::CommonOutput;
use package::Package;
use ui::start_app;

use crate::utils::tokio_exec;

mod assets;
mod data;
mod install;
mod output;
mod package;
mod ui;
mod utils;

#[cfg(not(target_os = "windows"))]
compile_error!("This tool is Windows-only");

fn main() {
	let result = run_cli();
	if let Err(e) = result {
		eprintln!("{}", cformat!("{e:?}"));
	}
}

fn run_cli() -> anyhow::Result<()> {
	let cli = Cli::parse();
	let mut out = CommonOutput;
	let mut data = Data::new(&mut out).context("Failed to create application data")?;
	match cli.command {
		Subcommand::App => {
			println!("Starting app");
			start_app().context("Failed to start app")?;
		}
		Subcommand::Install { packages } => {
			install_packages(packages, &mut data)?;
		}
		Subcommand::InstallAll => {
			install_packages(
				vec![
					Package::Phoenix,
					Package::REVClient,
					Package::AdvantageScope,
					Package::GRIP,
					Package::LimelightFinder,
				],
				&mut data,
			)?;
		}
	}

	Ok(())
}

fn install_packages(packages: Vec<Package>, data: &mut Data) -> anyhow::Result<()> {
	tokio_exec(async {
		for package in packages {
			package.install(data).await?;
		}
		Ok::<(), anyhow::Error>(())
	})??;
	cprintln!("<s,g>All packages installed");
	Ok(())
}

#[derive(Parser)]
struct Cli {
	#[clap(subcommand)]
	command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
	/// Opens the graphical application
	App,
	/// Installs a package
	Install {
		/// The names of the packages to install
		packages: Vec<Package>,
	},
	/// Installs all available packages
	InstallAll,
}
