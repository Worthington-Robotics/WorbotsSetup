use std::path::PathBuf;
use std::process::Command;

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, run_elevated};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(
		&data.client,
		"https://download.ni.com/support/nipkg/products/ni-f/ni-frc-2023-game-tools/23.1/online/ni-frc-2023-game-tools_23.1_online.exe",
		&installer_path,
	)
	.await?;

	// Run the installer
	data.out.progress("Starting installer");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

pub fn launch_driver_station(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = PathBuf::from("C:/Program Files (x86)/FRC Driver Station/DriverStation.exe");
	let mut cmd = run_elevated(exec)?;
	cmd.spawn()?;
	Ok(())
}

pub fn launch_ds_log_viewer(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = PathBuf::from("C:/Program Files (x86)/FRC Driver Station/DS_LogFileViewer.exe");
	Command::new(exec).spawn()?;
	Ok(())
}

pub fn launch_radio_utility(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = PathBuf::from("C:/Program Files (x86)/FRC Radio Configuration Utility/FRC Radio Configuration Utility.exe");
	let mut cmd = run_elevated(exec)?;
	cmd.spawn()?;
	Ok(())
}

pub fn launch_rio_imaging_tool(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = PathBuf::from("C:/Program Files (x86)/National Instruments/LabVIEW 2020/project/roboRIO Tool/roboRIO_ImagingTool.exe");
	Command::new(exec).spawn()?;
	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("game_tools");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
