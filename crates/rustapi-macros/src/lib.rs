//! Procedural macros for RustAPI
//!
//! This crate provides the attribute macros used in RustAPI:
//!
//! - `#[rustapi::main]` - Main entry point macro
//! - `#[rustapi::get("/path")]` - GET route handler
//! - `#[rustapi::post("/path")]` - POST route handler
//! - `#[rustapi::put("/path")]` - PUT route handler
//! - `#[rustapi::patch("/path")]` - PATCH route handler
//! - `#[rustapi::delete("/path")]` - DELETE route handler

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Main entry point macro for RustAPI applications
///
/// This macro wraps your async main function with the tokio runtime.
///
/// # Example
///
/// ```rust,ignore
/// use rustapi_rs::prelude::*;
///
/// #[rustapi::main]
/// async fn main() -> Result<()> {
///     RustApi::new()
///         .mount(hello)
///         .run("127.0.0.1:8080")
///         .await
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
            #block
        }
    };

    TokenStream::from(expanded)
}

/// Internal helper to generate route handler macros
fn generate_route_handler(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;
    let fn_async = &input.sig.asyncness;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;
    let fn_generics = &input.sig.generics;
    
    let path_value = path.value();
    
    // Generate a companion module with route info
    let route_fn_name = syn::Ident::new(
        &format!("{}_route", fn_name),
        fn_name.span()
    );
    
    // Pick the right route helper function based on method
    let route_helper = match method {
        "GET" => quote!(::rustapi_rs::get_route),
        "POST" => quote!(::rustapi_rs::post_route),
        "PUT" => quote!(::rustapi_rs::put_route),
        "PATCH" => quote!(::rustapi_rs::patch_route),
        "DELETE" => quote!(::rustapi_rs::delete_route),
        _ => quote!(::rustapi_rs::get_route),
    };

    // Extract metadata from attributes to chain builder methods
    let mut chained_calls = quote!();
    
    for attr in fn_attrs {
        // Check for tag, summary, description
        // Use loose matching on the last segment to handle crate renaming or fully qualified paths
        if let Some(ident) = attr.path().segments.last().map(|s| &s.ident) {
            let ident_str = ident.to_string();
            if ident_str == "tag" {
                if let Ok(lit) = attr.parse_args::<LitStr>() {
                    let val = lit.value();
                    chained_calls = quote! { #chained_calls .tag(#val) };
                }
            } else if ident_str == "summary" {
                if let Ok(lit) = attr.parse_args::<LitStr>() {
                    let val = lit.value();
                    chained_calls = quote! { #chained_calls .summary(#val) };
                }
            } else if ident_str == "description" {
                if let Ok(lit) = attr.parse_args::<LitStr>() {
                    let val = lit.value();
                    chained_calls = quote! { #chained_calls .description(#val) };
                }
            }
        }
    }

    let expanded = quote! {
        // The original handler function
        #(#fn_attrs)*
        #fn_vis #fn_async fn #fn_name #fn_generics (#fn_inputs) #fn_output #fn_block
        
        // Route info function - creates a Route for this handler
        #[doc(hidden)]
        #fn_vis fn #route_fn_name() -> ::rustapi_rs::Route {
            #route_helper(#path_value, #fn_name)
                #chained_calls
        }
    };

    TokenStream::from(expanded)
}

/// GET route handler macro
///
/// # Example
///
/// ```rust,ignore
/// #[rustapi::get("/users")]
/// async fn list_users() -> Json<Vec<User>> {
///     Json(vec![])
/// }
///
/// #[rustapi::get("/users/{id}")]
/// async fn get_user(Path(id): Path<i64>) -> Result<User> {
///     Ok(User { id, name: "John".into() })
/// }
/// ```
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_handler("GET", attr, item)
}

/// POST route handler macro
#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_handler("POST", attr, item)
}

/// PUT route handler macro
#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_handler("PUT", attr, item)
}

/// PATCH route handler macro
#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_handler("PATCH", attr, item)
}

/// DELETE route handler macro
#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_handler("DELETE", attr, item)
}

// ============================================
// Route Metadata Macros
// ============================================

/// Tag macro for grouping endpoints in OpenAPI documentation
///
/// # Example
///
/// ```rust,ignore
/// #[rustapi::get("/users")]
/// #[rustapi::tag("Users")]
/// async fn list_users() -> Json<Vec<User>> {
///     Json(vec![])
/// }
/// ```
#[proc_macro_attribute]
pub fn tag(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tag = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let tag_value = tag.value();
    
    // Add a doc comment with the tag info for documentation
    let expanded = quote! {
        #[doc = concat!("**Tag:** ", #tag_value)]
        #(#attrs)*
        #vis #sig #block
    };
    
    TokenStream::from(expanded)
}

/// Summary macro for endpoint summary in OpenAPI documentation
///
/// # Example
///
/// ```rust,ignore
/// #[rustapi::get("/users")]
/// #[rustapi::summary("List all users")]
/// async fn list_users() -> Json<Vec<User>> {
///     Json(vec![])
/// }
/// ```
#[proc_macro_attribute]
pub fn summary(attr: TokenStream, item: TokenStream) -> TokenStream {
    let summary = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let summary_value = summary.value();
    
    // Add a doc comment with the summary
    let expanded = quote! {
        #[doc = #summary_value]
        #(#attrs)*
        #vis #sig #block
    };
    
    TokenStream::from(expanded)
}

/// Description macro for detailed endpoint description in OpenAPI documentation
///
/// # Example
///
/// ```rust,ignore
/// #[rustapi::get("/users")]
/// #[rustapi::description("Returns a list of all users in the system. Supports pagination.")]
/// async fn list_users() -> Json<Vec<User>> {
///     Json(vec![])
/// }
/// ```
#[proc_macro_attribute]
pub fn description(attr: TokenStream, item: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let desc_value = desc.value();
    
    // Add a doc comment with the description
    let expanded = quote! {
        #[doc = ""]
        #[doc = #desc_value]
        #(#attrs)*
        #vis #sig #block
    };
    
    TokenStream::from(expanded)
}

