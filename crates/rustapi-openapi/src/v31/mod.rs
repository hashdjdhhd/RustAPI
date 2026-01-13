//! OpenAPI 3.1 specification support
//!
//! This module provides OpenAPI 3.1.0 specification generation with full
//! JSON Schema 2020-12 support and webhook definitions.
//!
//! # Key differences from OpenAPI 3.0
//!
//! - Full JSON Schema 2020-12 compatibility
//! - Webhooks support at the root level
//! - Nullable types use type arrays instead of `nullable: true`
//! - Support for `$ref` with sibling keywords
//!
//! # Example
//!
//! ```rust,ignore
//! use rustapi_openapi::v31::{OpenApi31Spec, Webhook};
//!
//! let spec = OpenApi31Spec::new("My API", "1.0.0")
//!     .description("An example API")
//!     .webhook("orderPlaced", Webhook::with_summary("Order placed event"))
//!     .build();
//! ```

mod schema;
mod spec;
mod webhooks;

#[cfg(test)]
mod tests;

pub use schema::{
    AdditionalProperties, Discriminator, ExternalDocumentation, JsonSchema2020, SchemaTransformer,
    TypeArray, Xml,
};
pub use spec::{
    ApiInfo31, Components31, Contact, ExternalDocs, License, OAuthFlow, OAuthFlows, OpenApi31Spec,
    SecurityScheme, Server, ServerVariable, Tag,
};
pub use webhooks::{
    Callback, Example, Header, MediaTypeObject, Webhook, WebhookOperation, WebhookRequestBody,
    WebhookResponse,
};
