use crate::{http_client::SendRequest, Result};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub(crate) struct ReqwestHttpClient;

impl SendRequest for ReqwestHttpClient {
    fn get<T: DeserializeOwned>(
        url: &str,
        timeout: Duration,
        headers: Option<(&str, &str)>,
    ) -> Result<T> {
        let mut req = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .build()?
            .get(url);

        if let Some((key, val)) = headers {
            req = req.header(key, val)
        }

        let json = req.send()?.json()?;

        Ok(json)
    }
}
