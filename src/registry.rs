use crate::{Error, Package};

pub use crates::Crates;
pub use github::GitHub;

mod crates;
mod github;

pub trait Registry {
    fn get_latest_version(pkg: &Package) -> Result<Option<String>, Error>;
}
