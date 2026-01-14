//! Webhook definitions for OpenAPI 3.1
//!
//! OpenAPI 3.1 adds support for webhooks at the root level of the specification.
//! Webhooks define callback URLs that your API can call when events occur.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::schema::JsonSchema2020;

/// Webhook definition for OpenAPI 3.1
///
/// A webhook describes an HTTP callback that your API will call when
/// a specific event occurs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Webhook {
    /// Summary of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Detailed description of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// HTTP methods for the webhook (typically POST)
    #[serde(flatten)]
    pub operations: HashMap<String, WebhookOperation>,
}

impl Webhook {
    /// Create a new webhook
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a webhook with a summary
    pub fn with_summary(summary: impl Into<String>) -> Self {
        Self {
            summary: Some(summary.into()),
            ..Default::default()
        }
    }

    /// Set the summary
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a POST operation
    pub fn post(mut self, operation: WebhookOperation) -> Self {
        self.operations.insert("post".to_string(), operation);
        self
    }

    /// Add a GET operation
    pub fn get(mut self, operation: WebhookOperation) -> Self {
        self.operations.insert("get".to_string(), operation);
        self
    }

    /// Add a PUT operation
    pub fn put(mut self, operation: WebhookOperation) -> Self {
        self.operations.insert("put".to_string(), operation);
        self
    }

    /// Add a DELETE operation
    pub fn delete(mut self, operation: WebhookOperation) -> Self {
        self.operations.insert("delete".to_string(), operation);
        self
    }

    /// Add an operation with a specific HTTP method
    pub fn operation(mut self, method: impl Into<String>, op: WebhookOperation) -> Self {
        self.operations.insert(method.into().to_lowercase(), op);
        self
    }
}

/// Webhook operation (similar to path operation)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebhookOperation {
    /// Tags for API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Brief summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Detailed description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,

    /// Unique operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    /// Request body schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<WebhookRequestBody>,

    /// Expected responses from the webhook consumer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<HashMap<String, WebhookResponse>>,

    /// Security requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,

    /// Whether this operation is deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

impl WebhookOperation {
    /// Create a new webhook operation
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the summary
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the operation ID
    pub fn operation_id(mut self, id: impl Into<String>) -> Self {
        self.operation_id = Some(id.into());
        self
    }

    /// Add tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Set the request body
    pub fn request_body(mut self, body: WebhookRequestBody) -> Self {
        self.request_body = Some(body);
        self
    }

    /// Add a response
    pub fn response(mut self, status: impl Into<String>, response: WebhookResponse) -> Self {
        let responses = self.responses.get_or_insert_with(HashMap::new);
        responses.insert(status.into(), response);
        self
    }

    /// Mark as deprecated
    pub fn deprecated(mut self) -> Self {
        self.deprecated = Some(true);
        self
    }
}

/// External documentation link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocs {
    /// URL to external documentation
    pub url: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ExternalDocs {
    /// Create new external documentation
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Request body for webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRequestBody {
    /// Description of the request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the body is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Content by media type
    pub content: HashMap<String, MediaTypeObject>,
}

impl WebhookRequestBody {
    /// Create a new request body with JSON content
    pub fn json(schema: JsonSchema2020) -> Self {
        let mut content = HashMap::new();
        content.insert(
            "application/json".to_string(),
            MediaTypeObject {
                schema: Some(schema),
                example: None,
                examples: None,
            },
        );
        Self {
            description: None,
            required: Some(true),
            content,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set required
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }
}

/// Media type object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTypeObject {
    /// Schema for the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonSchema2020>,

    /// Example value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Named examples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<HashMap<String, Example>>,
}

/// Example object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    /// Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Example value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,

    /// External example URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

/// Webhook response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    /// Description of the response
    pub description: String,

    /// Response content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, MediaTypeObject>>,

    /// Response headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Header>>,
}

impl WebhookResponse {
    /// Create a new response with description
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            content: None,
            headers: None,
        }
    }

    /// Add JSON content
    pub fn with_json(mut self, schema: JsonSchema2020) -> Self {
        let content = self.content.get_or_insert_with(HashMap::new);
        content.insert(
            "application/json".to_string(),
            MediaTypeObject {
                schema: Some(schema),
                example: None,
                examples: None,
            },
        );
        self
    }
}

/// Response header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonSchema2020>,

    /// Deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

/// Callback definition
///
/// A callback is a set of webhook URLs that may be called based on an operation.
/// Each callback can contain multiple expressions (URL templates) and operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Callback {
    /// URL expressions mapped to path items
    #[serde(flatten)]
    pub expressions: HashMap<String, Webhook>,
}

impl Callback {
    /// Create a new callback
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an expression with its webhook definition
    ///
    /// The expression is a runtime expression that will be evaluated against
    /// the parent operation's data.
    pub fn expression(mut self, expr: impl Into<String>, webhook: Webhook) -> Self {
        self.expressions.insert(expr.into(), webhook);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_creation() {
        let webhook = Webhook::with_summary("Order placed notification")
            .description("Called when a new order is placed")
            .post(
                WebhookOperation::new()
                    .summary("Notify about new order")
                    .operation_id("orderPlaced")
                    .request_body(WebhookRequestBody::json(
                        JsonSchema2020::object()
                            .with_property("orderId", JsonSchema2020::string())
                            .with_property("amount", JsonSchema2020::number())
                            .with_required("orderId"),
                    ))
                    .response(
                        "200",
                        WebhookResponse::new("Webhook processed successfully"),
                    ),
            );

        assert_eq!(
            webhook.summary,
            Some("Order placed notification".to_string())
        );
        assert!(webhook.operations.contains_key("post"));
    }

    #[test]
    fn test_webhook_serialization() {
        let webhook = Webhook::new().summary("Test webhook").post(
            WebhookOperation::new()
                .operation_id("test")
                .response("200", WebhookResponse::new("OK")),
        );

        let json = serde_json::to_value(&webhook).unwrap();
        assert!(json.get("summary").is_some());
        assert!(json.get("post").is_some());
    }

    #[test]
    fn test_callback_creation() {
        let callback = Callback::new().expression(
            "{$request.body#/callbackUrl}",
            Webhook::new().post(
                WebhookOperation::new()
                    .summary("Callback notification")
                    .response("200", WebhookResponse::new("Callback received")),
            ),
        );

        assert!(callback
            .expressions
            .contains_key("{$request.body#/callbackUrl}"));
    }
}
