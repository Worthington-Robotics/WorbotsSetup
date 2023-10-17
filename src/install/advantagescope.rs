use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};
use directories::ProjectDirs;

use crate::assets;
use crate::data::Data;
use crate::utils::{download_file, download_github_release, print_progress};

pub async fn install(data: &mut Data) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	print_progress("Getting Github release");
	let release = download_github_release(&data.client, "Mechanical-Advantage", "AdvantageScope")
		.await
		.context("Failed to get Github release")?;
	let asset = release
		.get_asset_pattern("win-x64")
		.ok_or(anyhow!("No valid asset file found"))?;

	// Download the installer
	print_progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(&data.client, &asset.browser_download_url, &installer_path).await?;

	// Run the installer
	print_progress("Starting installer");
	Command::new(installer_path).spawn()?.wait()?;

	// Configure AdvantageScope
	print_progress("Finished installer. Configuring");
	configure().context("Failed to configure")?;

	Ok(())
}

fn configure() -> anyhow::Result<()> {
	let as_dir = ProjectDirs::from("", "", "AdvantageScope")
		.ok_or(anyhow!("Failed to get AdvantageScope directories"))?
		.data_dir()
		.parent()
		.ok_or(anyhow!("Failed to get AdvantageScope dir"))?
		.to_owned();
	std::fs::create_dir_all(&as_dir)?;
	let frc_data_dir = as_dir.join("frcData");
	std::fs::create_dir_all(&frc_data_dir)?;

	// Create the user preferences
	let prefs = r#"{
		"theme": "system",
		"rioAddress": "10.41.45.2",
		"rioPath": "/media/sda1/",
		"liveMode": "nt4",
		"liveSubscribeMode": "low-bandwidth",
		"rlogPort": 5800,
		"threeDimensionMode": "quality"
	}"#;
	std::fs::write(as_dir.join("prefs.json"), prefs)
		.context("Failed to write AdvantageScope preferences")?;

	// Add the joystick
	std::fs::write(
		frc_data_dir.join("Joystick_Extreme3DPro.json"),
		assets::EXTREME_3D_PRO_CONFIG,
	)
	.context("Failed to create joystick config")?;
	std::fs::write(
		frc_data_dir.join("Joystick_Extreme3DPro.png"),
		assets::EXTREME_3D_PRO_IMAGE,
	)
	.context("Failed to create joystick image")?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("advantagescope");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
