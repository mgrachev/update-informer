# Update-informer [![CI]](https://github.com/mgrachev/update-informer/actions) [![Crates.io]](https://crates.io/crates/update-informer) [![docs.rs]](https://docs.rs/update-informer) [![codecov]](https://codecov.io/gh/mgrachev/update-informer)

[CI]: https://github.com/mgrachev/update-informer/workflows/CI/badge.svg
[Crates.io]: https://img.shields.io/crates/v/update-informer
[docs.rs]: https://img.shields.io/docsrs/update-informer
[codecov]: https://codecov.io/gh/mgrachev/update-informer/branch/main/graph/badge.svg?token=A4XD1DGFGJ

[directories]: https://github.com/dirs-dev/directories-rs
[ureq]: https://github.com/algesten/ureq
[semver]: https://github.com/dtolnay/semver
[serde]: https://github.com/serde-rs/serde
[CLI application]: https://cli.github.com
[JavaScript library]: https://github.com/yeoman/update-notifier
[Update-informer]: https://evrone.com/update-informer?utm_source=github&utm_campaign=update-informer
[Evrone]: https://evrone.com/?utm_source=github&utm_campaign=update-informer
[MIT]: https://choosealicense.com/licenses/mit

<img align="right"
     alt="update-informer"
     src="https://raw.githubusercontent.com/mgrachev/update-informer/main/logo.svg?sanitize=true">

Update informer for CLI applications written in Rust 🦀

It checks for a new version on Crates.io, GitHub and PyPI 🚀

## Benefits
* Support of **Crates.io**, **GitHub** and **PyPI**.
* Ability to **implement** your **own registry** to check updates.
* Configurable **check frequency** and **request timeout**.
* **Minimum dependencies** - only [directories], [ureq], [semver] and [serde].

The idea is actually not new, as GitHub has been doing for a long time in its [CLI application].<br>
There is also a popular [JavaScript library].

[Update-informer] is created & supported by [Evrone].

## Usage

Add `update-informer` to `Cargo.toml`:

```toml
[dependencies]
update-informer = "0.5.0"
```

By default, `update-informer` can only check on Crates.io.
To enable support for other registries, use `features`:

```toml
[dependencies]
update-informer = { version = "0.5.0", default_features = false, features = ["github"] }
```

Available features:

| Name   | Default? |
|--------|----------|
| cargo  | Yes      |
| github | No       |
| pypi   | No       |

## Crates.io

To check for a new version on Crates.io, use the `UpdateInformer::check_version` function.<br>
This function takes the project name and current version as well as check interval:

```rust
use update_informer::{registry, Check};

let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0");
if let Ok(Some(version)) = informer.check_version() {
    println!("New version is available: {}", version);
}
```

Also, you can take the name and version of the project from `Cargo` using environment variables:

```rust
use update_informer::{registry, Check};

let name = env!("CARGO_PKG_NAME");
let version = env!("CARGO_PKG_VERSION");
update_informer::new(registry::Crates, name, version).check_version();
```

## Interval

Note that the first check will start only after the interval has expired.
By default, the interval is 24 hours, but you can change it:

```rust
use std::time::Duration;
use update_informer::{registry, Check};

const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);

let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").interval(EVERY_HOUR);
informer.check_version(); // The check will start only after an hour
```

## Cache file

By default, `update-informer` creates a file in the cache directory to avoid spam requests to the registry API.

In order not to cache requests, use a zero interval:

```rust
use std::time::Duration;
use update_informer::{registry, Check};

let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").interval(Duration::ZERO);
informer.check_version();
```

## Request timeout

You can also change the request timeout. By default, it is 5 seconds:

 ```rust
 use std::time::Duration;
 use update_informer::{registry, Check};

 const THIRTY_SECONDS: Duration = Duration::from_secs(30);

 let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").timeout(THIRTY_SECONDS);
 informer.check_version();
 ```

## GitHub

To check for a new version on GitHub (note that the project name must contain the owner):

```rust
use update_informer::{registry, Check};

let informer = update_informer::new(registry::GitHub, "owner/repo", "0.1.0");
informer.check_version();
```

## PyPi

To check for a new version on PyPI:

```rust
use update_informer::{registry, Check};

let informer = update_informer::new(registry::PyPI, "package_name", "0.1.0");
informer.check_version();
```

## Implementing your own registry

You can implement your own registry to check updates. For example: 

```rust
use std::time::Duration;
use update_informer::{registry, Check, Package, Registry, Result};

struct YourOwnRegistry;

impl Registry for YourOwnRegistry {
    const NAME: &'static str = "your_own_registry";

    fn get_latest_version(pkg: &Package, _timeout: Duration) -> Result<Option<String>> {
        let url = format!("https://your_own_registry.com/{}/latest-version", pkg);
        let result = reqwest::blocking::get(url)?.text()?;
        let version = result.trim().to_string();

        Ok(Some(version))
    }
}

let informer = update_informer::new(YourOwnRegistry, "package_name", "0.1.0");
informer.check_version();
```

## Example

<details>
<summary>
A real example of using <code>update_informer</code> with <a href="https://github.com/mackwic/colored">colored</a> crate.
</summary>

```rust
use colored::*;
use update_informer::{registry, Check};

fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");

    let informer = update_informer::new(registry::Crates, pkg_name, current_version);
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
<img src="https://raw.githubusercontent.com/mgrachev/update-informer/main/images/example.png" alt="example" style="max-width: 100%;">

</details>


## Tests

In order not to check for updates in tests, you can use the `FakeUpdateInformer::check_version` function, which returns the desired version.

Example of usage in unit tests:

```rust
use update_informer::{registry, Check};

let name = "crate_name";
let version = "0.1.0";

#[cfg(not(test))]
let informer = update_informer::new(registry::Crates, name, version);

#[cfg(test)]
let informer = update_informer::fake(registry::Crates, name, version, "1.0.0");

if let Ok(Some(version)) = informer.check_version() {
    println!("New version is available: {}", version);
}
```

## Integration tests

To use the `FakeUpdateInformer::check_version` function in integration tests, you must first add the feature flag to `Cargo.toml`:

```toml
[features]
stub_check_version = []
```

Then use this feature flag in your code and integration tests:

```rust
use update_informer::{registry, Check};

let name = "crate_name";
let version = "0.1.0";

#[cfg(not(feature = "stub_check_version"))]
let informer = update_informer::new(registry::Crates, name, version);

#[cfg(feature = "stub_check_version")]
let informer = update_informer::fake(registry::Crates, name, version, "1.0.0");

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

[MIT]
