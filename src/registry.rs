use crate::{Error, Package};

pub use crates::Crates;
pub use github::GitHub;

mod crates;
mod github;

pub trait Registry {
    /// Gets the latest version of a package from the registry.
    ///
    /// # Arguments
    ///
    /// * `pkg` - A `Package` struct.
    fn get_latest_version(pkg: &Package) -> Result<Option<String>, Error>;
}
