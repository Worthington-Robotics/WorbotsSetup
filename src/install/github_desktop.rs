use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, get_local_data_dir};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;
	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(
		&data.client,
		"https://central.github.com/deployments/desktop/desktop/latest/win32",
		&installer_path,
	)
	.await?;

	// Run the installer
	data.out.progress("Starting installer");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

pub fn launch(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let gh_dir =
		get_local_data_dir("GitHubDesktop").context("Failed to get Github Desktop directory")?;
	Command::new(gh_dir.join("GitHubDesktop.exe")).spawn()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("github_desktop");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
