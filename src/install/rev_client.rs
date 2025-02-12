use std::path::PathBuf;
use std::process::Command;

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, get_github_releases, run_elevated};

use anyhow::anyhow;

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	data.out.progress("Getting Github release");
	let releases =
		get_github_releases(&data.client, "REVrobotics", "REV-Software-Binaries").await?;
	let asset = releases
		.iter()
		.find_map(|x| {
			if x.tag_name.contains("rhc") {
				// We want the release with all of the bundled offline FRC firmware
				x.get_asset_pattern("FRC")
			} else {
				None
			}
		})
		.ok_or(anyhow!("No valid release found"))?;

	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(&data.client, &asset.browser_download_url, &installer_path).await?;

	// Run the installer
	data.out.progress("Starting installer");
	let mut cmd = run_elevated(installer_path)?;
	cmd.spawn()?;
	data.out
		.instruction("The installer has started. Follow the steps it gives you");
	data.out.continue_prompt();

	Ok(())
}

pub fn launch(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = PathBuf::from(
		"C:/Program Files (x86)/REV Robotics/REV Hardware Client/REV Hardware Client.exe",
	);
	Command::new(exec).spawn()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("rev_client");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
