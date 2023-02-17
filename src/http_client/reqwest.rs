use crate::{http_client::HttpClient, Result};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub struct ReqwestHttpClient;

impl HttpClient for ReqwestHttpClient {
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
