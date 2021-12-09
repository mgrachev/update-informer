use serde::Deserialize;

use crate::registry::Registry;
use crate::Error;

static BASE_URL: &str = "https://api.github.com/repos";

#[derive(Deserialize, Debug)]
struct Response {
    tag_name: String,
}

pub struct GitHub;

impl Registry for GitHub {
    fn get_latest_version(pkg_name: &str) -> Result<Option<String>, Error> {
        let url = format!("{}/{}/{}", BASE_URL, pkg_name, "releases/latest");

        let mut resp: Response = ureq::get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()?
            .into_json()?;

        if resp.tag_name.starts_with('v') {
            resp.tag_name.remove(0);
        }

        Ok(Some(resp.tag_name))
    }
}
