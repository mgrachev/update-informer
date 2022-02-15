use crate::Error;
use serde::de::DeserializeOwned;
use std::time::Duration;

pub(crate) struct Client {
    request: ureq::Request,
}

pub(crate) fn get(url: &str, timeout: Duration) -> Client {
    Client {
        request: ureq::agent().get(url).timeout(timeout),
    }
}

impl Client {
    pub(crate) fn add_header(mut self, key: &str, value: &str) -> Self {
        self.request = self.request.set(key, value);
        self
    }

    pub(crate) fn call<T: DeserializeOwned>(self) -> Result<T, Error> {
        let json = self.request.call()?.into_json()?;

        Ok(json)
    }
}
