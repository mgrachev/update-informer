use crate::Error;

pub use crates::Crates;
pub use github::GitHub;

mod crates;
mod github;

pub trait Registry {
    fn get_latest_version(pkg_name: &str) -> Result<Option<String>, Error>;
}
