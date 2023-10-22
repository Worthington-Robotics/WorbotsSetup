use std::fmt::{Debug, Display};
use std::str::FromStr;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::assets::{
	ADVANTAGESCOPE_ICON, CTRE_ICON, LIMELIGHT_ICON, NI_ICON, REV_ICON, WPILIB_ICON,
};
use crate::data::Data;
use crate::install;
use crate::output::OutputTrait;

/// List of all packages, organized alphabetically by display name
pub static ALL_PACKAGES: &[Package] = &[
	Package::AdvantageScope,
	Package::CacheCAD,
	Package::Phoenix,
	Package::DSLogViewer,
	Package::Etcher,
	Package::DriverStation,
	Package::GameManual,
	Package::GameTools,
	Package::GithubDesktop,
	Package::Glass,
	Package::GRIP,
	Package::LimelightFinder,
	Package::PathPlanner,
	Package::PathWeaver,
	Package::PhoenixTuner,
	Package::RadioUtility,
	Package::REVClient,
	Package::RIOImagingTool,
	Package::TaskManager,
	Package::TeamNumberSetter,
	Package::Shuffleboard,
	Package::WorbotsGithub,
	Package::WPILib,
	Package::DataLogTool,
	Package::WPILibDocs,
	Package::OutlineViewer,
	Package::RobotBuilder,
	Package::SysId,
	Package::VSCode,
];

macro_rules! define_packages {
	(
		$all_pkgs:ident,
		$((
			$id:ident,
			$name:literal,
			$display_name:literal,
			$short_desc:literal,
			$icon:expr,
			$parent:expr,
			$can_install:literal,
			$install:path,
			$can_launch:literal,
			$launch:path
			$(,)*
		));* $(;)*
	) => {
		#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
		#[serde(rename_all = "snake_case")]
		pub enum Package {
			$(
				#[serde(rename = $name)]
				$id,
			)*
		}

		impl Display for Package {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(
					f,
					"{}",
					match self {
						$(
							Self::$id => $name,
						)*
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
					$(
						$name => Ok(Self::$id),
					)*
					other => Err(anyhow!(
						"Unknown package type {other}. Must be one of {:?}", $all_pkgs
					)),
				}
			}
		}

		// Display impls
		impl Package {
			/// Get the pretty display name of the package
			pub fn display_name(&self) -> &'static str {
				match self {
					$(
						Self::$id => $display_name,
					)*
				}
			}

			/// Get the short description of the package
			pub fn short_description(&self) -> &'static str {
				match self {
					$(
						Self::$id => $short_desc,
					)*
				}
			}

			/// Get the icon of the package if it has one
			pub fn get_icon(&self) -> Option<&'static [u8]> {
				match self {
					$(
						Self::$id => $icon,
					)*
				}
			}
		}

		impl Package {
			/// Gets the parent package of this package, if it has one
			pub fn get_parent(&self) -> Option<Self> {
				match self {
					$(
						Self::$id => $parent,
					)*
				}
			}

			/// Check if the package can be installed
			pub fn can_install(&self) -> bool {
				match self {
					$(
						Self::$id => $can_install,
					)*
				}
			}

			/// Install the package
			pub async fn install(&self, data: &mut Data<'_>) -> anyhow::Result<()> {
				data.out.progress(format!("Installing package {self}"));
				match self {
					$(
						Self::$id => $install(data).await?,
					)*
				}

				data.out.success("Package installed");

				Ok(())
			}

			/// Check if the package can be launched
			pub fn can_launch(&self) -> bool {
				match self {
					$(
						Self::$id => $can_launch,
					)*
				}
			}

			/// Launch the package if it can be launched
			pub async fn launch(&self, data: &mut Data<'_>) -> anyhow::Result<()> {
				data.out.progress(format!("Launching package {self}"));
				match self {
					$(
						Self::$id => $launch(data)?,
					)*
				}

				data.out.success("Package launched");

				Ok(())
			}
		}
	};
}

define_packages! {
	ALL_PACKAGES,
	(
		AdvantageScope,
		"advantagescope",
		"AdvantageScope",
		"A viewer for live robot telemetry and log files",
		Some(ADVANTAGESCOPE_ICON),
		None,
		true,
		install::advantagescope::install,
		true,
		install::advantagescope::launch,
	);
	(
		REVClient,
		"rev_client",
		"REV Hardware Client",
		"Updater and debugger for REV devices",
		Some(REV_ICON),
		None,
		true,
		install::rev_client::install,
		true,
		install::rev_client::launch,
	);
	(
		LimelightFinder,
		"limelight_finder",
		"Limelight Finder",
		"Tool to find Limelights on the robot network",
		Some(LIMELIGHT_ICON),
		None,
		true,
		install::limelight_finder::install,
		true,
		install::limelight_finder::launch,
	);
	(
		GRIP,
		"grip",
		"GRIP",
		"A graphical vision pipeline editor",
		None,
		None,
		true,
		install::grip::install,
		true,
		install::grip::launch,
	);
	(
		Phoenix,
		"phoenix",
		"CTRE Phoenix",
		"Tools for working with CTRE devices",
		Some(CTRE_ICON),
		None,
		true,
		install::phoenix::install,
		false,
		no_launch,
	);
	(
		PathPlanner,
		"pathplanner",
		"PathPlanner",
		"Autonomous path editor and generator",
		None,
		None,
		true,
		install::pathplanner::install,
		true,
		install::pathplanner::launch,
	);
	(
		GithubDesktop,
		"github_desktop",
		"GitHub Desktop",
		"Desktop app for GitHub, a website used to host robot code",
		None,
		None,
		true,
		install::github_desktop::install,
		true,
		install::github_desktop::launch,
	);
	(
		Etcher,
		"etcher",
		"Etcher",
		"Flashes OS images to drives. Used to flash the roboRIO 2",
		None,
		None,
		true,
		install::etcher::install,
		true,
		install::etcher::launch,
	);
	(
		PhoenixTuner,
		"phoenix_tuner",
		"Phoenix Tuner",
		"Allows viewing, debugging, and configuration of devices on a CAN network",
		Some(CTRE_ICON),
		Some(Self::Phoenix),
		false,
		no_install,
		true,
		install::phoenix::launch_phoenix_tuner,
	);
	(
		WPILib,
		"wpilib",
		"WPILib",
		"The official tools for developing FRC robots",
		Some(WPILIB_ICON),
		None,
		true,
		install::wpilib::install,
		false,
		no_launch,
	);
	(
		VSCode,
		"vscode",
		"WPILib VSCode",
		"Microsoft's code editor configured for robot development",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_vscode,
	);
	(
		DataLogTool,
		"data_log_tool",
		"WPILib Data Log Tool",
		"Viewer for robot log files",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_data_log_tool,
	);
	(
		TeamNumberSetter,
		"team_number_setter",
		"roboRIO Team Number Setter",
		"Simple tool used to set the team number on a roboRIO",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_team_number_setter,
	);
	(
		Glass,
		"glass",
		"Glass",
		"Dashboard for drivers and simulation",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_glass,
	);
	(
		OutlineViewer,
		"outline_viewer",
		"WPILib Outline Viewer",
		"A simple NetworkTables editor",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_outline_viewer,
	);
	(
		PathWeaver,
		"pathweaver",
		"PathWeaver",
		"The WPILib autonomous path editor",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_pathweaver,
	);
	(
		Shuffleboard,
		"shuffleboard",
		"Shuffleboard",
		"An interactive and customizable robot dashboard",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_shuffleboard,
	);
	(
		SysId,
		"sysid",
		"WPILib SysId",
		"A tool for analyzing and tuning robot control systems",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_sysid,
	);
	(
		RobotBuilder,
		"robotbuilder",
		"WPILib RobotBuilder",
		"Tool for generating WPILib robot projects",
		Some(WPILIB_ICON),
		Some(Self::WPILib),
		false,
		no_install,
		true,
		install::wpilib::launch_robotbuilder,
	);
	(
		GameTools,
		"game_tools",
		"FRC Game Tools",
		"Official tools for the robot, including the radio utility and driver station",
		Some(NI_ICON),
		None,
		true,
		install::game_tools::install,
		false,
		no_launch,
	);
	(
		DriverStation,
		"driver_station",
		"FRC Driver Station",
		"The station for drivers used to control the robot",
		Some(NI_ICON),
		Some(Self::GameTools),
		false,
		no_install,
		true,
		install::game_tools::launch_driver_station,
	);
	(
		DSLogViewer,
		"ds_log_viewer",
		"DS Log File Viewer",
		"A tool to view Driver Station log files",
		Some(NI_ICON),
		Some(Self::GameTools),
		false,
		no_install,
		true,
		install::game_tools::launch_ds_log_viewer,
	);
	(
		RadioUtility,
		"radio_utility",
		"FRC Radio Configuration Utility",
		"Flash and configure radio devices",
		Some(NI_ICON),
		Some(Self::GameTools),
		false,
		no_install,
		true,
		install::game_tools::launch_radio_utility,
	);
	(
		RIOImagingTool,
		"rio_imaging_tool",
		"roboRIO Imaging Tool",
		"Flash and update the roboRIO",
		Some(NI_ICON),
		Some(Self::GameTools),
		false,
		no_install,
		true,
		install::game_tools::launch_rio_imaging_tool,
	);
	(
		WPILibDocs,
		"wpilib_docs",
		"WPILib Docs",
		"Documentation for WPILib and developing FRC robots",
		Some(WPILIB_ICON),
		None,
		false,
		no_install,
		true,
		install::shortcuts::open_wpilib_docs,
	);
	(
		WorbotsGithub,
		"worbots_github",
		"WorBots GitHub",
		"GitHub organization for the WorBots team",
		None,
		None,
		false,
		no_install,
		true,
		install::shortcuts::open_worbots_github,
	);
	(
		GameManual,
		"game_manual",
		"FRC Game Manual",
		"Official manual for the FRC game",
		None,
		None,
		false,
		no_install,
		true,
		install::shortcuts::open_game_manual,
	);
	(
		CacheCAD,
		"cachecad",
		"CacheCAD",
		"A file management interface for Google Drive",
		None,
		None,
		true,
		install::cachecad::install,
		true,
		install::cachecad::launch,
	);
	(
		TaskManager,
		"task_manager",
		"Task Manager",
		"The Windows Task Manager",
		None,
		None,
		false,
		no_install,
		true,
		install::misc::launch_task_manager,
	);
}

async fn no_install(_: &mut Data<'_>) -> anyhow::Result<()> {
	Ok(())
}

fn no_launch(_: &mut Data<'_>) -> anyhow::Result<()> {
	Ok(())
}
