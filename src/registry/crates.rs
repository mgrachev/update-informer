use crate::registry::Registry;
use crate::{Error, Package};
use serde::Deserialize;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const REGISTRY_URL: &str = "https://crates.io";

#[derive(Deserialize)]
struct Response {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
struct Version {
    num: String,
}

/// The Rust community’s crate registry.
pub struct Crates;

#[cfg(not(test))]
fn get_base_url() -> String {
    format!("{}/api/v1/crates", REGISTRY_URL)
}

#[cfg(test)]
fn get_base_url() -> String {
    format!("{}/api/v1/crates", &mockito::server_url())
}

impl Registry for Crates {
    fn get_latest_version(pkg: &Package) -> Result<Option<String>, Error> {
        let url = format!("{}/{}/versions", get_base_url(), pkg);

        let resp: Response = ureq::get(&url).call()?.into_json()?;

        if let Some(v) = resp.versions.first() {
            return Ok(Some(v.num.clone()));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::mock_crates;

    const PKG_NAME: &str = "repo";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/crates";

    #[test]
    fn failure_test() {
        let pkg = Package::new(PKG_NAME);
        let data_path = format!("{}/not_found.json", FIXTURES_PATH);
        let _mock = mock_crates(&pkg, 404, &data_path);

        let result = Crates::get_latest_version(&pkg);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let pkg = Package::new(PKG_NAME);
        let data_path = format!("{}/versions.json", FIXTURES_PATH);
        let (_mock, data) = mock_crates(&pkg, 200, &data_path);

        let json: Response = serde_json::from_str(&data).expect("deserialize json");
        let latest_version = json
            .versions
            .first()
            .expect("get latest version")
            .num
            .clone();

        let result = Crates::get_latest_version(&pkg);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}
