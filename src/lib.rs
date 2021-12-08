use crate::version::Version;

mod crates;
mod version;

type Error = Box<dyn std::error::Error>;

pub fn check_version(pkg_name: &str, version: &str) -> Result<Option<Version>, Error> {
    let latest_version = crates::get_latest_version(pkg_name)?;

    let current_version = semver::Version::parse(version)?;

    if let Some(v) = latest_version {
        if v > current_version {
            return Ok(Some(v.into()));
        }
    }

    Ok(None)
}
