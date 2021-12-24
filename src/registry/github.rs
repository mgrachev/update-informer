use crate::registry::Registry;
use crate::{Error, Package};
use serde::Deserialize;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const REGISTRY_URL: &str = "https://api.github.com";

#[derive(Deserialize)]
struct Response {
    tag_name: String,
}

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
    fn get_latest_version(pkg: &Package) -> Result<Option<String>, Error> {
        let url = format!("{}/{}/releases/latest", get_base_url(), pkg);

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
    use crate::test_helper::mock_github;

    const PKG_NAME: &str = "owner/repo";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/github";

    #[test]
    fn failure_test() {
        let pkg = Package::new(PKG_NAME);
        let data_path = format!("{}/not_found.json", FIXTURES_PATH);
        let _mock = mock_github(&pkg, 404, &data_path);

        let result = GitHub::get_latest_version(&pkg);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let pkg = Package::new(PKG_NAME);
        let data_path = format!("{}/release.json", FIXTURES_PATH);
        let (_mock, data) = mock_github(&pkg, 200, &data_path);

        let json: Response = serde_json::from_str(&data).expect("deserialize json");
        let latest_version = json.tag_name[1..].to_string();

        let result = GitHub::get_latest_version(&pkg);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}
