# Update-informer [![CI](https://github.com/mgrachev/update-informer/workflows/CI/badge.svg)](https://github.com/mgrachev/update-informer/actions)

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
update-informer = "0.1.0"
```

To check for a new version on **Crates.io**, use the `check_version` function.<br>
This function takes the project name and current version as well as check interval:

```rust
use update_informer::{check_version, registry::Crates};

if let Ok(Some(version)) = check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24)) {
    println!("New version is available: {}", version);
}
```

Also, you can take the name and version of the project from **Cargo** using environment variables:

```rust
use update_informer::{check_version, registry::Crates};

check_version(Crates, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), Duration::from_secs(60 * 60 * 24));
```

Note that the first check will start only after the interval has expired:

```rust
use update_informer::{check_version, registry::Crates};

const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);

check_version(Crates, "repo", "0.1.0", EVERY_HOUR); // The check will start only after an hour
```

To check for a new version on **GitHub** (note that the project name must contain the owner):

```rust
use update_informer::{check_version, registry::GitHub};

check_version(GitHub, "owner/repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
```

## Example

A real example of using `update_informer` with [colored](https://github.com/mackwic/colored) crate:

```rust
use colored::*;
use std::time::Duration;
use update_informer::{check_version, registry::Crates};

fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let interval = Duration::from_secs(60 * 60 * 24);

    if let Ok(Some(version)) = check_version(Crates, pkg_name, current_version, interval) {
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

In order not to check for updates in tests, you can use the `stub_check_version` function, which returns the desired version.

Example of usage in unit tests:

```rust
use std::time::Duration;
use update_informer::registry::Crates;

#[cfg(not(test))]
let result = update_informer::check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24));

#[cfg(test)]
let result = update_informer::stub_check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24), "1.0.0");

if let Ok(Some(version)) = result {
    println!("New version is available: {}", version);
}
```

To use the `stub_check_version` function in integration tests, you must first add the feature flag to `Cargo.toml`:

```toml
[features]
stub_check_version = []
```

Then use this feature flag in your code and integration tests:

```rust
#[cfg(not(feature = "stub_check_version"))]
let result = update_informer::check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24));

#[cfg(feature = "stub_check_version")]
let result = update_informer::stub_check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24), "1.0.0");
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
