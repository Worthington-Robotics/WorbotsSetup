use std::{fmt::Display, str::FromStr};

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
	Package::REVClient,
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
			}
		)
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
			_ => Err(anyhow!("Unknown package type")),
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
			Self::Etcher => "Flashes OS images to drives. Used to flash the RoboRIO 2",
		}
	}
}

impl Package {
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
		}

		data.out.success("Package installed");

		Ok(())
	}

	/// Check if the package can be launched
	pub fn can_launch(&self) -> bool {
		!matches!(self, Self::Phoenix)
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
		}

		data.out.success("Package launched");

		Ok(())
	}
}
