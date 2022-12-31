
<h1 align="center">Tiny Update Notifier 🔔</h1>
<p>
  <a href="https://github.com/Araxeus/tiny-update-notifier/releases" target="_blank">
    <img alt="Version" src="https://img.shields.io/crates/v/tiny_update_notifier" onerror='this.onerror=undefined; this.src="https://img.shields.io/badge/version-1.0.0-blue.svg?cacheSeconds=2592000"'/>
  </a>
  <a href="https://github.com/Araxeus/tiny-update-notifier/blob/main/LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/github/license/Araxeus/tiny-update-notifier?color=yellow" />
  </a>
   <a href="https://github.com/Araxeus/tiny-update-notifier" target="_blank">
    <img alt="Maintenance" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg" />
  </a>
</p>

Tiny update notifier utility for rust cli programs

Checks for update on program launch if more than 24h have passed since the last check, then pops up a notification if a new version was found 📢

## Screenshots

![App Screenshot](https://user-images.githubusercontent.com/78568641/210151741-701ca397-d9bb-4acc-8e62-292a1d7495d4.png)

## Installation

Install tiny_update_notifier using Cargo

```bash
  cd my-project
  cargo add tiny_update_notifier
```
    
## Usage

```rust
tiny_updater_notifier::Notifier::new().run(pkg_version, pkg_name, pkg_repo_url)
```

## Example

```rust,no_run
use tiny_updater_notifier::Notifier;

fn main() -> std::io::Result<()> {
        Notifier::new(
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_REPOSITORY"),
    )
    .run();

    Ok(())
}
```
