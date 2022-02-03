# Update-informer [![CI](https://github.com/mgrachev/update-informer/workflows/CI/badge.svg)](https://github.com/mgrachev/update-informer/actions) [![Crates.io](https://img.shields.io/crates/v/update-informer)](https://crates.io/crates/update-informer) [![docs.rs](https://img.shields.io/docsrs/update-informer)](https://docs.rs/update-informer) [![codecov](https://codecov.io/gh/mgrachev/update-informer/branch/main/graph/badge.svg?token=A4XD1DGFGJ)](https://codecov.io/gh/mgrachev/update-informer)

<img align="right"
     alt="update-informer"
     src="https://raw.githubusercontent.com/mgrachev/update-informer/main/logo.svg?sanitize=true">

Update informer for CLI applications written in Rust 🦀

It checks for a new version on **Crates.io** and **GitHub** 🚀

### Benefits
* Support of **Crates.io** and **GitHub**.
* **Configurable frequency** of checks.
* **Minimum dependencies** - only [ureq](https://github.com/algesten/ureq), [semver](https://github.com/dtolnay/semver) and [serde](https://github.com/serde-rs/serde).

The idea is actually not new, as GitHub has been doing for a long time in its [CLI application](https://cli.github.com).<br>
There is also a popular [JavaScript library](https://github.com/yeoman/update-notifier).

## Usage

Add `update-informer` to `Cargo.toml`:

```toml
[dependencies]
update-informer = "0.2.0"
```

To check for a new version on **Crates.io**, use the `UpdateInformer::check_version` function.<br>
This function takes the project name and current version as well as check interval:

```rust
use update_informer::{registry::Crates, Check, UpdateInformer};

let informer = UpdateInformer::new(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
if let Ok(Some(version)) = informer.check_version() {
    println!("New version is available: {}", version);
}
```

Also, you can take the name and version of the project from **Cargo** using environment variables:

```rust
use update_informer::{registry::Crates, Check, UpdateInformer};

let name = env!("CARGO_PKG_NAME");
let version = env!("CARGO_PKG_VERSION");
UpdateInformer::new(Crates, name, version, Duration::from_secs(60 * 60 * 24)).check_version();
```

Note that the first check will start only after the interval has expired:

```rust
use update_informer::{registry::Crates, Check, UpdateInformer};

const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);

let informer = UpdateInformer::new(Crates, "repo", "0.1.0", EVERY_HOUR);
informer.check_version(); // The check will start only after an hour
```

To check for a new version on **GitHub** (note that the project name must contain the owner):

```rust
use update_informer::{registry::GitHub, Check, UpdateInformer};

let informer = UpdateInformer::new(GitHub, "owner/repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
informer.check_version();
```

## Example

A real example of using `update_informer` with [colored](https://github.com/mackwic/colored) crate:

```rust
use colored::*;
use std::time::Duration;
use update_informer::{registry::Crates, Check, UpdateInformer};

fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let interval = Duration::from_secs(60 * 60 * 24);

    let informer = UpdateInformer::new(Crates, pkg_name, current_version, interval);
    if let Ok(Some(version)) = informer.check_version() {
        let msg = format!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name.italic().cyan(),
            current_version = current_version,
            new_version = version.to_string().green()
        );

        let release_url = format!(
            "https://github.com/{pkg_name}/{pkg_name}/releases/tag/{version}",
            pkg_name = pkg_name,
            version = version
        )
            .yellow();

        println!("\n{msg}\n{url}", msg = msg, url = release_url);
    }
}
```

The result will look like:
![example](https://raw.githubusercontent.com/mgrachev/update-informer/main/images/example.png)

## Tests

In order not to check for updates in tests, you can use the `FakeUpdateInformer::check_version` function, which returns the desired version.

Example of usage in unit tests:

```rust
use update_informer::{registry::Crates, Check, FakeUpdateInformer, UpdateInformer};

let name = "repo";
let version = "0.1.0";
let interval = Duration::from_secs(60 * 60 * 24);

#[cfg(not(test))]
let informer = UpdateInformer::new(Crates, name, version, interval);

#[cfg(test)]
let informer = FakeUpdateInformer::new(Crates, name, version, interval, "1.0.0");

if let Ok(Some(version)) = informer.check_version() {
    println!("New version is available: {}", version);
}
```

To use the `FakeUpdateInformer::check_version` function in integration tests, you must first add the feature flag to `Cargo.toml`:

```toml
[features]
stub_check_version = []
```

Then use this feature flag in your code and integration tests:

```rust
#[cfg(not(feature = "stub_check_version"))]
let informer = UpdateInformer::new(Crates, name, version, interval);

#[cfg(feature = "stub_check_version")]
let informer = FakeUpdateInformer::new(Crates, name, version, interval, "1.0.0");

informer.check_version();
```

## Sponsors

<p>
  <a href="https://evrone.com/?utm_source=github&utm_campaign=update-informer">
    <img src="https://www.mgrachev.com/assets/static/sponsored_by_evrone.svg?sanitize=true"
      alt="Sponsored by Evrone">
  </a>
</p>

## License

[MIT](https://choosealicense.com/licenses/mit)
