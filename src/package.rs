use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use color_print::cprintln;
use serde::{Deserialize, Serialize};

use crate::{data::Data, install, utils::print_progress};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Package {
	#[serde(rename = "advantagescope")]
	AdvantageScope,
	#[serde(rename = "rev_client")]
	REVClient,
}

impl Display for Package {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::AdvantageScope => "advantagescope",
				Self::REVClient => "rev_client",
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
			_ => Err(anyhow!("Unknown package type")),
		}
	}
}

impl Package {
	/// Install the package
	pub async fn install(&self, data: &mut Data) -> anyhow::Result<()> {
		print_progress(&format!("Installing package {self}"));
		match self {
			Self::AdvantageScope => install::advantagescope::install(data).await?,
			Self::REVClient => install::rev_client::install(data).await?,
		}

		cprintln!("<g>Package installed");

		Ok(())
	}
}
