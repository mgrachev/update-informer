use serde::Deserialize;

use crate::registry::Registry;
use crate::Error;

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

pub struct Crates;

fn get_base_url() -> String {
    #[cfg(not(test))]
    let base_url = format!("{}/api/v1/crates", REGISTRY_URL);

    #[cfg(test)]
    let base_url = format!("{}/api/v1/crates", &mockito::server_url());

    base_url
}

impl Registry for Crates {
    fn get_latest_version(pkg_name: &str) -> Result<Option<String>, Error> {
        let url = format!("{}/{}/versions", get_base_url(), pkg_name);

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
    use mockito::mock;
    use std::fs;

    const PKG_NAME: &str = "dotenv-linter";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/crates";

    #[test]
    fn failure_test() {
        let mock_path = format!("/api/v1/crates/{}/versions", PKG_NAME);
        let data = fs::read_to_string(format!("{}/not_found.json", FIXTURES_PATH))
            .expect("read file to string");

        let _mock = mock("GET", mock_path.as_str())
            .with_status(404)
            .with_header("Content-Type", "application/json; charset=utf-8")
            .with_body(data.to_string())
            .create();

        let result = Crates::get_latest_version(PKG_NAME);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let mock_path = format!("/api/v1/crates/{}/versions", PKG_NAME);
        let data = fs::read_to_string(format!("{}/versions.json", FIXTURES_PATH))
            .expect("read file to string");

        let json: Response = serde_json::from_str(&data).expect("deserialize json");
        let latest_version = json
            .versions
            .first()
            .expect("get latest version")
            .num
            .clone();

        let _mock = mock("GET", mock_path.as_str())
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=utf-8")
            .with_body(data.to_string())
            .create();

        let result = Crates::get_latest_version(PKG_NAME);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}
