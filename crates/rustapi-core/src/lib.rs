//! # RustAPI Core
//!
//! Core library providing the foundational types and traits for RustAPI.
//!
//! This crate is not meant to be used directly. Use `rustapi-rs` instead.

mod app;
mod error;
mod extract;
mod handler;
pub mod middleware;
mod request;
mod response;
mod router;
mod server;
pub mod sse;
pub mod stream;

// Public API
pub use app::RustApi;
pub use error::{ApiError, Result};
pub use extract::{Body, ClientIp, Extension, FromRequest, FromRequestParts, HeaderValue, Headers, Json, Path, Query, State, ValidatedJson};
#[cfg(feature = "cookies")]
pub use extract::Cookies;
pub use handler::{
    Handler, HandlerService, Route, RouteHandler,
    get_route, post_route, put_route, patch_route, delete_route,
};
pub use middleware::{RequestId, RequestIdLayer, TracingLayer};
pub use request::Request;
pub use response::{Created, Html, IntoResponse, NoContent, Redirect, Response, WithStatus};
pub use router::{delete, get, patch, post, put, MethodRouter, Router};
pub use sse::{Sse, SseEvent};
pub use stream::StreamBody;
