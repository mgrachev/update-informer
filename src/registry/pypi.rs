use crate::{http, Package, Registry, Result, Version};
use serde::Deserialize;
use std::time::Duration;

#[cfg(test)]
use mockito;

#[derive(Deserialize, Debug)]
struct Response {
    info: Info,
}

#[derive(Deserialize, Debug)]
struct Info {
    version: String,
    yanked: bool,
}

/// The Python community’s package registry.
pub struct PyPI;

#[cfg(not(test))]
fn get_base_url() -> String {
    "https://pypi.org/pypi".to_string()
}

#[cfg(test)]
fn get_base_url() -> String {
    format!("{}/pypi", &mockito::server_url())
}

impl Registry for PyPI {
    const NAME: &'static str = "pypi";

    fn get_latest_version(
        pkg: &Package,
        _current_version: &Version,
        timeout: Duration,
    ) -> Result<Option<String>> {
        let url = format!("{}/{}/json", get_base_url(), pkg);

        let resp: Response = http::get(&url, timeout).call()?;

        if !resp.info.yanked {
            return Ok(Some(resp.info.version));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::mock_pypi;

    const PKG_NAME: &str = "filprofiler";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/pypi";
    const TIMEOUT: Duration = Duration::from_secs(5);

    #[test]
    fn failure_test() {
        let pkg = Package::new(PKG_NAME);
        let data_path = format!("{}/not_found.html", FIXTURES_PATH);
        let _mock = mock_pypi(&pkg, 404, &data_path);
        let current_version = Version::parse("0.1.0").expect("parse version");

        let result = PyPI::get_latest_version(&pkg, &current_version, TIMEOUT);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let pkg = Package::new(PKG_NAME);
        let data_path = format!("{}/release.json", FIXTURES_PATH);
        let (_mock, _data) = mock_pypi(&pkg, 200, &data_path);
        let current_version = Version::parse("0.1.0").expect("parse version");

        let latest_version = "2022.1.1".to_string();
        let result = PyPI::get_latest_version(&pkg, &current_version, TIMEOUT);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}
