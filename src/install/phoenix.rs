use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};
use color_print::cprintln;

use crate::data::Data;
use crate::utils::{download_file, download_github_release, print_progress};

pub async fn install(data: &mut Data) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	print_progress("Getting Github release");
	let release = download_github_release(&data.client, "CrossTheRoadElec", "Phoenix-Releases")
		.await
		.context("Failed to get Github release")?;
	let asset = release
		.get_asset_pattern(".exe")
		.ok_or(anyhow!("No valid asset file found"))?;

	// Download the installer
	print_progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(&data.client, &asset.browser_download_url, &installer_path).await?;

	// Run the installer
	print_progress("Starting installer");
	cprintln!("<s>Instructions: click next/I agree on every option");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("phoenix");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
