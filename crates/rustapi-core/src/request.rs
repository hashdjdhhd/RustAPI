//! Request types for RustAPI

use bytes::Bytes;
use http::{request::Parts, Extensions, HeaderMap, Method, Uri, Version};
use std::collections::HashMap;
use std::sync::Arc;

/// HTTP Request wrapper
///
/// Provides access to all parts of an incoming HTTP request.
pub struct Request {
    pub(crate) parts: Parts,
    pub(crate) body: Option<Bytes>,
    pub(crate) state: Arc<Extensions>,
    pub(crate) path_params: HashMap<String, String>,
}

impl Request {
    /// Create a new request from parts
    pub(crate) fn new(
        parts: Parts,
        body: Bytes,
        state: Arc<Extensions>,
        path_params: HashMap<String, String>,
    ) -> Self {
        Self {
            parts,
            body: Some(body),
            state,
            path_params,
        }
    }

    /// Get the HTTP method
    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    /// Get the URI
    pub fn uri(&self) -> &Uri {
        &self.parts.uri
    }

    /// Get the HTTP version
    pub fn version(&self) -> Version {
        self.parts.version
    }

    /// Get the headers
    pub fn headers(&self) -> &HeaderMap {
        &self.parts.headers
    }

    /// Get request extensions
    pub fn extensions(&self) -> &Extensions {
        &self.parts.extensions
    }

    /// Get mutable extensions
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.parts.extensions
    }

    /// Get the request path
    pub fn path(&self) -> &str {
        self.parts.uri.path()
    }

    /// Get the query string
    pub fn query_string(&self) -> Option<&str> {
        self.parts.uri.query()
    }

    /// Take the body bytes (can only be called once)
    pub fn take_body(&mut self) -> Option<Bytes> {
        self.body.take()
    }

    /// Get path parameters
    pub fn path_params(&self) -> &HashMap<String, String> {
        &self.path_params
    }

    /// Get a specific path parameter
    pub fn path_param(&self, name: &str) -> Option<&String> {
        self.path_params.get(name)
    }

    /// Get shared state
    pub fn state(&self) -> &Arc<Extensions> {
        &self.state
    }
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.parts.method)
            .field("uri", &self.parts.uri)
            .field("version", &self.parts.version)
            .finish()
    }
}
