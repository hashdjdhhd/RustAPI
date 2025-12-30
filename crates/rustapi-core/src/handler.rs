//! Handler trait and utilities

use crate::extract::FromRequest;
use crate::request::Request;
use crate::response::{IntoResponse, Response};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

/// Trait representing an async handler function
pub trait Handler<T>: Clone + Send + Sync + Sized + 'static {
    /// The response type
    type Future: Future<Output = Response> + Send + 'static;

    /// Call the handler with the request
    fn call(self, req: Request) -> Self::Future;
}

/// Wrapper to convert a Handler into a tower Service
pub struct HandlerService<H, T> {
    handler: H,
    _marker: PhantomData<fn() -> T>,
}

impl<H, T> HandlerService<H, T> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H: Clone, T> Clone for HandlerService<H, T> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

// Implement Handler for async functions with 0-6 extractors

// 0 args
impl<F, Fut, Res> Handler<()> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, _req: Request) -> Self::Future {
        Box::pin(async move {
            self().await.into_response()
        })
    }
}

// 1 arg
impl<F, Fut, Res, T1> Handler<(T1,)> for F
where
    F: FnOnce(T1) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
    T1: FromRequest + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, mut req: Request) -> Self::Future {
        Box::pin(async move {
            let t1 = match T1::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            self(t1).await.into_response()
        })
    }
}

// 2 args
impl<F, Fut, Res, T1, T2> Handler<(T1, T2)> for F
where
    F: FnOnce(T1, T2) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
    T1: FromRequest + Send + 'static,
    T2: FromRequest + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, mut req: Request) -> Self::Future {
        Box::pin(async move {
            let t1 = match T1::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t2 = match T2::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            self(t1, t2).await.into_response()
        })
    }
}

// 3 args
impl<F, Fut, Res, T1, T2, T3> Handler<(T1, T2, T3)> for F
where
    F: FnOnce(T1, T2, T3) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
    T1: FromRequest + Send + 'static,
    T2: FromRequest + Send + 'static,
    T3: FromRequest + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, mut req: Request) -> Self::Future {
        Box::pin(async move {
            let t1 = match T1::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t2 = match T2::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t3 = match T3::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            self(t1, t2, t3).await.into_response()
        })
    }
}

// 4 args
impl<F, Fut, Res, T1, T2, T3, T4> Handler<(T1, T2, T3, T4)> for F
where
    F: FnOnce(T1, T2, T3, T4) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
    T1: FromRequest + Send + 'static,
    T2: FromRequest + Send + 'static,
    T3: FromRequest + Send + 'static,
    T4: FromRequest + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, mut req: Request) -> Self::Future {
        Box::pin(async move {
            let t1 = match T1::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t2 = match T2::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t3 = match T3::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t4 = match T4::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            self(t1, t2, t3, t4).await.into_response()
        })
    }
}

// 5 args
impl<F, Fut, Res, T1, T2, T3, T4, T5> Handler<(T1, T2, T3, T4, T5)> for F
where
    F: FnOnce(T1, T2, T3, T4, T5) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
    T1: FromRequest + Send + 'static,
    T2: FromRequest + Send + 'static,
    T3: FromRequest + Send + 'static,
    T4: FromRequest + Send + 'static,
    T5: FromRequest + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, mut req: Request) -> Self::Future {
        Box::pin(async move {
            let t1 = match T1::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t2 = match T2::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t3 = match T3::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t4 = match T4::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            let t5 = match T5::from_request(&mut req).await {
                Ok(v) => v,
                Err(e) => return e.into_response(),
            };
            self(t1, t2, t3, t4, t5).await.into_response()
        })
    }
}

// Type-erased handler for storage in router
pub(crate) type BoxedHandler = Box<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync
>;

/// Create a boxed handler from any Handler
pub(crate) fn into_boxed_handler<H, T>(handler: H) -> BoxedHandler
where
    H: Handler<T>,
    T: 'static,
{
    Box::new(move |req| {
        let handler = handler.clone();
        Box::pin(async move {
            handler.call(req).await
        })
    })
}
