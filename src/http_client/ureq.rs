use crate::{
    http_client::{HeaderMap, HttpClient},
    Result,
};
use serde::de::DeserializeOwned;
use std::time::Duration;
use ureq::Agent;

pub struct UreqHttpClient;

impl HttpClient for UreqHttpClient {
    fn get<T: DeserializeOwned>(url: &str, timeout: Duration, headers: HeaderMap) -> Result<T> {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(timeout))
            .build()
            .into();

        let mut req = agent.get(url);

        for (header, value) in headers {
            req = req.header(header, value);
        }

        let json = req.call()?.body_mut().read_json()?;

        Ok(json)
    }
}
