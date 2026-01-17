use std::collections::BTreeMap;
use ::url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get
}

#[derive(Debug, Clone, Default)]
pub struct Headers(BTreeMap<String, String>);

impl Headers {
    pub fn new() -> Self { Self(BTreeMap::new()) }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Query(BTreeMap<String, String>);

impl Query {
    pub fn new() -> Self { Self(BTreeMap::new()) }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

#[derive(Debug, Clone)]
pub enum Body {
    Empty,
    Bytes(Vec<u8>),
    Json(Vec<u8>),
}

impl Default for Body {
    fn default() -> Self { Body::Empty }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub url: Url,
    pub headers: Headers,
    pub query: Query,
    pub body: Body,
}

impl HttpRequest {
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            query: Query::new(),
            body: Body::Empty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn body_as_str_lossy(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}