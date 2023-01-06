use crate::{Package, Result, Version};
use std::time::Duration;

#[cfg(feature = "crates")]
pub use crates::Crates;

#[cfg(feature = "github")]
pub use github::GitHub;

#[cfg(feature = "npm")]
pub use npm::Npm;

#[cfg(feature = "pypi")]
pub use pypi::PyPI;

#[cfg(feature = "crates")]
mod crates;

#[cfg(feature = "github")]
mod github;

#[cfg(feature = "npm")]
mod npm;

#[cfg(feature = "pypi")]
mod pypi;

pub trait Registry {
    /// The name of the registry.
    const NAME: &'static str;

    /// Gets the latest version of a package from the registry.
    ///
    /// # Arguments
    ///
    /// * `pkg` - A `Package` struct.
    /// * `current_version` - The current version of the package.
    /// * `timeout` - A request timeout.
    fn get_latest_version(
        pkg: &Package,
        current_version: &Version,
        timeout: Duration,
    ) -> Result<Option<String>>;
}
