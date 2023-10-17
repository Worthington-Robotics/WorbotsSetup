use std::path::Path;

use anyhow::Context;
use color_print::cprintln;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

/// Wrapper around Client::get
pub async fn download(
	client: &Client,
	url: impl reqwest::IntoUrl,
) -> anyhow::Result<reqwest::Response> {
	let out = client
		.get(url)
		.header("User-Agent", "Worbots 4145 Setup Tool")
		.send()
		.await
		.context("Failed to download")?
		.error_for_status()?;
	Ok(out)
}

/// Download a file to a path
pub async fn download_file(
	client: &Client,
	url: impl reqwest::IntoUrl,
	path: &Path,
) -> anyhow::Result<()> {
	let bytes = download(client, url).await?.bytes().await?;
	std::fs::write(path, bytes).context("Failed to write to file")?;
	Ok(())
}

/// Download and parse into JSON
pub async fn download_json<D: DeserializeOwned>(
	client: &Client,
	url: impl reqwest::IntoUrl,
) -> anyhow::Result<D> {
	let out = download(client, url)
		.await?
		.json()
		.await
		.context("Failed to parse response JSON")?;
	Ok(out)
}

/// Download a latest Github release
pub async fn download_github_release(
	client: &Client,
	user: &str,
	repo: &str,
) -> anyhow::Result<GithubRelease> {
	let url = format!("https://api.github.com/repos/{user}/{repo}/releases/latest");
	let out = download_json(client, url).await?;
	Ok(out)
}

/// Structure for a Github release
#[derive(Deserialize)]
pub struct GithubRelease {
	pub assets: Vec<GithubReleaseAsset>,
}

/// Asset in a Github release
#[derive(Deserialize)]
pub struct GithubReleaseAsset {
	pub name: String,
	pub url: String,
	/// The URL to the actual file
	pub browser_download_url: String,
}

impl GithubRelease {
	/// Get the first asset who's name matches a pattern
	pub fn get_asset_pattern(&self, pat: &str) -> Option<&GithubReleaseAsset> {
		self.assets.iter().find(|x| x.name.contains(pat))
	}
}

/// Print a progress message
pub fn print_progress(message: &str) {
	cprintln!("<s>{message}...");
}
