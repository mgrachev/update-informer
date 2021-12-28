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

match check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24))? {
    Some(version) => {
        println!("New version is available: {}", version);
    }
    None => {
        println!("No new version");
    }
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

To check for a new version on **GitHub**:

```rust
use update_informer::{check_version, registry::GitHub};

check_version(GitHub, "owner/repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
```

Note that the project name must contain the owner.

## Sponsors

<p>
  <a href="https://evrone.com/?utm_source=github&utm_campaign=update-informer">
    <img src="https://www.mgrachev.com/assets/static/sponsored_by_evrone.svg?sanitize=true"
      alt="Sponsored by Evrone">
  </a>
</p>

## License

[MIT](https://choosealicense.com/licenses/mit)
