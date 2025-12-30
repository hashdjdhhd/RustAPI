//! Router implementation using radix tree (matchit)

use crate::error::ApiError;
use crate::handler::{into_boxed_handler, BoxedHandler, Handler};
use crate::request::Request;
use crate::response::{IntoResponse, Response};
use bytes::Bytes;
use http::{Extensions, Method, StatusCode};
use http_body_util::Full;
use matchit::Router as MatchitRouter;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// HTTP method router for a single path
pub struct MethodRouter {
    handlers: HashMap<Method, BoxedHandler>,
}

impl MethodRouter {
    /// Create a new empty method router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Add a handler for a specific method
    fn on(mut self, method: Method, handler: BoxedHandler) -> Self {
        self.handlers.insert(method, handler);
        self
    }

    /// Get handler for a method
    pub(crate) fn get_handler(&self, method: &Method) -> Option<&BoxedHandler> {
        self.handlers.get(method)
    }

    /// Get allowed methods for 405 response
    pub(crate) fn allowed_methods(&self) -> Vec<Method> {
        self.handlers.keys().cloned().collect()
    }
}

impl Default for MethodRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a GET route handler
pub fn get<H, T>(handler: H) -> MethodRouter
where
    H: Handler<T>,
    T: 'static,
{
    MethodRouter::new().on(Method::GET, into_boxed_handler(handler))
}

/// Create a POST route handler
pub fn post<H, T>(handler: H) -> MethodRouter
where
    H: Handler<T>,
    T: 'static,
{
    MethodRouter::new().on(Method::POST, into_boxed_handler(handler))
}

/// Create a PUT route handler
pub fn put<H, T>(handler: H) -> MethodRouter
where
    H: Handler<T>,
    T: 'static,
{
    MethodRouter::new().on(Method::PUT, into_boxed_handler(handler))
}

/// Create a PATCH route handler
pub fn patch<H, T>(handler: H) -> MethodRouter
where
    H: Handler<T>,
    T: 'static,
{
    MethodRouter::new().on(Method::PATCH, into_boxed_handler(handler))
}

/// Create a DELETE route handler
pub fn delete<H, T>(handler: H) -> MethodRouter
where
    H: Handler<T>,
    T: 'static,
{
    MethodRouter::new().on(Method::DELETE, into_boxed_handler(handler))
}

/// Main router
pub struct Router {
    inner: MatchitRouter<MethodRouter>,
    state: Arc<Extensions>,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self {
            inner: MatchitRouter::new(),
            state: Arc::new(Extensions::new()),
        }
    }

    /// Add a route
    pub fn route(mut self, path: &str, method_router: MethodRouter) -> Self {
        // Convert {param} style to :param for matchit
        let matchit_path = convert_path_params(path);
        
        match self.inner.insert(matchit_path.clone(), method_router) {
            Ok(_) => {}
            Err(e) => {
                panic!("Route conflict: {} - {}", path, e);
            }
        }
        self
    }

    /// Add application state
    pub fn state<S: Clone + Send + Sync + 'static>(mut self, state: S) -> Self {
        let extensions = Arc::make_mut(&mut self.state);
        extensions.insert(state);
        self
    }

    /// Nest another router under a prefix
    pub fn nest(self, _prefix: &str, _router: Router) -> Self {
        // TODO: Implement router nesting
        self
    }

    /// Match a request and return the handler + params
    pub(crate) fn match_route(
        &self,
        path: &str,
        method: &Method,
    ) -> RouteMatch {
        match self.inner.at(path) {
            Ok(matched) => {
                let method_router = matched.value;
                
                if let Some(handler) = method_router.get_handler(method) {
                    // Convert params to HashMap
                    let params: HashMap<String, String> = matched
                        .params
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    
                    RouteMatch::Found { handler, params }
                } else {
                    RouteMatch::MethodNotAllowed {
                        allowed: method_router.allowed_methods(),
                    }
                }
            }
            Err(_) => RouteMatch::NotFound,
        }
    }

    /// Get shared state
    pub(crate) fn state_ref(&self) -> Arc<Extensions> {
        self.state.clone()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of route matching
pub(crate) enum RouteMatch<'a> {
    Found {
        handler: &'a BoxedHandler,
        params: HashMap<String, String>,
    },
    NotFound,
    MethodNotAllowed {
        allowed: Vec<Method>,
    },
}

/// Convert {param} style to :param for matchit
fn convert_path_params(path: &str) -> String {
    let mut result = String::with_capacity(path.len());
    let mut in_param = false;
    
    for ch in path.chars() {
        match ch {
            '{' => {
                in_param = true;
                result.push(':');
            }
            '}' => {
                in_param = false;
            }
            _ => {
                result.push(ch);
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_path_params() {
        assert_eq!(convert_path_params("/users/{id}"), "/users/:id");
        assert_eq!(
            convert_path_params("/users/{user_id}/posts/{post_id}"),
            "/users/:user_id/posts/:post_id"
        );
        assert_eq!(convert_path_params("/static/path"), "/static/path");
    }
}
