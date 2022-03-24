//! # Overview
//! Update informer for CLI applications written in Rust. It checks for a new version on [`Crates.io`], [`GitHub`] and [`PyPI`].
//!
//! ## Benefits
//! * Support of [Crates.io](#cratesio), [GitHub](#github) and [PyPI](#pypi).
//! * [Ability to implement your own registry to check updates](#implementing-your-own-registry).
//! * Configurable [check frequency](#interval) and [request timeout](#request-timeout).
//! * Minimum dependencies - only [`directories`], [`ureq`], [`semver`] and [`serde`].
//!
//! ## Usage
//!
//! By default, `update-informer` can only check on [`Crates.io`].
//! To enable support for other registries, use `features`:
//!
//! ```toml
//! [dependencies]
//! update-informer = { version = "0.5.0", default_features = false, features = ["github"] }
//! ```
//!
//! Available features:
//!
//! | Name   | Default? |
//! |--------|----------|
//! | cargo  | Yes      |
//! | github | No       |
//! | pypi   | No       |
//!
//! ## Crates.io
//!
//! To check for a new version on Crates.io, use the [`UpdateInformer::check_version`] function. This function takes the project name and current version:
//!
//! ```rust
//! use update_informer::{registry, Check};
//!
//! let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0");
//! if let Ok(Some(version)) = informer.check_version() {
//!     println!("New version is available: {}", version);
//! }
//! ```
//!
//! Also, you can take the name and version of the project from `Cargo` using environment variables:
//!
//! ```rust
//! use update_informer::{registry, Check};
//!
//! let name = env!("CARGO_PKG_NAME");
//! let version = env!("CARGO_PKG_VERSION");
//! update_informer::new(registry::Crates, name, version).check_version();
//! ```
//!
//! ## Interval
//!
//! Note that the first check will start only after the interval has expired.
//! By default, the interval is 24 hours, but you can change it:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{registry, Check};
//!
//! const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);
//!
//! let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").interval(EVERY_HOUR);
//! informer.check_version(); // The check will start only after an hour
//! ```
//!
//! ## Cache file
//!
//! By default, `update-informer` creates a file in the cache directory to avoid spam requests to the registry API.
//!
//! In order not to cache requests, use a zero interval:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{registry, Check};
//!
//! let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").interval(Duration::ZERO);
//! informer.check_version();
//! ```
//!
//! ## Request timeout
//!
//! You can also change the request timeout. By default, it is 5 seconds:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{registry, Check};
//!
//! const THIRTY_SECONDS: Duration = Duration::from_secs(30);
//!
//! let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").timeout(THIRTY_SECONDS);
//! informer.check_version();
//! ```
//!
//! ## GitHub
//!
//! To check for a new version on GitHub (note that the project name must contain the owner):
//!
//! ```rust
//! use update_informer::{registry, Check};
//!
//! let informer = update_informer::new(registry::GitHub, "owner/repo", "0.1.0");
//! informer.check_version();
//! ```
//!
//! ## PyPi
//!
//! To check for a new version on PyPI:
//!
//! ```rust
//! use update_informer::{registry, Check};
//!
//! let informer = update_informer::new(registry::PyPI, "package_name", "0.1.0");
//! informer.check_version();
//! ```
//!
//! ## Implementing your own registry
//!
//! You can implement your own registry to check updates. For example:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{registry, Check, Package, Registry, Result};
//!
//! struct YourOwnRegistry;
//!
//! impl Registry for YourOwnRegistry {
//!     const NAME: &'static str = "your_own_registry";
//!
//!     fn get_latest_version(pkg: &Package, _timeout: Duration) -> Result<Option<String>> {
//!         let url = format!("https://your_own_registry.com/{}/latest-version", pkg);
//!         let result = reqwest::blocking::get(url)?.text()?;
//!         let version = result.trim().to_string();
//!
//!         Ok(Some(version))
//!     }
//! }
//!
//! let informer = update_informer::new(YourOwnRegistry, "package_name", "0.1.0");
//! informer.check_version();
//! ```
//!
//! ## Tests
//!
//! In order not to check for updates in tests, you can use the [`FakeUpdateInformer::check_version`] function, which returns the desired version.
//!
//! Example of usage in unit tests:
//!
//! ```rust
//! use update_informer::{registry, Check};
//!
//! let name = "crate_name";
//! let version = "0.1.0";
//!
//! #[cfg(not(test))]
//! let informer = update_informer::new(registry::Crates, name, version);
//!
//! #[cfg(test)]
//! let informer = update_informer::fake(registry::Crates, name, version, "1.0.0");
//!
//! if let Ok(Some(version)) = informer.check_version() {
//!     println!("New version is available: {}", version);
//! }
//! ```
//!
//! ## Integration tests
//!
//! To use the [`FakeUpdateInformer::check_version`] function in integration tests, you must first add the feature flag to `Cargo.toml`:
//!
//! ```toml
//! [features]
//! stub_check_version = []
//! ```
//!
//! Then use this feature flag in your code and integration tests:
//!
//! ```rust
//! use update_informer::{registry, Check};
//!
//! let name = "crate_name";
//! let version = "0.1.0";
//!
//! #[cfg(not(feature = "stub_check_version"))]
//! let informer = update_informer::new(registry::Crates, name, version);
//!
//! #[cfg(feature = "stub_check_version")]
//! let informer = update_informer::fake(Crates, name, version, "1.0.0");
//!
//! informer.check_version();
//! ```
//!
//! [`Crates.io`]: https://crates.io
//! [`GitHub`]: https://github.com
//! [`PyPI`]: https://pypi.org
//! [`directories`]: https://github.com/dirs-dev/directories-rs
//! [`ureq`]: https://github.com/algesten/ureq
//! [`semver`]: https://github.com/dtolnay/semver
//! [`serde`]: https://github.com/serde-rs/serde

#[doc = include_str!("../README.md")]
use crate::{version::Version, version_file::VersionFile};
use std::time::Duration;

pub use package::Package;
pub use registry::Registry;

mod http;
mod package;
mod version;
mod version_file;

#[cfg(test)]
mod test_helper;

/// A registry service that stores information about releases.
pub mod registry;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub trait Check {
    /// Checks for a new version in the registry.
    fn check_version(&self) -> Result<Option<Version>> {
        Ok(None)
    }
}

/// Checks for a new version on Crates.io, GitHub and PyPi.
pub struct UpdateInformer<R: Registry, N: AsRef<str>, V: AsRef<str>> {
    _registry: R,
    name: N,
    version: V,
    interval: Duration,
    timeout: Duration,
}

/// Constructs a new `UpdateInformer`.
///
/// # Arguments
///
/// * `registry` - A registry service such as Crates.io or GitHub.
/// * `name` - A project name.
/// * `version` - Current version of the project.
///
/// # Examples
///
/// ```rust
/// use update_informer::{registry, Check};
///
/// let name = env!("CARGO_PKG_NAME");
/// let version = env!("CARGO_PKG_VERSION");
/// let informer = update_informer::new(registry::Crates, name, version);
/// ```
pub fn new<R, N, V>(registry: R, name: N, version: V) -> UpdateInformer<R, N, V>
where
    R: Registry,
    N: AsRef<str>,
    V: AsRef<str>,
{
    UpdateInformer {
        _registry: registry,
        name,
        version,
        interval: Duration::from_secs(60 * 60 * 24), // Once a day
        timeout: Duration::from_secs(5),
    }
}

impl<R: Registry, N: AsRef<str>, V: AsRef<str>> UpdateInformer<R, N, V> {
    /// Constructs a new `UpdateInformer`.
    ///
    /// # Arguments
    ///
    /// * `registry` - A registry service such as Crates.io or GitHub.
    /// * `name` - A project name.
    /// * `version` - Current version of the project.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use update_informer::{registry, Check, UpdateInformer};
    ///
    /// let name = env!("CARGO_PKG_NAME");
    /// let version = env!("CARGO_PKG_VERSION");
    /// let informer = UpdateInformer::new(registry::Crates, name, version);
    /// ```
    #[deprecated(since = "0.5.0", note = "Use `update_informer::new` instead")]
    pub fn new(registry: R, name: N, version: V) -> Self {
        crate::new(registry, name, version)
    }

    /// Sets an interval how often to check for a new version.
    ///
    /// # Arguments
    ///
    /// * `interval` - An interval in seconds. By default, it is 24 hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use update_informer::{registry, Check};
    ///
    /// const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);
    ///
    /// let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").interval(EVERY_HOUR);
    /// informer.check_version(); // The check will start only after an hour
    /// ```
    pub fn interval(self, interval: Duration) -> Self {
        Self { interval, ..self }
    }

    /// Sets a request timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout` - A request timeout. By default, it is 5 seconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use update_informer::{registry, Check};
    ///
    /// const THIRTY_SECONDS: Duration = Duration::from_secs(30);
    ///
    /// let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0").timeout(THIRTY_SECONDS);
    /// informer.check_version();
    /// ```
    pub fn timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }
}

impl<R: Registry, N: AsRef<str>, V: AsRef<str>> Check for UpdateInformer<R, N, V> {
    /// Checks for a new version in the registry.
    ///
    /// # Examples
    ///
    /// To check for a new version on Crates.io:
    ///
    /// ```rust
    /// use update_informer::{registry, Check};
    ///
    /// let informer = update_informer::new(registry::Crates, "crate_name", "0.1.0");
    /// informer.check_version();
    /// ```
    fn check_version(&self) -> Result<Option<Version>> {
        let pkg = Package::new(self.name.as_ref());

        // If the interval is zero, don't use the cache file
        let latest_version = if self.interval.is_zero() {
            match R::get_latest_version(&pkg, self.timeout)? {
                Some(v) => v,
                None => return Ok(None),
            }
        } else {
            let latest_version_file = VersionFile::new(R::NAME, &pkg, self.version.as_ref())?;
            let last_modified = latest_version_file.last_modified()?;

            if last_modified >= self.interval {
                // This is needed to update mtime of the file
                latest_version_file.recreate_file()?;

                match R::get_latest_version(&pkg, self.timeout)? {
                    Some(v) => {
                        latest_version_file.write_version(&v)?;
                        v
                    }
                    None => return Ok(None),
                }
            } else {
                latest_version_file.get_version()?
            }
        };

        let latest_version = Version::parse(latest_version)?;
        let current_version = Version::parse(self.version.as_ref())?;

        if latest_version > current_version {
            return Ok(Some(latest_version));
        }

        Ok(None)
    }
}

/// Fake `UpdateInformer`. Used only for tests.
pub struct FakeUpdateInformer<V: AsRef<str>> {
    version: V,
}

/// Constructs a new `FakeUpdateInformer`.
///
/// # Arguments
///
/// * `registry` - A registry service such as Crates.io or GitHub (not used).
/// * `name` - A project name (not used).
/// * `version` - Current version of the project (not used).
/// * `interval` - An interval how often to check for a new version (not used).
/// * `new_version` - A desired version.
///
/// # Examples
///
/// ```rust
/// use update_informer::{registry, Check};
///
/// let informer = update_informer::fake(registry::Crates, "repo", "0.1.0", "1.0.0");
/// ```
pub fn fake<R, N, V>(_registry: R, _name: N, _version: V, new_version: V) -> FakeUpdateInformer<V>
where
    R: Registry,
    N: AsRef<str>,
    V: AsRef<str>,
{
    FakeUpdateInformer {
        version: new_version,
    }
}

impl<V: AsRef<str>> FakeUpdateInformer<V> {
    /// Constructs a new `FakeUpdateInformer`.
    ///
    /// # Arguments
    ///
    /// * `registry` - A registry service such as Crates.io or GitHub (not used).
    /// * `name` - A project name (not used).
    /// * `version` - Current version of the project (not used).
    /// * `interval` - An interval how often to check for a new version (not used).
    /// * `new_version` - A desired version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use update_informer::{registry, Check, FakeUpdateInformer};
    ///
    /// let informer = FakeUpdateInformer::new(registry::Crates, "repo", "0.1.0", "1.0.0");
    /// ```
    #[deprecated(since = "0.5.0", note = "Use `update_informer::fake` instead")]
    pub fn new<R, N>(registry: R, name: N, version: V, new_version: V) -> Self
    where
        R: Registry,
        N: AsRef<str>,
        V: AsRef<str>,
    {
        crate::fake(registry, name, version, new_version)
    }

    /// Returns `FakeUpdateInformer`.
    pub fn interval(self, _interval: Duration) -> Self {
        self
    }

    /// Returns `FakeUpdateInformer`.
    pub fn timeout(self, _timeout: Duration) -> Self {
        self
    }
}

impl<V: AsRef<str>> Check for FakeUpdateInformer<V> {
    /// Returns the desired version as a new version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use update_informer::{registry, Check};
    ///
    /// let informer = update_informer::fake(registry::Crates, "crate_name", "0.1.0", "1.0.0");
    /// let result = informer.check_version();
    /// assert!(result.is_ok());
    ///
    /// let version = result.unwrap();
    /// assert!(version.is_some());
    /// assert_eq!(version.unwrap().to_string(), "v1.0.0");
    /// ```
    fn check_version(&self) -> Result<Option<Version>> {
        let version = Version::parse(self.version.as_ref())?;

        Ok(Some(version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{registry::Crates, test_helper::within_test_dir};
    use mockito::Mock;
    use std::fs;

    const PKG_NAME: &str = "repo";
    const CURRENT_VERSION: &str = "3.1.0";
    const LATEST_VERSION: &str = "3.1.1";

    fn mock_crates(pkg: &str) -> Mock {
        let pkg = Package::new(pkg);
        let (mock, _) = crate::test_helper::mock_crates(
            &pkg,
            200,
            "tests/fixtures/registry/crates/versions.json",
        );

        mock
    }

    #[test]
    fn no_new_version_with_interval_test() {
        within_test_dir(|_| {
            let informer = crate::new(Crates, PKG_NAME, CURRENT_VERSION);
            let result = informer.check_version();

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        });
    }

    #[test]
    fn no_new_version_on_registry_test() {
        within_test_dir(|_| {
            let _mock = mock_crates(PKG_NAME);
            let informer = crate::new(Crates, PKG_NAME, LATEST_VERSION).interval(Duration::ZERO);
            let result = informer.check_version();

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        });
    }

    #[test]
    fn check_version_on_crates_test() {
        within_test_dir(|_| {
            let _mock = mock_crates(PKG_NAME);
            let informer = crate::new(Crates, PKG_NAME, CURRENT_VERSION).interval(Duration::ZERO);
            let result = informer.check_version();
            let version = Version::parse(LATEST_VERSION).expect("parse version");

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(version));
        });
    }

    #[test]
    fn return_version_from_file_test() {
        within_test_dir(|version_file| {
            fs::write(version_file, "4.0.0").expect("create file");

            let informer = crate::new(Crates, PKG_NAME, CURRENT_VERSION);
            let result = informer.check_version();
            let version = Version::parse("4.0.0").expect("parse version");

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(version));
        });
    }

    #[test]
    fn create_version_file_test() {
        within_test_dir(|version_file| {
            assert!(!version_file.exists());

            let informer = crate::new(Crates, PKG_NAME, CURRENT_VERSION);
            let result = informer.check_version();
            assert!(result.is_ok());
            assert!(version_file.exists());

            let version = fs::read_to_string(version_file).expect("read file");
            assert_eq!(version, CURRENT_VERSION);
        });
    }

    #[test]
    fn do_not_create_version_file_test() {
        within_test_dir(|version_file| {
            assert!(!version_file.exists());

            let _mock = mock_crates(PKG_NAME);
            let informer = crate::new(Crates, PKG_NAME, CURRENT_VERSION).interval(Duration::ZERO);
            let result = informer.check_version();

            assert!(result.is_ok());
            assert!(!version_file.exists());
        });
    }

    #[test]
    fn check_version_with_string_name_test() {
        within_test_dir(|_| {
            let pkg_name = format!("{}/{}", "owner", PKG_NAME);
            let informer = crate::new(Crates, pkg_name, CURRENT_VERSION);
            let result = informer.check_version();

            assert!(result.is_ok());
        });
    }

    #[test]
    fn check_version_with_string_version_test() {
        within_test_dir(|_| {
            let version = String::from(CURRENT_VERSION);
            let informer = crate::new(Crates, PKG_NAME, version);
            let result = informer.check_version();

            assert!(result.is_ok());
        });
    }

    #[test]
    fn check_version_with_amp_string_test() {
        within_test_dir(|_| {
            let pkg_name = format!("{}/{}", "owner", PKG_NAME);
            let version = String::from(CURRENT_VERSION);
            let informer = crate::new(Crates, &pkg_name, &version);
            let result = informer.check_version();

            assert!(result.is_ok());
        });
    }

    #[test]
    fn fake_check_version_test() {
        let version = "1.0.0";
        let informer = crate::fake(Crates, PKG_NAME, CURRENT_VERSION, version)
            .interval(Duration::ZERO)
            .timeout(Duration::ZERO);
        let result = informer.check_version();
        let version = Version::parse(version).expect("parse version");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(version));
    }
}
