# Update-informer

Update informer for CLI applications written in Rust 🦀

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
update-notifier = "0.1.0"
```

And then:

```rust
match update_informer::check_version(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) {
    Ok(result) => match result {
        Some(version) => {
            println!("New version is available: {}", version);
        }
        None => {
            println!("No new version");
        }
    },
    Err(e) => {
        eprint!("Error: {}", e);
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
