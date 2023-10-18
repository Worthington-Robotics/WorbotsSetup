use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, download_github_release};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	data.out.progress("Getting Github release");
	let release = download_github_release(&data.client, "CrossTheRoadElec", "Phoenix-Releases")
		.await
		.context("Failed to get Github release")?;
	let asset = release
		.get_asset_pattern(".exe")
		.ok_or(anyhow!("No valid asset file found"))?;

	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(&data.client, &asset.browser_download_url, &installer_path).await?;

	// Run the installer
	data.out.progress("Starting installer");
	data.out.instruction("Click next/I agree on every option");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("phoenix");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
