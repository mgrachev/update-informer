use serde::Deserialize;

use crate::registry::Registry;
use crate::Error;

static BASE_URL: &str = "https://crates.io./api/v1/crates";

#[derive(Deserialize)]
struct Response {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
struct Version {
    num: String,
}

pub struct Crates;

impl Registry for Crates {
    fn get_latest_version(pkg_name: &str) -> Result<Option<String>, Error> {
        let url = format!("{}/{}/{}", BASE_URL, pkg_name, "versions");

        let resp: Response = ureq::get(&url).call()?.into_json()?;

        if let Some(v) = resp.versions.first() {
            return Ok(Some(v.num.clone()));
        }

        Ok(None)
    }
}
