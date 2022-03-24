use crate::{Package, Result};
use std::time::Duration;

#[cfg(feature = "crates")]
pub use crates::Crates;

#[cfg(feature = "github")]
pub use github::GitHub;

#[cfg(feature = "pypi")]
pub use pypi::PyPI;

#[cfg(feature = "crates")]
mod crates;

#[cfg(feature = "github")]
mod github;

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
    fn get_latest_version(pkg: &Package, timeout: Duration) -> Result<Option<String>>;
}
