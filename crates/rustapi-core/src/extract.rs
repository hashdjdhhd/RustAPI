//! Extractors for RustAPI
//!
//! Extractors automatically parse and validate data from incoming requests.

use crate::error::{ApiError, Result};
use crate::request::Request;
use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

/// Trait for extracting data from request parts (headers, path, query)
///
/// This is used for extractors that don't need the request body.
pub trait FromRequestParts: Sized {
    /// Extract from request parts
    fn from_request_parts(req: &Request) -> Result<Self>;
}

/// Trait for extracting data from the full request (including body)
///
/// This is used for extractors that consume the request body.
pub trait FromRequest: Sized {
    /// Extract from the full request
    fn from_request(req: &mut Request) -> impl Future<Output = Result<Self>> + Send;
}

// Blanket impl: FromRequestParts -> FromRequest
impl<T: FromRequestParts> FromRequest for T {
    async fn from_request(req: &mut Request) -> Result<Self> {
        T::from_request_parts(req)
    }
}

/// JSON body extractor
///
/// Parses the request body as JSON and deserializes into type `T`.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Deserialize)]
/// struct CreateUser {
///     name: String,
///     email: String,
/// }
///
/// async fn create_user(Json(body): Json<CreateUser>) -> impl IntoResponse {
///     // body is already deserialized
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned + Send> FromRequest for Json<T> {
    async fn from_request(req: &mut Request) -> Result<Self> {
        let body = req.take_body().ok_or_else(|| {
            ApiError::internal("Body already consumed")
        })?;

        let value: T = serde_json::from_slice(&body)?;
        Ok(Json(value))
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Query string extractor
///
/// Parses the query string into type `T`.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Deserialize)]
/// struct Pagination {
///     page: Option<u32>,
///     limit: Option<u32>,
/// }
///
/// async fn list_users(Query(params): Query<Pagination>) -> impl IntoResponse {
///     // params.page, params.limit
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Query<T>(pub T);

impl<T: DeserializeOwned> FromRequestParts for Query<T> {
    fn from_request_parts(req: &Request) -> Result<Self> {
        let query = req.query_string().unwrap_or("");
        let value: T = serde_urlencoded::from_str(query).map_err(|e| {
            ApiError::bad_request(format!("Invalid query string: {}", e))
        })?;
        Ok(Query(value))
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Path parameter extractor
///
/// Extracts path parameters defined in the route pattern.
///
/// # Example
///
/// For route `/users/{id}`:
///
/// ```rust,ignore
/// async fn get_user(Path(id): Path<i64>) -> impl IntoResponse {
///     // id is extracted from path
/// }
/// ```
///
/// For multiple params `/users/{user_id}/posts/{post_id}`:
///
/// ```rust,ignore
/// async fn get_post(Path((user_id, post_id)): Path<(i64, i64)>) -> impl IntoResponse {
///     // Both params extracted
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Path<T>(pub T);

impl<T: FromStr> FromRequestParts for Path<T>
where
    T::Err: std::fmt::Display,
{
    fn from_request_parts(req: &Request) -> Result<Self> {
        let params = req.path_params();
        
        // For single param, get the first one
        if let Some((_, value)) = params.iter().next() {
            let parsed = value.parse::<T>().map_err(|e| {
                ApiError::bad_request(format!("Invalid path parameter: {}", e))
            })?;
            return Ok(Path(parsed));
        }
        
        Err(ApiError::internal("Missing path parameter"))
    }
}

impl<T> Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// State extractor
///
/// Extracts shared application state.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Clone)]
/// struct AppState {
///     db: DbPool,
/// }
///
/// async fn handler(State(state): State<AppState>) -> impl IntoResponse {
///     // Use state.db
/// }
/// ```
#[derive(Debug, Clone)]
pub struct State<T>(pub T);

impl<T: Clone + Send + Sync + 'static> FromRequestParts for State<T> {
    fn from_request_parts(req: &Request) -> Result<Self> {
        req.state()
            .get::<T>()
            .cloned()
            .map(State)
            .ok_or_else(|| {
                ApiError::internal(format!(
                    "State of type `{}` not found. Did you forget to call .state()?",
                    std::any::type_name::<T>()
                ))
            })
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Raw body bytes extractor
#[derive(Debug, Clone)]
pub struct Body(pub Bytes);

impl FromRequest for Body {
    async fn from_request(req: &mut Request) -> Result<Self> {
        let body = req.take_body().ok_or_else(|| {
            ApiError::internal("Body already consumed")
        })?;
        Ok(Body(body))
    }
}

impl Deref for Body {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Optional extractor wrapper
///
/// Makes any extractor optional - returns None instead of error on failure.
impl<T: FromRequestParts> FromRequestParts for Option<T> {
    fn from_request_parts(req: &Request) -> Result<Self> {
        Ok(T::from_request_parts(req).ok())
    }
}

// Implement FromRequestParts for common primitive types (path params)
macro_rules! impl_from_request_parts_for_primitives {
    ($($ty:ty),*) => {
        $(
            impl FromRequestParts for $ty {
                fn from_request_parts(req: &Request) -> Result<Self> {
                    let Path(value) = Path::<$ty>::from_request_parts(req)?;
                    Ok(value)
                }
            }
        )*
    };
}

impl_from_request_parts_for_primitives!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool,
    String
);

// Re-export Json from response for extraction (they share the type)
pub use crate::response::Json as JsonResponse;
