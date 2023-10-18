use std::{
	fmt::{Debug, Display},
	str::FromStr,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{data::Data, install, output::OutputTrait};

/// List of all packages, organized alphabetically by display name
pub static ALL_PACKAGES: &[Package] = &[
	Package::AdvantageScope,
	Package::Phoenix,
	Package::Etcher,
	Package::GithubDesktop,
	Package::GRIP,
	Package::LimelightFinder,
	Package::PathPlanner,
	Package::PhoenixTuner,
	Package::REVClient,
	Package::TeamNumberSetter,
	Package::WPILib,
	Package::VSCode,
	Package::DataLogTool,
];

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Package {
	#[serde(rename = "advantagescope")]
	AdvantageScope,
	#[serde(rename = "rev_client")]
	REVClient,
	LimelightFinder,
	#[serde(rename = "grip")]
	GRIP,
	Phoenix,
	#[serde(rename = "pathplanner")]
	PathPlanner,
	GithubDesktop,
	Etcher,
	PhoenixTuner,
	#[serde(rename = "wpilib")]
	WPILib,
	#[serde(rename = "vscode")]
	VSCode,
	DataLogTool,
	TeamNumberSetter,
}

impl Display for Package {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::AdvantageScope => "advantagescope",
				Self::REVClient => "rev_client",
				Self::LimelightFinder => "limelight_finder",
				Self::GRIP => "grip",
				Self::Phoenix => "phoenix",
				Self::PathPlanner => "pathplanner",
				Self::GithubDesktop => "github_desktop",
				Self::Etcher => "etcher",
				Self::PhoenixTuner => "phoenix_tuner",
				Self::WPILib => "wpilib",
				Self::VSCode => "vscode",
				Self::DataLogTool => "data_log_tool",
				Self::TeamNumberSetter => "team_number_setter",
			}
		)
	}
}

impl Debug for Package {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as Display>::fmt(&self, f)
	}
}

impl FromStr for Package {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"advantagescope" => Ok(Self::AdvantageScope),
			"rev_client" => Ok(Self::REVClient),
			"limelight_finder" => Ok(Self::LimelightFinder),
			"grip" => Ok(Self::GRIP),
			"phoenix" => Ok(Self::Phoenix),
			"path_planner" => Ok(Self::PathPlanner),
			"github_desktop" => Ok(Self::GithubDesktop),
			"etcher" => Ok(Self::Etcher),
			"phoenix_tuner" => Ok(Self::Etcher),
			"wpilib" => Ok(Self::WPILib),
			"vscode" => Ok(Self::VSCode),
			"data_log_tool" => Ok(Self::DataLogTool),
			"team_number_setter" => Ok(Self::TeamNumberSetter),
			other => Err(anyhow!(
				"Unknown package type {other}. Must be one of {ALL_PACKAGES:?}"
			)),
		}
	}
}

// Display impls
impl Package {
	/// Get the pretty display name of the package
	pub fn display_name(&self) -> &'static str {
		match self {
			Self::AdvantageScope => "AdvantageScope",
			Self::REVClient => "REV Hardware Client",
			Self::LimelightFinder => "Limelight Finder",
			Self::GRIP => "GRIP",
			Self::Phoenix => "CTRE Phoenix",
			Self::PathPlanner => "PathPlanner",
			Self::GithubDesktop => "GitHub Desktop",
			Self::Etcher => "Etcher",
			Self::PhoenixTuner => "Phoenix Tuner",
			Self::WPILib => "WPILib",
			Self::VSCode => "WPILib VSCode",
			Self::DataLogTool => "WPILib Data Log Tool",
			Self::TeamNumberSetter => "roboRIO Team Number Setter",
		}
	}

	/// Get the short description of the package
	pub fn short_description(&self) -> &'static str {
		match self {
			Self::AdvantageScope => "A viewer for live robot telemetry and log files",
			Self::REVClient => "Updater and debugger for REV devices",
			Self::LimelightFinder => "Tool to find Limelights on the robot network",
			Self::GRIP => "A graphical vision pipeline editor",
			Self::Phoenix => "Tools for working with CTRE devices",
			Self::PathPlanner => "Autonomous path editor and generator",
			Self::GithubDesktop => "Desktop app for GitHub, a website used to host robot code",
			Self::Etcher => "Flashes OS images to drives. Used to flash the roboRIO 2",
			Self::PhoenixTuner => {
				"Allows viewing, debugging, and configuration of devices on a CAN network"
			}
			Self::WPILib => "The official tools for developing FRC robots",
			Self::VSCode => "Microsoft's code editor configured for robot development",
			Self::DataLogTool => "Viewer for robot log files",
			Self::TeamNumberSetter => "Simple tool used to set the team number on a roboRIO",
		}
	}
}

impl Package {
	/// Gets the parent package of this package, if it has one
	pub fn get_parent(&self) -> Option<Package> {
		match self {
			Self::PhoenixTuner => Some(Self::Phoenix),
			Self::VSCode | Self::DataLogTool | Self::TeamNumberSetter => Some(Self::WPILib),
			_ => None,
		}
	}

	/// Check if the package can be installed
	pub fn can_install(&self) -> bool {
		!matches!(
			self,
			Self::PhoenixTuner | Self::VSCode | Self::DataLogTool | Self::TeamNumberSetter
		)
	}

	/// Install the package
	pub async fn install(&self, data: &mut Data<'_>) -> anyhow::Result<()> {
		data.out.progress(format!("Installing package {self}"));
		match self {
			Self::AdvantageScope => install::advantagescope::install(data).await?,
			Self::REVClient => install::rev_client::install(data).await?,
			Self::LimelightFinder => install::limelight_finder::install(data).await?,
			Self::GRIP => install::grip::install(data).await?,
			Self::Phoenix => install::phoenix::install(data).await?,
			Self::PathPlanner => install::pathplanner::install(data).await?,
			Self::GithubDesktop => install::github_desktop::install(data).await?,
			Self::Etcher => install::etcher::install(data).await?,
			Self::PhoenixTuner => {}
			Self::WPILib => install::wpilib::install(data).await?,
			Self::VSCode => {}
			Self::DataLogTool => {}
			Self::TeamNumberSetter => {}
		}

		data.out.success("Package installed");

		Ok(())
	}

	/// Check if the package can be launched
	pub fn can_launch(&self) -> bool {
		!matches!(self, Self::Phoenix | Self::WPILib)
	}

	/// Launch the package if it can be launched
	pub async fn launch(&self, data: &mut Data<'_>) -> anyhow::Result<()> {
		data.out.progress(format!("Launching package {self}"));
		match self {
			Self::AdvantageScope => install::advantagescope::launch(data)?,
			Self::REVClient => install::rev_client::launch(data)?,
			Self::LimelightFinder => install::limelight_finder::launch(data)?,
			Self::GRIP => install::grip::launch(data)?,
			Self::Phoenix => {}
			Self::PathPlanner => install::pathplanner::launch(data)?,
			Self::GithubDesktop => install::github_desktop::launch(data)?,
			Self::Etcher => install::etcher::launch(data)?,
			Self::PhoenixTuner => install::phoenix::launch_phoenix_tuner(data)?,
			Self::WPILib => {}
			Self::VSCode => install::wpilib::launch_vscode(data)?,
			Self::DataLogTool => install::wpilib::launch_data_log_tool(data)?,
			Self::TeamNumberSetter => install::wpilib::launch_team_number_setter(data)?,
		}

		data.out.success("Package launched");

		Ok(())
	}
}
