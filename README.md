
<h1 align="center">🔔 Tiny Update Notifier 🔔</h1>

<p align="center">
  <a href="https://github.com/Araxeus/tiny-update-notifier" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white" />
  </a>
  <a href="https://crates.io/crates/tiny_update_notifier" target="_blank">
    <img alt="Version" src="https://img.shields.io/crates/v/tiny_update_notifier?logo=semanticweb&style=for-the-badge&labelColor=yellow&color=grey" onerror='this.onerror=undefined; this.src="https://img.shields.io/badge/version-1.0.0-blue.svg?cacheSeconds=2592000"'/>
  </a>
</p>
<p align="center">
  <a href="https://github.com/Araxeus/tiny-update-notifier/blob/main/LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/github/license/Araxeus/tiny-update-notifier?color=blue&style=plastic" />
  </a>
  <a href="https://github.com/Araxeus/tiny-update-notifier" target="_blank">
    <img alt="Maintenance" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg?style=plastic" />
  </a>
</p>

<p align="center">
Tiny update notifier utility for rust cli programs
</p>

---

Checks for update if **more than 24h have passed** since the last check (Customizable),

Then pops up a notification if a new version was found 📢
> supports crates.io and github releases

![App Screenshot](https://user-images.githubusercontent.com/78568641/210151741-701ca397-d9bb-4acc-8e62-292a1d7495d4.png)

## Installation

Install tiny_update_notifier using Cargo

```bash
  cargo add tiny_update_notifier
```
    
## Usage
##### Multi-threaded / Non-blocking :
```rust
// check on crates.io
tiny_update_notifier::check_cratesIO(pkg_version, pkg_name);

// check on github releases
tiny_update_notifier::check_github(pkg_version, pkg_name, pkg_repo_url);
```

##### Single-threaded / Blocking
```rust
tiny_update_notifier::Notifier::new(
    tiny_update_notifier::Source,
    pkg_version,
    pkg_name,
    pkg_repo_url
)
.interval(Duration) //Optional, default is 24h
.run();
```

## Examples

```rust
// Spawns a thread to check for updates on Crates.io and notify user if there is a new version available.
use tiny_update_notifier::check_cratesIO;

fn main() {
    check_cratesIO(
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
    );
}
```
```rust
// Spawns a thread to check for updates on GitHub Releases and notify user if there is a new version available.
use tiny_update_notifier::check_github;

fn main() {
    check_github(
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_REPOSITORY"),
    );
}
```

```rust
// Equivalent to check_github, except the interval is changed to 1 week
use std::{thread, time::Duration};
use tiny_update_notifier::{Notifier, Source};

fn main() {
    thread::spawn(|| {
        Notifier::new(
            Source::GitHub,
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_REPOSITORY"),
        )
        .interval(Duration::from_secs(60 * 60 * 24 * 7))
        .run();
    });
}
```
----
> Used by https://github.com/Araxeus/ls-interactive/
