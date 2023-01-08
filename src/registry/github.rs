use crate::{
    http_client::{HttpClient, SendRequest},
    Package, Registry, Result, Version,
};
use serde::Deserialize;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const REGISTRY_URL: &str = "https://api.github.com";

#[derive(Deserialize)]
struct Response {
    tag_name: String,
}

/// The most popular and largest project hosting.
pub struct GitHub;

#[cfg(not(test))]
fn get_base_url() -> String {
    format!("{}/repos", REGISTRY_URL)
}

#[cfg(test)]
fn get_base_url() -> String {
    format!("{}/repos", &mockito::server_url())
}

impl Registry for GitHub {
    const NAME: &'static str = "github";

    fn get_latest_version<T: SendRequest>(
        http_client: HttpClient<T>,
        pkg: &Package,
        _current_version: &Version,
    ) -> Result<Option<String>> {
        let url = format!("{}/{}/releases/latest", get_base_url(), pkg);
        let resp = http_client
            .headers(("Accept", "application/vnd.github.v3+json"))
            .get::<Response>(&url)?;

        if resp.tag_name.starts_with('v') {
            return Ok(Some(resp.tag_name[1..].to_string()));
        }

        Ok(Some(resp.tag_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client;
    use crate::test_helper::mock_github;
    use std::time::Duration;

    const PKG_NAME: &str = "owner/repo";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/github";
    const TIMEOUT: Duration = Duration::from_secs(5);

    #[test]
    fn failure_test() {
        let pkg = Package::new(PKG_NAME);
        let client = http_client::new(http_client::UreqHttpClient, TIMEOUT);
        let data_path = format!("{}/not_found.json", FIXTURES_PATH);
        let _mock = mock_github(&pkg, 404, &data_path);
        let current_version = Version::parse("0.1.0").expect("parse version");

        let result = GitHub::get_latest_version(client, &pkg, &current_version);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let pkg = Package::new(PKG_NAME);
        let client = http_client::new(http_client::UreqHttpClient, TIMEOUT);
        let data_path = format!("{}/release.json", FIXTURES_PATH);
        let (_mock, data) = mock_github(&pkg, 200, &data_path);

        let json: Response = serde_json::from_str(&data).expect("deserialize json");
        let latest_version = json.tag_name[1..].to_string();
        let current_version = Version::parse("1.6.3-canary.0").expect("parse version");

        let result = GitHub::get_latest_version(client, &pkg, &current_version);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}
