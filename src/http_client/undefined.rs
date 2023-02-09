use crate::{http_client::SendRequest, Result};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub struct UndefinedHttpClient;

impl SendRequest for UndefinedHttpClient {
    fn get<T: DeserializeOwned>(
        _url: &str,
        _timeout: Duration,
        _headers: Option<(&str, &str)>,
    ) -> Result<T> {
        panic!("choose HTTP client (ureq or reqwest) or implement your own");
    }
}
