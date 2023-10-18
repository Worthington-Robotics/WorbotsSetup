use std::path::Path;

use anyhow::anyhow;
use directories::ProjectDirs;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{output::Output, package::Package};

/// Container for project directories, data, and other shared state
pub struct Data<'o> {
	pub dirs: ProjectDirs,
	pub client: Client,
	pub out: &'o mut Output,
}

impl<'o> Data<'o> {
	pub fn new(out: &'o mut Output) -> anyhow::Result<Self> {
		let dirs = directories::ProjectDirs::from("worbots_setup", "4145", "worbots_setup")
			.ok_or(anyhow!("Failed to create project directories"))?;

		let out = Data {
			dirs,
			client: Client::new(),
			out,
		};

		Ok(out)
	}

	pub fn get_data_directory(&self) -> anyhow::Result<&Path> {
		let out = self.dirs.data_dir();
		std::fs::create_dir_all(out)?;
		Ok(out)
	}
}

/// Persistent data for the tool
#[derive(Serialize, Deserialize)]
pub struct PersistentData {
	pub installed_packages: Vec<Package>,
}
