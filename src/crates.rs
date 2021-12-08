use serde::Deserialize;

use crate::Error;

static BASE_URL: &str = "https://crates.io./api/v1/crates";

#[derive(Deserialize)]
struct CratesResponse {
    versions: Vec<CratesVersion>,
}

#[derive(Deserialize)]
struct CratesVersion {
    num: String,
}

pub(crate) fn get_latest_version(pkg_name: &str) -> Result<Option<semver::Version>, Error> {
    let url = format!("{}/{}/{}", BASE_URL, pkg_name, "versions");

    let resp: CratesResponse = ureq::get(&url).call()?.into_json()?;

    if let Some(v) = resp.versions.first() {
        let version = semver::Version::parse(&v.num)?;

        return Ok(Some(version));
    }

    Ok(None)
}
