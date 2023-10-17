use std::{
	ffi::OsStr,
	io::{stdin, stdout, Read, Write},
	os::windows::process::CommandExt,
	path::Path,
	process::Command,
};

use anyhow::Context;
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

/// Get the list of releases for a Github project
pub async fn get_github_releases(
	client: &Client,
	user: &str,
	repo: &str,
) -> anyhow::Result<Vec<GithubRelease>> {
	let url = format!("https://api.github.com/repos/{user}/{repo}/releases");
	let out = download_json(client, url).await?;
	Ok(out)
}

/// A single Github release
#[derive(Deserialize)]
pub struct GithubRelease {
	pub tag_name: String,
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
	println!("{message}...");
}

/// Creates a command with elevated permissions (which some installers require).
/// See https://stackoverflow.com/a/60958546
pub fn run_elevated(cmd: impl AsRef<OsStr>) -> Command {
	let mut out = Command::new("cmd");
	out.args(&["/C", "start"]);
	out.arg(cmd);
	out.creation_flags(0x00000008);

	out
}

/// Prompt the user to press any key to continue
pub fn continue_prompt() {
	let mut stdout = stdout();
	let _ = stdout.write(b"Press Enter to continue...");
	let _ = stdout.flush();
	let _ = stdin().read(&mut [0]);
}
