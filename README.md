
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

Checks for update on program launch if more than 24h have passed since the last check, then pops up a notification if a new version was found 📢

![App Screenshot](https://user-images.githubusercontent.com/78568641/210151741-701ca397-d9bb-4acc-8e62-292a1d7495d4.png)

## Installation

Install tiny_update_notifier using Cargo

```bash
  cd my-project
  cargo add tiny_update_notifier
```
    
## Usage

```rust
tiny_update_notifier::Notifier::new().run(pkg_version, pkg_name, pkg_repo_url)
```

## Examples

```rust
// Spawns a thread to check for updates and notify user if there is a new version available.
use tiny_update_notifier::run_notifier;

fn main() {
    run_notifier(
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_REPOSITORY"),
    );
}
```

```rust
// equivalent to the code above
use std::thread;
use tiny_update_notifier::Notifier;

fn main() {
    thread::spawn(|| {
        Notifier::new(
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_REPOSITORY"),
        )
        .run();
    });
}
```
----
> Used by https://github.com/Araxeus/ls-interactive/
