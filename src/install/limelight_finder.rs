use std::path::PathBuf;
use std::process::Command;

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::download_file;

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;
	// Download the program
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(
		&data.client,
		"https://downloads.limelightvision.io/software/LimelightFinderSetup1_0_1.exe",
		&installer_path,
	)
	.await?;

	// Run the installer
	data.out.progress("Starting installer");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("limelight_finder");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
