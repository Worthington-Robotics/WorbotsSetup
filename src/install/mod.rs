pub mod advantagescope;
pub mod etcher;
pub mod game_tools;
pub mod github_desktop;
pub mod grip;
pub mod limelight_finder;
pub mod pathplanner;
pub mod phoenix;
pub mod rev_client;
pub mod wpilib;

pub mod shortcuts {
	use std::process::Command;

	use crate::data::Data;

	pub fn open_wpilib_docs(_data: &mut Data) -> anyhow::Result<()> {
		open_shortcut("https://docs.wpilib.org/en/stable/index.html")
	}

	fn open_shortcut(url: &str) -> anyhow::Result<()> {
		Command::new("cmd.exe").arg("/C").arg("start").arg(&url).spawn()?.wait()?;
		Ok(())
	}
}
