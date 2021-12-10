use crate::registry::Registry;
use crate::version::Version;

pub mod registry;
mod version;

type Error = Box<dyn std::error::Error>;

pub fn check_version<T>(_svc: T, pkg_name: &str, version: &str) -> Result<Option<Version>, Error>
where
    T: Registry,
{
    let current_version = semver::Version::parse(version)?;
    let latest_version = T::get_latest_version(pkg_name)?;

    if let Some(v) = latest_version {
        let v = semver::Version::parse(&v)?;

        if v > current_version {
            return Ok(Some(v.into()));
        }
    }

    Ok(None)
}
