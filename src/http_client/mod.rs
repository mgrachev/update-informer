use crate::Result;
use serde::de::DeserializeOwned;
use std::time::Duration;

#[cfg(feature = "ureq")]
mod ureq;
#[cfg(feature = "ureq")]
pub use crate::http_client::ureq::UreqHttpClient;
#[cfg(feature = "ureq")]
pub type DefaultHttpClient = UreqHttpClient;

#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(all(feature = "reqwest", not(feature = "ureq")))]
pub use crate::http_client::reqwest::ReqwestHttpClient;
#[cfg(all(feature = "reqwest", not(feature = "ureq")))]
pub type DefaultHttpClient = ReqwestHttpClient;

/// An HTTP client to send requests to the registry.
pub struct HttpClient<'a, T: SendRequest> {
    _inner: T,
    timeout: Duration,
    headers: Option<(&'a str, &'a str)>,
}

pub(crate) fn new<'a, T: SendRequest>(client: T, timeout: Duration) -> HttpClient<'a, T> {
    HttpClient {
        _inner: client,
        timeout,
        headers: None,
    }
}

impl<'a, T: SendRequest> HttpClient<'a, T> {
    pub(crate) fn headers(self, headers: (&'a str, &'a str)) -> Self {
        Self {
            headers: Some(headers),
            ..self
        }
    }

    pub(crate) fn get<D: DeserializeOwned>(self, url: &str) -> Result<D> {
        T::get(url, self.timeout, self.headers)
    }
}

pub trait SendRequest {
    fn get<T: DeserializeOwned>(
        url: &str,
        timeout: Duration,
        headers: Option<(&str, &str)>,
    ) -> Result<T>;
}
