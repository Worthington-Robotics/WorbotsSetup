use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_bytes, download_github_release};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	data.out.progress("Getting Github release");
	let release = download_github_release(&data.client, "mjansen4857", "pathplanner")
		.await
		.context("Failed to get Github release")?;
	let asset = release
		.get_asset_pattern("Windows.zip")
		.ok_or(anyhow!("No valid asset file found"))?;

	// Download the installer
	data.out.progress("Downloading installer");
	let bytes = download_bytes(&data.client, &asset.browser_download_url).await?;

	// Extract the installer
	let zip_path = dir.join("extracted");
	let cur = BufReader::new(Cursor::new(bytes));
	zip_extract::extract(cur, &zip_path, true).context("Failed to extract archive")?;

	Ok(())
}

pub fn launch(data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = get_path(data)?.join("extracted/pathplanner.exe");
	Command::new(exec).spawn()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("pathplanner");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
