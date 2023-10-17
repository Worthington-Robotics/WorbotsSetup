use anyhow::Context;
use clap::Parser;
use color_print::{cformat, cprintln};
use data::Data;
use package::Package;

mod assets;
mod data;
mod install;
mod package;
mod utils;

#[tokio::main]
async fn main() {
	let result = run_cli().await;
	if let Err(e) = result {
		eprintln!("{}", cformat!("{e:?}"));
	}
}

async fn run_cli() -> anyhow::Result<()> {
	let cli = Cli::parse();
	let mut data = Data::new().context("Failed to create application data")?;
	match cli.command {
		Subcommand::Install { packages } => {
			install_packages(packages, &mut data).await?;
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
			)
			.await?;
		}
	}

	Ok(())
}

async fn install_packages(packages: Vec<Package>, data: &mut Data) -> anyhow::Result<()> {
	for package in packages {
		package.install(data).await?;
	}
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
	/// Installs a package
	Install {
		/// The names of the packages to install
		packages: Vec<Package>,
	},
	/// Installs all available packages
	InstallAll,
}
