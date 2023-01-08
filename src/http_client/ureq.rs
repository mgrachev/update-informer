use crate::{http_client::SendRequest, Result};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub(crate) struct UreqHttpClient;

impl SendRequest for UreqHttpClient {
    fn get<T: DeserializeOwned>(
        url: &str,
        timeout: Duration,
        headers: Option<(&str, &str)>,
    ) -> Result<T> {
        let mut req = ureq::agent().get(url).timeout(timeout);

        if let Some((key, value)) = headers {
            req = req.set(key, value);
        }

        let json = req.call()?.into_json()?;

        Ok(json)
    }
}
