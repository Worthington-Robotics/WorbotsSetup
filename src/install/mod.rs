pub mod advantagescope;
pub mod cachecad;
pub mod etcher;
pub mod game_tools;
pub mod github_desktop;
pub mod grip;
pub mod limelight_finder;
pub mod pathplanner;
pub mod phoenix;
pub mod rev_client;
pub mod wpilib;

pub mod misc {
	use std::path::PathBuf;

	use crate::{data::Data, utils::run_elevated};

	pub fn launch_task_manager(_data: &mut Data<'_>) -> anyhow::Result<()> {
		let exec = PathBuf::from("C:/WINDOWS/system32/Taskmgr.exe");
		run_elevated(exec)?.spawn()?.wait()?;
		Ok(())
	}
}

pub mod shortcuts {
	use std::process::Command;

	use crate::data::Data;

	pub fn open_wpilib_docs(_data: &mut Data) -> anyhow::Result<()> {
		open_shortcut("https://docs.wpilib.org/en/stable/index.html")
	}

	pub fn open_worbots_github(_data: &mut Data) -> anyhow::Result<()> {
		open_shortcut("https://github.com/Worthington-Robotics")
	}

	pub fn open_game_manual(_data: &mut Data) -> anyhow::Result<()> {
		open_shortcut("https://firstfrc.blob.core.windows.net/frc2023/Manual/2023FRCGameManual.pdf")
	}

	fn open_shortcut(url: &str) -> anyhow::Result<()> {
		Command::new("cmd.exe")
			.arg("/C")
			.arg("start")
			.arg(&url)
			.spawn()?
			.wait()?;
		Ok(())
	}
}
