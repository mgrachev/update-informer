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

pub mod registry;

type Error = Box<dyn std::error::Error>;

pub fn check_version<S, N, V>(
    _service: S,
    name: N,
    version: V,
    interval: Duration,
) -> Result<Option<Version>, Error>
where
    S: Registry,
    N: AsRef<str>,
    V: AsRef<str>,
{
    let pkg = Package::new(name.as_ref());
    let latest_version_file = VersionFile::new(&pkg, version.as_ref());
    let last_modified = latest_version_file.last_modified()?;

    let latest_version = if last_modified >= interval {
        // This is needed to update mtime of the file
        latest_version_file.recreate_file()?;

        match S::get_latest_version(&pkg)? {
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
}
