# Update-informer

[![CI](https://github.com/mgrachev/update-informer/workflows/CI/badge.svg)](https://github.com/mgrachev/update-informer/actions)

Update informer for CLI applications written in Rust 🦀

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
update-notifier = "0.1.0"
```

To check the version on crates.io:

```rust
use update_informer::registry::Crates;

match update_informer::check_version(Crates, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))? {
    Some(version) => {
        println!("New version is available: {}", version);
    }
    None => {
        println!("No new version");
    }
}
```

To check the version on GitHub:

```rust
use update_informer::registry::GitHub;

// Format: {owner}/{repo}
let pkg_name = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_NAME"));

match update_informer::check_version(GitHub, &pkg_name, env!("CARGO_PKG_VERSION"))? {
    Some(version) => {
        println!("New version is available: {}", version);
    }
    None => {
        println!("No new version");
    }
};
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
