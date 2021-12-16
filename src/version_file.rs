use crate::{Error, Package};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io};

#[derive(Debug, PartialEq)]
pub(crate) struct VersionFile<'a> {
    path: PathBuf,
    version: &'a str,
}

const VERSION_SUFFIX: &str = "latest-version";

impl<'a> VersionFile<'a> {
    pub(crate) fn new(pkg: &Package, version: &'a str) -> Self {
        let file_name = format!(".{}-{}", pkg.name, VERSION_SUFFIX);
        let path = home_path().join(file_name);

        Self { path, version }
    }

    pub(crate) fn last_modified(&self) -> Result<Duration, Error> {
        let metadata = match fs::metadata(&self.path) {
            Ok(meta) => meta,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                self.write_version(&self.version)?;
                return Ok(Duration::default());
            }
            Err(e) => return Err(e.into()),
        };

        let last_modified = metadata.modified()?.elapsed();
        Ok(last_modified.unwrap_or_default())
    }

    pub(crate) fn recreate_file(&self) -> io::Result<()> {
        fs::remove_file(&self.path)?;
        self.write_version(&self.version)
    }

    pub(crate) fn write_version<V: AsRef<str>>(&self, version: V) -> io::Result<()> {
        fs::write(&self.path, version.as_ref())
    }

    pub(crate) fn get_version(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

#[cfg(not(test))]
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn home_path() -> PathBuf {
    PathBuf::from(env!("HOME"))
}

#[cfg(not(test))]
#[cfg(target_os = "windows")]
fn home_path() -> PathBuf {
    PathBuf::from(env!("USERPROFILE")) // Or use $HOMEPATH?
}

#[cfg(test)]
fn home_path() -> PathBuf {
    std::env::temp_dir().join("update-informer-test")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::within_test_dir;

    #[test]
    fn new_test() {
        let pkg = Package::new("repo");
        let version_file1 = VersionFile::new(&pkg, "0.1.0");
        let version_file2 = VersionFile {
            path: home_path().join(".repo-latest-version"),
            version: "0.1.0",
        };

        assert_eq!(version_file1, version_file2);
    }

    #[test]
    fn last_modified_file_exists_test() {
        within_test_dir(|path| {
            fs::write(&path, "0.1.0").expect("creates test file");

            let version_file = VersionFile {
                path,
                version: "0.1.0",
            };

            let last_modified = version_file.last_modified();
            assert!(last_modified.is_ok());
            assert!(!last_modified.unwrap().is_zero());
        });
    }

    #[test]
    fn last_modified_file_not_exists_test() {
        within_test_dir(|path| {
            let version_file = VersionFile {
                path: path.clone(),
                version: "0.1.0",
            };

            let last_modified = version_file.last_modified();
            assert!(last_modified.is_ok());
            assert!(last_modified.unwrap().is_zero());

            let version = fs::read_to_string(&path).expect("read test file");
            assert_eq!(version, "0.1.0");
        });
    }

    #[test]
    fn recreate_file_test() {
        within_test_dir(|path| {
            fs::write(&path, "0.1.0").expect("creates test file");

            let version_file = VersionFile {
                path: path.clone(),
                version: "1.0.0",
            };

            let result = version_file.recreate_file();
            assert!(result.is_ok());

            let version = fs::read_to_string(&path).expect("read test file");
            assert_eq!(version, "1.0.0");
        });
    }

    #[test]
    fn write_version_test() {
        within_test_dir(|path| {
            fs::write(&path, "1.0.0").expect("creates test file");

            let version_file = VersionFile {
                path: path.clone(),
                version: "1.0.0",
            };

            let result = version_file.write_version("2.0.0");
            assert!(result.is_ok());

            let version = fs::read_to_string(&path).expect("read test file");
            assert_eq!(version, "2.0.0");
        });
    }

    #[test]
    fn get_version_file_exists_test() {
        within_test_dir(|path| {
            fs::write(&path, "1.0.0").expect("creates test file");

            let version_file = VersionFile {
                path: path.clone(),
                version: "1.0.0",
            };

            let result = version_file.get_version();
            assert!(result.is_ok());

            let version = fs::read_to_string(&path).expect("read test file");
            assert_eq!(version, "1.0.0");
        });
    }

    #[test]
    fn get_version_file_not_exists_test() {
        within_test_dir(|path| {
            let version_file = VersionFile {
                path: path.clone(),
                version: "1.0.0",
            };

            let result = version_file.get_version();
            assert!(result.is_err());
        });
    }
}
