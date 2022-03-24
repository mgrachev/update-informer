use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
/// A package representation.
pub struct Package<'a> {
    owner: Option<&'a str>,
    name: &'a str,
}

impl<'a> Package<'a> {
    pub(crate) fn new(name: &'a str) -> Self {
        if !name.contains('/') {
            return Self { owner: None, name };
        }

        let parts: Vec<&str> = name.split('/').collect();

        Self {
            owner: Some(parts[0]),
            name: parts[1],
        }
    }

    /// Return a name suitable for storing on filesystem, that will include
    /// owner if it is set.
    pub(crate) fn name(&self) -> String {
        let owner = self.owner.map(|s| format!("{}-", s)).unwrap_or_default();
        format!("{}{}", owner, self.name)
    }
}

impl Display for Package<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.owner.is_none() {
            return write!(f, "{}", self.name);
        }

        write!(f, "{}/{}", self.owner.as_ref().unwrap(), self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::Package;

    #[test]
    fn new_with_name_test() {
        let pkg1 = Package::new("repo");
        let pkg2 = Package {
            owner: None,
            name: "repo",
        };

        assert_eq!(pkg1, pkg2);
        assert_eq!(pkg1.name(), "repo".to_string())
    }

    #[test]
    fn new_with_owner_and_name_test() {
        let pkg1 = Package::new("owner/repo");
        let pkg2 = Package {
            owner: Some("owner"),
            name: "repo",
        };

        assert_eq!(pkg1, pkg2);
        assert_eq!(pkg1.name(), "owner-repo".to_string())
    }

    #[test]
    fn name_fmt_test() {
        let pkg = Package::new("repo");
        assert_eq!(String::from("repo"), format!("{}", pkg))
    }

    #[test]
    fn name_with_owner_fmt_test() {
        let pkg = Package::new("owner/repo");
        assert_eq!(String::from("owner/repo"), format!("{}", pkg))
    }
}
