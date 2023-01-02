#![allow(clippy::multiple_crate_versions)] // TODO: Remove this when possible
use directories::ProjectDirs;
/// Use either `tiny_update_notifier::{check_cratesIO, check_github}`
/// spawns a new thread to check for updates and notify user if there is a new version available.
///
/// ## Examples
///
/// ```rust,no_run
/// tiny_update_notifier::check_cratesIO(
///     env!("CARGO_PKG_VERSION"),
///     env!("CARGO_PKG_NAME"),
/// );
///
/// tiny_update_notifier::check_github(
///     env!("CARGO_PKG_VERSION"),
///     env!("CARGO_PKG_NAME"),
///     env!("CARGO_PKG_REPOSITORY"),
/// );
/// ```
use notify_rust::Notification;

use std::{
    fs,
    io::{self, Error, ErrorKind},
    time::Duration,
};

/// Source to check for updates
pub enum Source {
    /// Check for updates on Crates.io
    ///
    /// (Only works if the package is published on Crates.io)
    CratesIO,
    /// Check for updates on GitHub
    ///
    /// (Only works if the package is published on GitHub)
    GitHub,
}

/// Spawns a thread to check for updates on Crates.io and notify user if there is a new version available.
///
/// This function returns immediately and does not block the current thread.
///
/// ## Examples
///
/// ```rust,no_run
/// tiny_update_notifier::check_cratesIO(
///     env!("CARGO_PKG_VERSION"),
///     env!("CARGO_PKG_NAME"),
/// );
/// ```
#[allow(non_snake_case)]
pub fn check_cratesIO(version: &'static str, name: &'static str) {
    spawn(Source::CratesIO, version, name, "");
}

/// Spawns a thread to check for updates on GitHub Releases and notify user if there is a new version available.
///
/// This function returns immediately and does not block the current thread.
///
/// ## Examples
///
/// ```rust,no_run
/// tiny_update_notifier::check_github(
///     env!("CARGO_PKG_VERSION"),
///     env!("CARGO_PKG_NAME"),
///     env!("CARGO_PKG_REPOSITORY"),
/// );  
/// ```
pub fn check_github(version: &'static str, name: &'static str, repo_url: &'static str) {
    spawn(Source::GitHub, version, name, repo_url);
}

fn spawn(source: Source, version: &'static str, name: &'static str, repo_url: &'static str) {
    std::thread::spawn(move || {
        Notifier::new(source, version, name, repo_url).run();
    });
}

/// Use `Notifier::new(source, pkg_version, pkg_name, pkg_repo_url).run()`
/// to check for updates and notify user if there is a new version available.
///
/// ## Examples
///
/// ```rust,no_run
/// use tiny_update_notifier::{Notifier, Source};
/// std::thread::spawn(|| {
///     Notifier::new(
///         Source::GitHub,
///         env!("CARGO_PKG_VERSION"),
///         env!("CARGO_PKG_NAME"),
///         env!("CARGO_PKG_REPOSITORY"),
///     )
///     .interval(Duration::from_secs(60 * 60 * 24 * 7)) // Change interval to 7 days (Default is 24H)
///     .run();
/// });
/// ```
pub struct Notifier {
    version: &'static str,
    name: &'static str,
    repo_url: &'static str,
    source: Source,
    interval: Duration,
}

impl Notifier {
    /// Use `Notifier::new(source, pkg_version, pkg_name, pkg_repo_url).run()`
    /// to check for updates and notify user if there is a new version available.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use tiny_update_notifier::{Notifier, Source};
    /// std::thread::spawn(|| {
    ///     Notifier::new(
    ///         Source::GitHub,
    ///         env!("CARGO_PKG_VERSION"),
    ///         env!("CARGO_PKG_NAME"),
    ///         env!("CARGO_PKG_REPOSITORY"),
    ///     )
    ///    .interval(Duration::from_secs(60 * 60 * 24 * 7)) // Change interval to 7 days (Default is 24H)
    ///    .run();
    /// });
    /// ```
    #[must_use]
    pub const fn new(
        source: Source,
        version: &'static str,
        name: &'static str,
        repo_url: &'static str,
    ) -> Self {
        Self {
            version,
            name,
            repo_url,
            source,
            interval: Duration::from_secs(60 * 60 * 24), // Default 24H
        }
    }

    /// Change the interval between checks for updates
    /// (Default is 24H)
    #[must_use]
    pub const fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Run the notifier
    pub fn run(&mut self) {
        match Self::should_check_update(self) {
            Err(e) => {
                Self::notification(self, &format!("Error: should_check_update() Failed: \n{e}"));
            }
            Ok(true) => Self::check_version(self),
            Ok(false) => (),
        };
    }

    fn check_version(&mut self) {
        if let Ok(new_version) = Self::get_latest_version(self) {
            if new_version != self.version {
                let link = if self.repo_url.is_empty() {
                    String::new()
                } else {
                    format!(
                        "\n{repo_url}/releases/tag/{new_version}",
                        repo_url = self.repo_url,
                    )
                };

                Self::notification(
                    self,
                    &format!(
                        "A new release of {pkg_name} is available: \n\
        v{current_version} -> v{new_version}{link}",
                        pkg_name = self.name,
                        current_version = self.version
                    ),
                );
            }

            Self::write_last_checked(self).unwrap_or_else(|e| {
                Self::notification(self, &format!("Error: write_last_checked() failed: \n{e}"));
            });
        }
    }

    fn notification(&mut self, body: &str) {
        Notification::new()
            .summary(self.name)
            .body(body)
            .icon("/usr/share/icons/hicolor/256x256/apps/gnome-software.png")
            .timeout(5000)
            .show()
            .ok();
    }

    fn get_latest_version(&mut self) -> io::Result<String> {
        let output = std::process::Command::new("curl")
            .arg("--silent")
            .arg(self.get_api_link()?)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let data: serde_json::Value = serde_json::from_str(&stdout)?;
                let version = self.extract_version_from_json(&data);
                Ok(version)
            }
            Err(e) => {
                Self::notification(self, &format!("Error: get_latest_version() failed: \n{e}"));
                Err(e)
            }
        }
    }

    fn get_api_link(&self) -> io::Result<String> {
        match self.source {
            Source::CratesIO => Ok(format!("https://crates.io/api/v1/crates/{}", self.name)),
            Source::GitHub => {
                let repo_url = self.repo_url;
                let data = repo_url.split('/').collect::<Vec<&str>>();
                if data.len() < 5 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid GitHub repo url",
                    ));
                };

                Ok(format!(
                    "https://api.github.com/repos/{owner}/{repo}/releases/latest",
                    owner = data[3],
                    repo = data[4]
                ))
            }
        }
    }

    fn extract_version_from_json(&self, data: &serde_json::Value) -> String {
        match self.source {
            Source::CratesIO => data["crate"]["max_stable_version"]
                .to_string()
                .trim_matches('"')
                .to_string(),
            Source::GitHub => data["tag_name"]
                .to_string()
                .trim_matches('"')
                .trim_start_matches('v')
                .to_string(),
        }
    }

    fn should_check_update(&mut self) -> io::Result<bool> {
        let binding = Self::get_cache_dir(self)?;
        let cache_dir = binding.cache_dir();
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir)?;
        }
        let path = cache_dir.join(format!("{}-last-update-check", self.name));
        if path.exists() {
            let metadata = fs::metadata(path)?;
            let last_modified_diff = metadata.modified()?.elapsed().unwrap_or_default();
            Ok(last_modified_diff > self.interval)
        } else {
            Ok(true)
        }
    }

    fn write_last_checked(&mut self) -> io::Result<()> {
        let path = Self::get_cache_dir(self)?
            .cache_dir()
            .join(format!("{}-last-update-check", self.name));
        fs::write(path, "")
    }

    fn get_cache_dir(&mut self) -> io::Result<ProjectDirs> {
        let project_dir = ProjectDirs::from("", "", self.name);
        project_dir
            .ok_or_else(|| io::Error::new(ErrorKind::Other, "Could not get project directory"))
    }
}
