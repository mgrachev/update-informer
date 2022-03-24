use crate::Result;
use serde::de::DeserializeOwned;
use std::time::Duration;

/// HTTP client
pub(crate) struct Client {
    request: ureq::Request,
}

/// Initializes GET request
pub(crate) fn get(url: &str, timeout: Duration) -> Client {
    Client {
        request: ureq::agent().get(url).timeout(timeout),
    }
}

impl Client {
    #[allow(dead_code)] // Used only for GitHub
    /// Adds HTTP header to request
    pub(crate) fn add_header(mut self, key: &str, value: &str) -> Self {
        self.request = self.request.set(key, value);
        self
    }

    /// Sends a request and parses it to JSON
    pub(crate) fn call<T: DeserializeOwned>(self) -> Result<T> {
        let json = self.request.call()?.into_json()?;

        Ok(json)
    }
}
