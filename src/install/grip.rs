use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, download_github_release, get_local_data_dir};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	data.out.progress("Getting Github release");
	let release = download_github_release(&data.client, "WPIRoboticsProjects", "GRIP")
		.await
		.context("Failed to get Github release")?;
	let asset = release
		.get_asset_pattern("x64.exe")
		.ok_or(anyhow!("No valid asset file found"))?;

	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(&data.client, &asset.browser_download_url, &installer_path).await?;

	// Run the installer
	data.out.progress("Starting installer");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

pub fn launch(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let grip_dir = get_local_data_dir("GRIP").context("Failed to get GRIP directory")?;
	Command::new(grip_dir.join("GRIP.exe")).spawn()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("grip");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
