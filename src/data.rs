use std::path::Path;

use anyhow::anyhow;
use directories::ProjectDirs;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::package::Package;

/// Container for project directories, data, and other shared state
pub struct Data {
	pub dirs: ProjectDirs,
	pub client: Client,
}

impl Data {
	pub fn new() -> anyhow::Result<Self> {
		let dirs = directories::ProjectDirs::from("worbots_setup", "4145", "worbots_setup")
			.ok_or(anyhow!("Failed to create project directories"))?;

		let out = Data {
			dirs,
			client: Client::new(),
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
