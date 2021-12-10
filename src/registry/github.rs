use serde::Deserialize;

use crate::registry::Registry;
use crate::Error;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const REGISTRY_URL: &str = "https://api.github.com";

#[derive(Deserialize)]
struct Response {
    tag_name: String,
}

pub struct GitHub;

fn get_base_url() -> String {
    #[cfg(not(test))]
    let base_url = format!("{}/repos", REGISTRY_URL);

    #[cfg(test)]
    let base_url = format!("{}/repos", &mockito::server_url());

    base_url
}

impl Registry for GitHub {
    fn get_latest_version(pkg_name: &str) -> Result<Option<String>, Error> {
        let url = format!("{}/{}/releases/latest", get_base_url(), pkg_name);

        let resp: Response = ureq::get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()?
            .into_json()?;

        if resp.tag_name.starts_with('v') {
            return Ok(Some(resp.tag_name[1..].to_string()));
        }

        Ok(Some(resp.tag_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;

    const PKG_NAME: &str = "dotenv-linter/dotenv-linter";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/github";

    #[test]
    fn failure_test() {
        let mock_path = format!("/repos/{}/releases/latest", PKG_NAME);
        let data = fs::read_to_string(format!("{}/not_found.json", FIXTURES_PATH))
            .expect("read file to string");

        let _mock = mock("GET", mock_path.as_str())
            .with_status(404)
            .with_header("Content-Type", "application/json; charset=utf-8")
            .with_body(data.to_string())
            .create();

        let result = GitHub::get_latest_version(PKG_NAME);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let mock_path = format!("/repos/{}/releases/latest", PKG_NAME);
        let data = fs::read_to_string(format!("{}/release.json", FIXTURES_PATH))
            .expect("read file to string");

        let json: Response = serde_json::from_str(&data).expect("deserialize json");
        let latest_version = json.tag_name[1..].to_string();

        let _mock = mock("GET", mock_path.as_str())
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=utf-8")
            .with_body(data.to_string())
            .create();

        let result = GitHub::get_latest_version(PKG_NAME);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}
