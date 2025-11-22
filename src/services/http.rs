use reqwest::{Client, RequestBuilder, Method, Url};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

pub struct Http;

impl Http {
    /// Make a GET request
    pub fn get(url: &str) -> RequestBuilder {
        Client::new().get(url)
    }

    /// Make a POST request
    pub fn post(url: &str) -> RequestBuilder {
        Client::new().post(url)
    }

    /// Make a PUT request
    pub fn put(url: &str) -> RequestBuilder {
        Client::new().put(url)
    }

    /// Make a PATCH request
    pub fn patch(url: &str) -> RequestBuilder {
        Client::new().patch(url)
    }

    /// Make a DELETE request
    pub fn delete(url: &str) -> RequestBuilder {
        Client::new().delete(url)
    }

    /// Create a new pending request with headers
    pub fn with_headers(headers: HashMap<String, String>) -> RequestBuilderWrapper {
        RequestBuilderWrapper::new().with_headers(headers)
    }

    /// Create a new pending request with a bearer token
    pub fn with_token(token: &str) -> RequestBuilderWrapper {
        RequestBuilderWrapper::new().with_token(token)
    }

    /// Indicate that the request expects JSON response
    pub fn accept_json() -> RequestBuilderWrapper {
        RequestBuilderWrapper::new().accept_json()
    }

    /// Indicate that the request sends JSON
    pub fn as_json() -> RequestBuilderWrapper {
        RequestBuilderWrapper::new().as_json()
    }

    /// Indicate that the request sends Form data
    pub fn as_form() -> RequestBuilderWrapper {
        RequestBuilderWrapper::new().as_form()
    }

    /// Set the timeout for the request
    pub fn timeout(seconds: u64) -> RequestBuilderWrapper {
        RequestBuilderWrapper::new().timeout(Duration::from_secs(seconds))
    }
}

pub struct RequestBuilderWrapper {
    client: Client,
    headers: Option<HashMap<String, String>>,
    token: Option<String>,
    timeout: Option<Duration>,
}

impl RequestBuilderWrapper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            headers: None,
            token: None,
            timeout: None,
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        let mut current = self.headers.unwrap_or_default();
        current.extend(headers);
        self.headers = Some(current);
        self
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn accept_json(mut self) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert("Accept".to_string(), "application/json".to_string());
        self.headers = Some(headers);
        self
    }

    pub fn as_json(mut self) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.headers = Some(headers);
        self
    }

    pub fn as_form(mut self) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());
        self.headers = Some(headers);
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    fn apply_options(&self, mut req: RequestBuilder) -> RequestBuilder {
        if let Some(headers) = &self.headers {
            for (k, v) in headers {
                req = req.header(k, v);
            }
        }
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout);
        }
        req
    }

    pub fn get(self, url: &str) -> RequestBuilder {
        self.apply_options(self.client.get(url))
    }

    pub fn post(self, url: &str) -> RequestBuilder {
        self.apply_options(self.client.post(url))
    }

    pub fn put(self, url: &str) -> RequestBuilder {
        self.apply_options(self.client.put(url))
    }

    pub fn patch(self, url: &str) -> RequestBuilder {
        self.apply_options(self.client.patch(url))
    }

    pub fn delete(self, url: &str) -> RequestBuilder {
        self.apply_options(self.client.delete(url))
    }
}
