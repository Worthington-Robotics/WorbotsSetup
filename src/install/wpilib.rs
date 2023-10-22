use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, get_github_releases};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;

	data.out.progress("Getting Github release");
	let releases = get_github_releases(&data.client, "wpilibsuite", "allwpilib")
		.await
		.context("Failed to get Github releases")?;
	let asset = releases
		.iter()
		.find_map(|x| {
			// We only want 2023 releases
			if x.tag_name.contains("2023") {
				x.get_asset_pattern("Windows")
			} else {
				None
			}
		})
		.ok_or(anyhow!("No valid release found"))?;

	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.iso");
	download_file(&data.client, &asset.browser_download_url, &installer_path).await?;

	// Extract the installer
	data.out.progress("Extracting installer");
	let powershell_cmd = format!(
		"$mountResult = Mount-DiskImage -ImagePath '{}' -PassThru; ($mountResult | Get-Volume).DriveLetter",
		installer_path
			.to_str()
			.ok_or(anyhow!("Cannot convert path to string"))?
	);
	let output = Command::new("powershell.exe")
		.arg("-command")
		.arg(powershell_cmd)
		.output()?;
	let drive_letter =
		String::from_utf8(output.stdout).context("Failed to convert drive letter to UTF-8")?;
	// Remove the newline from the end
	let drive_letter = drive_letter.strip_suffix("\r\n").unwrap_or(&drive_letter);

	// Run the installer
	data.out.progress("Starting installer");
	let installer_path = PathBuf::from(format!("{drive_letter}:/WPILibInstaller.exe"));
	dbg!(&installer_path);
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

pub fn launch_vscode(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = PathBuf::from("C:/Users/Public/wpilib/2023/vscode/Code.exe");
	Command::new(exec).spawn()?;
	Ok(())
}

pub fn launch_data_log_tool(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("DataLogTool.vbs")
}

pub fn launch_team_number_setter(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("roboRIOTeamNumberSetter.vbs")
}

pub fn launch_glass(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("Glass.vbs")
}

pub fn launch_outline_viewer(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("OutlineViewer.vbs")
}

pub fn launch_pathweaver(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("PathWeaver.vbs")
}

pub fn launch_shuffleboard(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("Shuffleboard.vbs")
}

pub fn launch_sysid(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("SysId.vbs")
}

pub fn launch_robotbuilder(_data: &mut Data<'_>) -> anyhow::Result<()> {
	launch_vbs_tool("RobotBuilder.vbs")
}

fn launch_vbs_tool(tool: &str) -> anyhow::Result<()> {
	let exec = PathBuf::from(format!("C:/Users/Public/wpilib/2023/tools/{tool}"));
	Command::new("cscript").arg(exec).spawn()?;
	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("wpilib");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
