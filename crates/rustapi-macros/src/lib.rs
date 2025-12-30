//! Procedural macros for RustAPI
//!
//! This crate provides the attribute macros used in RustAPI:
//!
//! - `#[rustapi::main]` - Main entry point macro
//! - `#[rustapi::get]`, `#[rustapi::post]`, etc. - Route handler macros (future)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Main entry point macro for RustAPI applications
///
/// This macro wraps your async main function with the tokio runtime.
/// It's a convenience wrapper that eliminates the need to manually
/// add `#[tokio::main]`.
///
/// # Example
///
/// ```rust,ignore
/// use rustapi_rs::prelude::*;
///
/// #[rustapi::main]
/// async fn main() -> Result<()> {
///     RustApi::new()
///         .route("/", get(hello))
///         .run("127.0.0.1:8080")
///         .await
/// }
/// ```
///
/// This expands to:
///
/// ```rust,ignore
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // your code
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    let expanded = quote! {
        #(#attrs)*
        #[::tokio::main]
        #vis #sig {
            // Initialize RustAPI
            #block
        }
    };

    TokenStream::from(expanded)
}

// Future: Route macros
// These will be implemented in Faz 2 to provide cleaner syntax like:
//
// #[rustapi::get("/users/{id}")]
// async fn get_user(id: i64) -> Result<User> { ... }
//
// For MVP, we use the functional approach: .route("/users/{id}", get(get_user))
