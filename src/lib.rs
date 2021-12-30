//! # Overview
//! Update informer for CLI applications written in Rust. It checks for a new version on **Crates.io** and **GitHub**.
//!
//! ### Benefits
//! * Support of **Crates.io** and **GitHub**.
//! * **Configurable frequency** of checks.
//! * **Minimum dependencies** - only [ureq](https://github.com/algesten/ureq), [semver](https://github.com/dtolnay/semver) and [serde](https://github.com/serde-rs/serde).
//!
//! ## Usage
//!
//! To check for a new version on **Crates.io**, use the [check_version] function. This function takes the project name and current version as well as check interval:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{check_version, registry::Crates};
//!
//! if let Ok(Some(version)) = check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24)) {
//!     println!("New version is available: {}", version);
//! }
//! ```
//!
//! Also, you can take the name and version of the project from **Cargo** using environment variables:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{check_version, registry::Crates};
//!
//! check_version(Crates, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), Duration::from_secs(60 * 60 * 24));
//! ```
//!
//! Note that the first check will start only after the interval has expired:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{check_version, registry::Crates};
//!
//! const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);
//!
//! check_version(Crates, "repo", "0.1.0", EVERY_HOUR); // The check will start only after an hour
//! ```
//!
//! To check for a new version on **GitHub** (note that the project name must contain the owner):
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::{check_version, registry::GitHub};
//!
//! check_version(GitHub, "owner/repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
//! ```
//!
//! ## Tests
//!
//! In order not to check for updates in tests, you can use the [stub_check_version] function, which returns the desired version.
//!
//! Example of usage in unit tests:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::registry::Crates;
//!
//! #[cfg(not(test))]
//! let result = update_informer::check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
//!
//! #[cfg(test)]
//! let result = update_informer::stub_check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24), "1.0.0");
//!
//! if let Ok(Some(version)) = result {
//!     println!("New version is available: {}", version);
//! }
//! ```
//!
//! To use the [stub_check_version] function in integration tests, you must first add the feature flag to `Cargo.toml`:
//!
//! ```toml
//! [features]
//! stub_check_version = []
//! ```
//!
//! Then use this feature flag in your code and integration tests:
//!
//! ```rust
//! use std::time::Duration;
//! use update_informer::registry::Crates;
//!
//! #[cfg(not(feature = "stub_check_version"))]
//! let result = update_informer::check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
//!
//! #[cfg(feature = "stub_check_version")]
//! let result = update_informer::stub_check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24), "1.0.0");
//! ```

use crate::package::Package;
use crate::registry::Registry;
use crate::version::Version;
use crate::version_file::VersionFile;
use std::time::Duration;

mod package;
mod version;
mod version_file;

#[cfg(test)]
mod test_helper;

/// A registry service that stores information about releases.
pub mod registry;

type Error = Box<dyn std::error::Error>;

/// Checks for a new version on Crates.io and GitHub.
///
/// # Arguments
/// * `registry` - A registry service such as Crates.io or GitHub.
/// * `name` - A project name.
/// * `version` - Current version of the project.
/// * `interval` - An interval how often to check for a new version.
///
/// # Examples
/// To check for a new version on **Crates.io**:
/// ```rust
/// use std::time::Duration;
/// use update_informer::{check_version, registry::Crates};
///
/// check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
/// ```
///
/// To check for a new version on **GitHub**:
/// ```rust
/// use std::time::Duration;
/// use update_informer::{check_version, registry::GitHub};
///
/// check_version(GitHub, "owner/repo", "0.1.0", Duration::from_secs(60 * 60 * 24));
/// ```
/// Note that the first check will start only after the interval has expired:
///
/// ```rust
/// use std::time::Duration;
/// use update_informer::{check_version, registry::Crates};
///
/// const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);
///
/// check_version(Crates, "repo", "0.1.0", EVERY_HOUR); // The check will start only after an hour
/// ```
pub fn check_version<R, N, V>(
    _registry: R,
    name: N,
    version: V,
    interval: Duration,
) -> Result<Option<Version>, Error>
where
    R: Registry,
    N: AsRef<str>,
    V: AsRef<str>,
{
    let pkg = Package::new(name.as_ref());
    let latest_version_file = VersionFile::new(&pkg, version.as_ref());
    let last_modified = latest_version_file.last_modified()?;

    let latest_version = if last_modified >= interval {
        // This is needed to update mtime of the file
        latest_version_file.recreate_file()?;

        match R::get_latest_version(&pkg)? {
            Some(v) => {
                latest_version_file.write_version(&v)?;
                v
            }
            None => return Ok(None),
        }
    } else {
        latest_version_file.get_version()?
    };

    let latest_version = Version::parse(latest_version)?;
    let current_version = Version::parse(version)?;
    if latest_version > current_version {
        return Ok(Some(latest_version));
    }

    Ok(None)
}

/// Returns the desired version as a new version.
/// Used only for tests.
///
/// # Arguments
/// * `registry` - A registry service such as Crates.io or GitHub (not used).
/// * `name` - A project name (not used).
/// * `version` - Current version of the project (not used).
/// * `interval` - An interval how often to check for a new version (not used).
/// * `new_version` - A desired version.
///
/// # Examples
/// ```rust
/// use std::time::Duration;
/// use update_informer::{registry::Crates, stub_check_version};
///
/// let result = stub_check_version(Crates, "repo", "0.1.0", Duration::from_secs(60 * 60 * 24), "1.0.0");
/// assert!(result.is_ok());
///
/// let version = result.unwrap();
/// assert!(version.is_some());
/// assert_eq!(version.unwrap().to_string(), "v1.0.0");
/// ```
pub fn stub_check_version<R, N, V>(
    _registry: R,
    _name: N,
    _version: V,
    _interval: Duration,
    new_version: V,
) -> Result<Option<Version>, Error>
where
    R: Registry,
    N: AsRef<str>,
    V: AsRef<str>,
{
    let version = Version::parse(new_version)?;

    Ok(Some(version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{Crates, GitHub};
    use crate::test_helper::within_test_dir;
    use mockito::Mock;
    use std::fs;

    const PKG_NAME: &str = "repo";
    const CURRENT_VERSION: &str = "3.1.0";
    const LATEST_VERSION: &str = "3.1.1";
    const ONE_DAY: Duration = Duration::from_secs(60 * 60 * 24);

    fn mock_github(pkg: &str) -> Mock {
        let pkg = Package::new(pkg);
        let (mock, _) = crate::test_helper::mock_github(
            &pkg,
            200,
            "tests/fixtures/registry/github/release.json",
        );

        mock
    }

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
            let result = check_version(Crates, PKG_NAME, CURRENT_VERSION, ONE_DAY);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        });
    }

    #[test]
    fn no_new_version_on_registry_test() {
        within_test_dir(|_| {
            let _mock = mock_crates(PKG_NAME);
            let result = check_version(Crates, PKG_NAME, LATEST_VERSION, Duration::default());

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        });
    }

    #[test]
    fn check_version_on_github_test() {
        within_test_dir(|_| {
            let _mock = mock_github("owner/repo");
            let result = check_version(GitHub, "owner/repo", CURRENT_VERSION, Duration::default());
            let version = Version::parse(LATEST_VERSION).expect("parse version");

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(version));
        });
    }

    #[test]
    fn check_version_on_crates_test() {
        within_test_dir(|_| {
            let _mock = mock_crates(PKG_NAME);
            let result = check_version(Crates, PKG_NAME, CURRENT_VERSION, Duration::default());
            let version = Version::parse(LATEST_VERSION).expect("parse version");

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(version));
        });
    }

    #[test]
    fn return_version_from_file_test() {
        within_test_dir(|version_file| {
            fs::write(version_file, "4.0.0").expect("create file");

            let result = check_version(Crates, PKG_NAME, CURRENT_VERSION, ONE_DAY);
            let version = Version::parse("4.0.0").expect("parse version");

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(version));
        });
    }

    #[test]
    fn create_version_file_test() {
        within_test_dir(|version_file| {
            assert!(!version_file.exists());

            let result = check_version(Crates, PKG_NAME, CURRENT_VERSION, ONE_DAY);
            assert!(result.is_ok());
            assert!(version_file.exists());

            let version = fs::read_to_string(version_file).expect("read file");
            assert_eq!(version, CURRENT_VERSION);
        });
    }

    #[test]
    fn check_version_with_string_name_test() {
        within_test_dir(|_| {
            let pkg_name = format!("{}/{}", "owner", PKG_NAME);
            let result = check_version(Crates, pkg_name, CURRENT_VERSION, ONE_DAY);

            assert!(result.is_ok());
        });
    }

    #[test]
    fn check_version_with_string_version_test() {
        within_test_dir(|_| {
            let version = String::from(CURRENT_VERSION);
            let result = check_version(Crates, PKG_NAME, version, ONE_DAY);

            assert!(result.is_ok());
        });
    }

    #[test]
    fn check_version_with_amp_string_test() {
        within_test_dir(|_| {
            let pkg_name = format!("{}/{}", "owner", PKG_NAME);
            let version = String::from(CURRENT_VERSION);
            let result = check_version(Crates, &pkg_name, &version, ONE_DAY);

            assert!(result.is_ok());
        });
    }

    #[test]
    fn stub_check_version_test() {
        let version = "1.0.0";
        let result = stub_check_version(Crates, PKG_NAME, CURRENT_VERSION, ONE_DAY, version);
        let version = Version::parse(version).expect("parse version");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(version));
    }
}
