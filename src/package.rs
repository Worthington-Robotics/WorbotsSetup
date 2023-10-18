use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{data::Data, install, output::OutputTrait};

/// List of all packages
pub static ALL_PACKAGES: &[Package] = &[
	Package::AdvantageScope,
	Package::REVClient,
	Package::LimelightFinder,
	Package::GRIP,
	Package::Phoenix,
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
		}

		data.out.success("Package installed");

		Ok(())
	}
}
