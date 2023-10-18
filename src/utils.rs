use std::{
	ffi::OsStr,
	future::Future,
	io::{stdin, stdout, Read, Write},
	os::windows::process::CommandExt,
	path::{Path, PathBuf},
	process::Command,
	thread::JoinHandle,
};

use anyhow::{anyhow, Context};
use directories::ProjectDirs;
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

/// Download bytes
pub async fn download_bytes(
	client: &Client,
	url: impl reqwest::IntoUrl,
) -> anyhow::Result<bytes::Bytes> {
	let bytes = download(client, url).await?.bytes().await?;
	Ok(bytes)
}

/// Download a file to a path
pub async fn download_file(
	client: &Client,
	url: impl reqwest::IntoUrl,
	path: &Path,
) -> anyhow::Result<()> {
	let bytes = download_bytes(client, url)
		.await
		.context("Failed to download file bytes")?;
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
		self.get_asset_patterns(&[pat])
	}

	/// Get the first asset who's name matches multiple patterns
	pub fn get_asset_patterns(&self, pats: &[&str]) -> Option<&GithubReleaseAsset> {
		self.assets
			.iter()
			.find(|x| pats.iter().all(|p| x.name.contains(p)))
	}
}

/// Print a progress message
pub fn print_progress(message: impl AsRef<str>) {
	println!("{}...", message.as_ref());
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

/// Executes a function with a brand new tokio runtime
pub fn tokio_exec<F: Future>(f: F) -> anyhow::Result<F::Output> {
	let rt = tokio::runtime::Runtime::new().context("Failed to start runtime")?;
	Ok(rt.block_on(f))
}

/// Executes a function with a brand new tokio runtime on a new thread
pub fn tokio_exec_deferred<F>(f: F) -> anyhow::Result<JoinHandle<anyhow::Result<F::Output>>>
where
	F: Future + Send + 'static,
	F::Output: Send + 'static,
{
	let h = std::thread::spawn(|| tokio_exec(f));

	Ok(h)
}

/// Gets a data directory and ensures it exists using ProjectDirs
pub fn get_data_dir(project: &str) -> anyhow::Result<PathBuf> {
	let out = get_simple_project_dirs(project)?
		.data_dir()
		.parent()
		.ok_or(anyhow!("Failed to get parent data directory"))?
		.to_owned();

	std::fs::create_dir_all(&out).context("Failed to ensure data directory exists")?;

	Ok(out)
}

/// Gets a local data directory and ensures it exists using ProjectDirs
pub fn get_local_data_dir(project: &str) -> anyhow::Result<PathBuf> {
	let out = get_simple_project_dirs(project)?
		.data_local_dir()
		.parent()
		.ok_or(anyhow!("Failed to get parent local data directory"))?
		.to_owned();

	std::fs::create_dir_all(&out).context("Failed to ensure local data directory exists")?;

	Ok(out)
}

/// Gets a local program file and ensures the directories leading up to it exist
pub fn get_local_program(project: &str, exec_name: &str) -> anyhow::Result<PathBuf> {
	let programs_dir =
		get_local_data_dir("Programs").context("Failed to get programs directory")?;
	let dir = programs_dir.join(project);
	std::fs::create_dir_all(&dir)?;
	Ok(dir.join(exec_name))
}

fn get_simple_project_dirs(project: &str) -> anyhow::Result<ProjectDirs> {
	let out = ProjectDirs::from("", "", project).ok_or(anyhow!("Failed to get directories"))?;
	Ok(out)
}
