use std::fmt::{Display, Formatter};

pub struct Version(semver::Version);

impl From<semver::Version> for Version {
    fn from(v: semver::Version) -> Self {
        Self(v)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}
