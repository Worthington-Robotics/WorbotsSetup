use std::path::PathBuf;
use std::process::Command;

use crate::data::Data;
use crate::output::OutputTrait;
use crate::utils::{download_file, get_local_program};

pub async fn install(data: &mut Data<'_>) -> anyhow::Result<()> {
	let dir = get_path(data)?;
	// Download the installer
	data.out.progress("Downloading installer");
	let installer_path = dir.join("installer.exe");
	download_file(
		&data.client,
		"https://drive.google.com/uc?export=download&id=1M0O8KoP2JmWFuwO7RJNRggehF6l53jJE&confirm=t&uuid=22ead10c-923a-4d7e-b1d5-17758bc282b2&at=AB6BwCDs19_YnorJcuXHkfS2yJIW:1698016272600",
		&installer_path,
	)
	.await?;

	// Run the installer
	data.out.progress("Starting installer");
	Command::new(installer_path).spawn()?.wait()?;

	Ok(())
}

pub fn launch(_data: &mut Data<'_>) -> anyhow::Result<()> {
	let exec = get_local_program("CacheCAD", "CacheCAD_GUI.exe")?;
	// Change the cwd because program logs are local to it
	let cwd = exec.parent().expect("Parent directory missing").to_owned();
	Command::new(exec).current_dir(cwd).spawn()?;

	Ok(())
}

fn get_path(data: &Data) -> anyhow::Result<PathBuf> {
	let out = data.get_data_directory()?.join("cachecad");
	std::fs::create_dir_all(&out)?;
	Ok(out)
}
